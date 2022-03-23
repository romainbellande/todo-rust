FROM rust:1-buster

RUN mkdir /.cargo

RUN chown -R ${UID}:${GID} /.cargo && \
    mkdir -p /usr/src/app && \
    chown -R ${UID}:${GID} /usr/src/app

RUN cargo install trunk \
                  wasm-bindgen-cli

RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

EXPOSE 3001
