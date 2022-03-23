FROM rust:1-buster

RUN mkdir /.cargo

RUN chown -R ${UID}:${GID} /.cargo && \
    mkdir -p /usr/src/app && \
    chown -R ${UID}:${GID} /usr/src/app

RUN cargo install sea-orm-cli \
                  cargo-watch

WORKDIR /usr/src/app

EXPOSE 3001
