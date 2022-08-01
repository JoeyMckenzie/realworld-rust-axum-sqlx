# stage one - copy over our build files for compilation, including workspace and .env files
FROM rust:1.61.0-slim-bullseye AS build

WORKDIR /app

# all the other projects are required as we're within a workspace conduit
COPY ./crates/conduit-web ./crates/conduit-web
COPY ./crates/conduit-domain ./crates/conduit-domain

# install trunk via wget
# RUN apt-get update
# RUN apt install --yes --no-install-recommends wget
# RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

# on rebuilds, we explicitly cache our rust build dependencies to speed things up
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux \
    rustup install stable \
    cargo install --locked trunk \
    trunk build --release

# stage two - we'll utilize a second container to run our built binary from our first container - slim containers!
FROM nginx:latest AS deploy

EXPOSE 80

COPY --from=build /app/crates/conduit-web/dist /usr/share/nginx/html
