FROM rust:stretch

WORKDIR /usr/echo-provider
COPY . .
RUN cargo build
