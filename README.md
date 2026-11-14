# Solo-maintainer Rust project template

[![CI](../../actions/workflows/ci.yml/badge.svg)](../../actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE) [![Citation](https://img.shields.io/badge/citation-CFF-blue.svg)](CITATION.cff)

A Rust 2024 project baseline with reusable CI, real coverage, Renovate, and an explicit manifest version.

## Status

This repository is designed for one maintainer. Automated checks are required;
no second reviewer, CODEOWNERS approval, team membership, or mandatory human
approval is introduced.

## Usage

Choose **Use this template** to create a Rust library, then follow the
[adoption checklist](BOOTSTRAP.md). The initial library exposes
`replace_me::ready()` and a smoke test. Replace that example with the project's
API and behaviour tests.

1. Replace `replace-me` in `Cargo.toml` and update crate references.
2. Run `cargo fmt --all --check`.
3. Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`.
4. Run `cargo test --all-features`.

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for setup and validation commands, and
[BOOTSTRAP.md](BOOTSTRAP.md) for the adoption checklist.

The pinned shared CI checks formatting, runs Clippy with warnings denied and
uses cargo-llvm-cov to run tests and collect coverage. It is configured to
upload LCOV through Codecov OIDC. Verify provider activation and an actual
upload in each generated repository.

## Versioning

`Cargo.toml` is authoritative. Release automation must update it and verify that the release tag matches; runtime code must not contain another version.

## Logging

This starts as a passive library. It may emit `tracing` events when warranted, but the calling application owns subscriber configuration.

## Security

Follow [SECURITY.md](SECURITY.md) for private reporting and the safe contact
fallback if GitHub's **Report a vulnerability** option is unavailable. Do not
include credentials, sensitive source data or exploit details in public issues.

## Citation

See [CITATION.cff](CITATION.cff). Release-specific versions and identifiers are
added only when the release exists.

## License

Repository-authored starter material is MIT licensed; see [LICENSE](LICENSE).
Record third-party and source-data rights separately.
