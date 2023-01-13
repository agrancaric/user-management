FROM rust:1.66 as builder

WORKDIR /usr/local/bin

RUN USER=root cargo new user_management

COPY Cargo.toml Cargo.lock /usr/local/bin/user_management/

WORKDIR /usr/local/bin/user_management

RUN cargo build --release

COPY . /usr/local/bin/user_management

WORKDIR /usr/local/bin/user_management

RUN cargo build --release

FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

ENV SERVER_ADDRESS 0.0.0.0
ENV SERVER_PORT 8081

EXPOSE 8081

COPY --from=builder /usr/local/bin/user_management/target/release/user_management /usr/local/bin

CMD ["/usr/local/bin/user_management"]
