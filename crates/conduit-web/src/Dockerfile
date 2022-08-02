# stage one - copy over our build files for compilation
FROM rust:1.61.0-slim-bullseye AS build

WORKDIR /app

# copy over our workspace crates to build the web project
COPY ./crates/conduit-web ./crates/conduit-web
COPY ./crates/conduit-domain ./crates/conduit-domain

# on rebuilds, we explicitly cache our rust build dependencies to speed things up and prepare a release build using trunk
RUN --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/usr/local/rustup \
    set -eux \
    rustup install stable \
    cargo install --locked trunk \
    trunk build --release

# stage two - we use an nginx container to serve the static web files as we don't require much else - just copy over our dist folder and serve!
FROM nginx:latest AS deploy

EXPOSE 80

COPY --from=build /app/crates/conduit-web/dist /usr/share/nginx/html
