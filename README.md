![realworld_logo](/realworld-dual-mode.png)

An implementation of the RealWorld project using Rust
alongside [axum](https://github.com/tokio-rs/axum) and [sqlx](https://github.com/launchbadge/sqlx).

To get started, install [Docker](https://www.docker.com/) and [make](https://www.gnu.org/software/make/)
on your local machine (Windows users may want to use WSL for ease of development), then clone/fork the repository. Once
you have the project
local to your machine, create an `.env` file local to workspace with valid values (you can use the defaults as well).

```bash
mv .env.example .env
make docker
```

Once the application containers have started, verify all integration tests pass:

```
make integration
```

The above target will run the included Postman suite of tests designed by the authors of the RealWorld project.
Once the tests have completed, verify all unit tests are passing as well:

```
cargo test # or make test
```

Again, the target above will run all included unit tests found in the project.

## Architecture

This project is more or less a Rust playground for myself and others to learn Rust, axum, and sqlx.
We utilize [cargo workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) to help encapsulate
project-specific logic/domains, with a rough organization strategy as follows:

- `conduit-api` - web project housing axum specific setup, endpoints, routing, request/response marshalling, etc.
- `conduit-bin` - the application entry point, consisting of a single `main.rs` file to drive startup and wire library
  dependencies together
- `conduit-core` - core logic and contract definitions between domains, services, and repository
- `conduit-domain` - a simple project to house PORS (plain old rust structs) used for API request and responses,
  services, etc.
- `conduit-infrastructure` - a project adapter containing implementations of the core business logics definitions from
  higher up

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

## Using Docker

The project utilizes Docker containers for Postgres and prometheus metrics. For example, when starting the
application with `make docker`, navigating to `localhost:9090` will bring you to the prometheus metrics page.
From there, running integration tests with `make integration` to simulate traffic to the API will allow you the various
metrics
that are recorded in the service layer: request count, request latency, and histograms of service request intervals.

To start the API outside the Docker context, ensure that Postgres is running before booting up:

```bash
make start-db
```

Once the Postgres container has started, go ahead and spin up the API for active development:

```bash
make dev # or cargo run
```

If you're starting the application for the first time, it will attempt to seed a bit of data that is also used for
testing.

# TODO

There's a lot more unit tests to write...