build:
    cargo build

docker-build:
    docker build -t technosophos/echo-actor:latest .
    docker create -ti --name echoactor-build technosophos/echo-actor:latest /bin/sh
    docker cp echoactor-build:/usr/echo-provider/target/debug/libecho_provider.so ./target/debug
    docker rm -f echoactor-build