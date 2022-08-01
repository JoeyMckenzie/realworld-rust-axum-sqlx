[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) [![CI](https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx/actions/workflows/ci.yml/badge.svg)](https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx/actions/workflows/ci.yml) [![Coverage](https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx/actions/workflows/coverage.yml/badge.svg)](https://github.com/JoeyMckenzie/realworld-rust-axum-sqlx/actions/workflows/coverage.yml) [![Codecov](https://codecov.io/gh/JoeyMckenzie/realworld-rust-axum-sqlx/branch/main/graph/badge.svg?token=B8Z26ZBQ5N)](https://codecov.io/gh/JoeyMckenzie/realworld-rust-axum-sqlx)

![realworld_logo](/realworld-dual-mode.png)

A fullstack implementation of the RealWorld project using [Rust](https://www.rust-lang.org/), [axum](https://github.com/tokio-rs/axum), [sqlx](https://github.com/launchbadge/sqlx), and [yew](https://yew.rs).

To get started, install [Docker](https://www.docker.com/) and [cargo-make](https://github.com/sagiegurari/cargo-make)
on your local machine (Windows users may want to use WSL for ease of development), then clone/fork the repository. Once
you have the project
local to your machine, create an `.env` file local to workspace with valid values (you can use the defaults as well).

```bash
cp .env.example .env
```

Next, create and start our Docker containers:

```bash
cargo make docker
```

Once the application containers have started, verify all integration tests pass:

```bash
cargo make integration
```

The above target will run the included Postman suite of tests designed by the authors of the RealWorld project.
Once the tests have completed, verify all unit tests are passing as well:

```bash
cargo make test
```

Again, the target above will run all included unit tests found in the project. To run the frontend project
using [trunk](https://trunkrs.dev/):

```bash
cargo make web
```

## Project Structure

This project is more or less a Rust playground for myself and others to learn Rust, axum, and sqlx.
We utilize [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) to help encapsulate
project-specific logic/domains, with a rough organization strategy as follows:

- `crates/conduit-bin` - API entry point, consisting of a single `main.rs` file to drive startup and wire library
  dependencies together
- `crates/conduit-web` - (_in progress_) web frontend project utilizing [yew](https://yew.rs/)
- `crates/conduit-api` - web API project housing axum specific setup, endpoints, routing, request/response marshalling,
  etc.
- `crates/conduit-core` - core logic and contract definitions between domains, services, and repository
- `crates/conduit-domain` - a simple project to house PORS (plain old rust structs) used for API request and responses,
  services, etc.
- `crates/conduit-infrastructure` - a project adapter containing implementations of the core business logics definitions
  from
  higher up
- `integrations` - contains the RealWorld Postman test scripts and collection
- `cypress` - contains the e2e tests used to test the frontend project (`crates/conduit-web`)
- `deploy` - contains all Docker Compose files used for building the project containers
- `.husky` - contains husky git hooks

#### Why is there `package.json` file?

I use [husky](https://github.com/typicode/husky) for pre-commit hooks
and [lint-staged](https://www.npmjs.com/package/lint-staged)
to format staged files to keep committed code well formatted. While there are a few other options for including
pre-commit hooks
into a Rust project, and certainly those that are more appropriate for Rust projects, I wanted to leave open the
opportunity
of bringing on a TS-based frontend sometime in the future to have the true RealWorld fullstack experience. The
pre-commit hooks will format, lint, and test all code so that each commit ensure that tests are passing and code does
not contain any obvious errors.

To bypass the included hooks, simply pass a `--no-verify` flag while committing code

```bash
git commit -m "feat(core): add some amazing unit tests" --no-verify
```

## Using Docker

The project utilizes Docker containers for Postgres and Prometheus metrics, as well as containers for the API and frontend. For example, when starting the
application with `cargo make docker`, navigating to `localhost:9090` will bring you to the Prometheus metrics page.
From there, running integration tests with `cargo make integration` to simulate traffic to the API allows one to observe
the
various
metrics that are recorded in the service layer: request count, request latency, and histograms of service request
intervals.

To start the API outside the Docker context, run:

```bash
cargo make dev # or cargo run
```

The `dev` tasks takes on the `postgres` task as a dependency, so your database container will start automatically.

If you're starting the application for the first time, it will attempt to seed a bit of data that is also used for
testing.

## TODO

There's a lot more unit tests to write...
