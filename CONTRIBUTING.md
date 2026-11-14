# Contributing

## Development

Use stable Rust with the `rustfmt` and `clippy` components, as selected by the existing pinned reusable CI. Record `rustc --version` and `cargo --version`; do not silently change the project's edition or toolchain while making a documentation change.

```sh
rustup component add rustfmt clippy
```

Follow rustfmt output and keep Clippy warnings actionable. Add tests for changed invariants. This is a passive library: applications own global tracing/logging subscriber setup.

See [BOOTSTRAP.md](BOOTSTRAP.md) when creating a new project from this starter.

## Validation

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --all-features
```

Run all three commands from the repository root. CI also uses `cargo-llvm-cov` 0.8.7 to produce `lcov.info` and uploads it through Codecov OIDC. A local test run is not evidence that the hosted coverage upload succeeded.

Record the command, tool versions and outcome in the pull request. Distinguish
passed, failed, skipped and unavailable checks; include a hosted run link when
available. Required automated checks must pass before integration.

## Pull requests

Use a focused branch such as `fix/short-summary`, `feat/short-summary` or
`docs/short-summary`. Explain the problem and resulting behaviour, link related
issues, and include the validation evidence. Keep commits reviewable and update
affected documentation, examples and tests with the change. Identify any
compatibility, generated-output, release or data impact.

The repository has one maintainer. Automated review and required checks replace
mandatory human approval; do not add a second-reviewer, team or CODEOWNERS gate.
Preserve unrelated work and existing security/publication requirements.

## Security and support

Use the repository's issue chooser for reproducible bugs or scoped proposals.
For usage questions, include a minimal safe example and what you have already
tried. Follow [SECURITY.md](SECURITY.md) for vulnerabilities; do not put private
data, credentials or sensitive exploit details in public issues or logs.

Follow the account's
[code of conduct](https://github.com/edithatogo/.github/blob/main/CODE_OF_CONDUCT.md).
When adopting this template under a different owner, replace this link with the
new project's policy and verify its support and private-reporting routes.
