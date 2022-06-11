FROM rust:latest

WORKDIR /app
COPY Cargo.lock ./
COPY Cargo.toml ./
COPY .env ./
COPY ./conduit-api ./conduit-api
COPY ./conduit-bin ./conduit-bin
COPY ./conduit-core ./conduit-core
COPY ./conduit-domain ./conduit-domain
COPY ./conduit-infrastructure ./conduit-infrastructure

RUN cargo build --workspace --release

CMD ["./target/release/conduit-bin"]