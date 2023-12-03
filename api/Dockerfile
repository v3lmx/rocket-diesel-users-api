FROM rust:1.73.0

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 8000

VOLUME ["/usr/local/cargo"]