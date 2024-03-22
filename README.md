# Learning goals

- [ ] Basic rust usage
- [ ] CI/CD setup
- [ ] Containerization
- [ ] Cloud service integration
- [ ] Metrics, logs, traces
- [ ] Familiarize with rust toolchain: testing, async/await

# Notes

`cargo watch -x check -x test -x run` to run compilation/test/run on file changes.

To check test coverage:

```shell
cargo install cargo-tarpaulin
cargo tarpaulin --ignore-tests
```

To disable a clippy warning: `#[allow(clippy::lint_name)]`


# TODO:

- [ ] Add CI: https://gist.github.com/LukeMathWalker/5ae1107432ce283310c3e601fac915f3