#[macro_use]
extern crate wascc_codec as codec;

#[macro_use]
extern crate log;

use codec::capabilities::{CapabilityProvider, Dispatcher, NullDispatcher};
use codec::core::{OP_CONFIGURE, OP_REMOVE_ACTOR};
use wascc_codec::core::CapabilityConfiguration;
use wascc_codec::deserialize;

use std::error::Error;
use std::sync::RwLock;

capability_provider!(EchoProvider, EchoProvider::new);

const CAPABILITY_ID: &str = "wok:echoProvider";
const OP_ECHO: &str = "EchoRequest";

pub struct EchoProvider {
    dispatcher: RwLock<Box<dyn Dispatcher>>,
}

impl Default for EchoProvider {
    fn default() -> Self {
        env_logger::init();

        EchoProvider {
            dispatcher: RwLock::new(Box::new(NullDispatcher::new())),
        }
    }
}

impl EchoProvider {
    pub fn new() -> Self {
        Self::default()
    }

    fn configure(&self, _config: CapabilityConfiguration) -> Result<Vec<u8>, Box<dyn Error>> {
        // Do nothing here
        Ok(vec![])
    }

    pub fn send(&self, module_id: &str, msg: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let req = format!("{}!EchoRequest", module_id);
        self.dispatcher
            .read()
            .unwrap()
            .dispatch(&req, msg.as_bytes())
    }
}

impl CapabilityProvider for EchoProvider {
    fn capability_id(&self) -> &'static str {
        CAPABILITY_ID
    }

    // Invoked by the runtime host to give this provider plugin the ability to communicate
    // with actors
    fn configure_dispatch(&self, dispatcher: Box<dyn Dispatcher>) -> Result<(), Box<dyn Error>> {
        trace!("Dispatcher received.");
        let mut lock = self.dispatcher.write().unwrap();
        *lock = dispatcher;

        Ok(())
    }

    fn name(&self) -> &'static str {
        "EchoProvider test harness"
    }

    // Invoked by host runtime to allow an actor to make use of the capability
    // All providers MUST handle the "configure" message, even if no work will be done
    fn handle_call(&self, actor: &str, op: &str, msg: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        trace!("Received host call from {}, operation - {}", actor, op);

        match op {
            OP_CONFIGURE if actor == "system" => self.configure(deserialize(msg)?),
            // We just return a copy of the input
            OP_ECHO => Ok(msg.to_owned()),
            OP_REMOVE_ACTOR if actor == "system" => Ok(vec![]),
            _ => Err("bad dispatch".into()),
        }
    }
}
