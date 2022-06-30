# all credit goes to https://fasterthanli.me/articles/remote-development-with-rust-on-fly-io#what-the-heck-is-fly-io-for-even
# for an an awesome walkthrough of docker files for rust, this is more or less a direct copy pasta with a few minor tweaks

# after containerization, this manages to come in at a whopping ~163mb, still a bit to we could optimize but this should do for now

# stage one - copy over our build files for compilation, including workspace and .env files
FROM rust:1.61.0-slim-bullseye AS build

WORKDIR /app

COPY ./rust-toolchain ./
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./.env.docker ./.env
COPY ./crates/conduit-bin ./crates/conduit-bin
COPY ./crates/conduit-web ./crates/conduit-web
COPY ./crates/conduit-api ./crates/conduit-api
COPY ./crates/conduit-core ./crates/conduit-core
COPY ./crates/conduit-domain ./crates/conduit-domain
COPY ./crates/conduit-infrastructure ./crates/conduit-infrastructure

# on rebuilds, we explicitly cache our rust build dependencies to speed things up
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux; \
    rustup install stable; \
    cargo build --workspace --release; \
    objcopy --compress-debug-sections target/release/conduit-bin ./conduit

# stage two - we'll utilize a second container to run our built binary from our first container - slim containers!
FROM debian:11.3-slim as deploy

RUN set -eux; \
    export DEBIAN_FRONTEND=noninteractive; \
    apt update; \
    apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
    apt clean autoclean; \
    apt autoremove --yes; \
    rm -rf /var/lib/{apt,dpkg,cache,log}/;

WORKDIR /deploy

COPY --from=build /app/conduit ./

CMD ["./conduit"]