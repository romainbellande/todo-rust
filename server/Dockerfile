FROM rust:1-buster

RUN mkdir /.cargo

RUN chmod -R 777 /.cargo

RUN cargo install sea-orm-cli

RUN cargo install cargo-watch

WORKDIR /usr/src/app

EXPOSE 3001
