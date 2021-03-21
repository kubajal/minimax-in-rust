FROM rust:1.50

COPY . /rust-app
RUN cargo install cargo-watch
WORKDIR /rust-app
ENTRYPOINT cargo watch -s 'cargo run --example server'