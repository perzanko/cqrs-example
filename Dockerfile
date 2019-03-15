FROM rustlang/rust:nightly

RUN cargo install diesel_cli --no-default-features --features postgres

RUN cargo install cargo-watch

WORKDIR /usr/src/app

COPY Cargo.toml .

EXPOSE 3000

VOLUME ["/usr/local/cargo"]