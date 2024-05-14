# Rust / Actix / JWT

A Clean Architecture for a Rest API in rust with JWT.

# Installing

```bash
cargo build
```

# Running

define the environment on which we're running by adding `ENV=<env>`, which will use the `.env.<env>` file

```bash
ENV=dev cargo run
```

# Watch

define the environment on which we're running by adding `ENV=<env>`, which will use the `.env.<env>` file

```bash
ENV=dev cargo watch -x run
```

# Code quality & security

Used in CI/CD

```bash
cargo fmt --all -- --check
cargo clippy --all-targets
cargo audit
cargo outdated
```
