FROM rust:latest as build

WORKDIR /app

# for stage one, copy over our build files for compilation, including workspace and .env files
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./.env.docker ./.env
COPY ./conduit-api ./conduit-api
COPY ./conduit-bin ./conduit-bin
COPY ./conduit-core ./conduit-core
COPY ./conduit-domain ./conduit-domain
COPY ./conduit-infrastructure ./conduit-infrastructure

RUN cargo build --workspace --release

# for stage two, we'll utilize a second container to run our built binary from our first container - ultra slim containers!
FROM rust:latest as deploy

WORKDIR /deploy
COPY --from=build /app/target/release/conduit-bin ./
CMD ["./conduit-bin"]