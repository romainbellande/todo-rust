FROM rust:1-buster

RUN mkdir /.cargo

RUN chmod -R 777 /.cargo

RUN cargo install sea-orm-cli

RUN cargo install cargo-watch \
                  trunk \
                  wasm-bindgen-cli

RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

EXPOSE 3001