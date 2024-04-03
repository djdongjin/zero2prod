## Learning goals
- [ ] Basic rust usage
- [ ] CI/CD setup
- [ ] Containerization
- [ ] Cloud service integration
- [ ] Metrics, logs, traces
- [ ] Familiarize with rust toolchain: testing, async/await

## Notes
`cargo watch -x check -x test -x run` to run compilation/test/run on file changes.

To check test coverage:

```shell
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests
```

To disable a clippy warning: `#[allow(clippy::lint_name)]`

## TODOs

### Ch3

- [ ] Understand `mod`, `pub use` (e.g., in [mod.rs](./src/routes/mod.rs)).
- [ ] Understand when to use, e.g. `use crate::routes` vs `use zero2prod::startup::run`.

### Ch4

- [ ] Understand all those tracing related crates (e.g., `tracing::instrument`).
- [ ] Wire tracing with open telemetry (`tracing-opentelemetry`).
- [ ] `tracing` v.s. `log`.

### Ch5

- [ ] Use a container image for the app. E.g. [Wolfi](https://github.com/wolfi-dev/).
- [ ] Deploy to cloudflare if possible.

### Ch6

- [ ] `.into()` method
- [ ] `?` operator

### Ch8 - Error handling

- [ ] `std::fmt::Debug` and `std::fmt::Display`.
- [ ] `?` v.s. `unwrap()`.
- [ ] `async`/`await`.