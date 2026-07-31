# Solo-maintainer Rust project template

[![CI](../../actions/workflows/ci.yml/badge.svg)](../../actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE) [![Citation](https://img.shields.io/badge/citation-CFF-blue.svg)](CITATION.cff)

A Rust 2024 project baseline with reusable CI, real coverage, Renovate, and an explicit manifest version.

## Status

This repository is designed for one maintainer. Automated checks are required;
no second reviewer, CODEOWNERS approval, team membership, or mandatory human
approval is introduced.

## Start here

1. Replace `replace-me` in `Cargo.toml`.
2. Run `cargo fmt --check`.
3. Run `cargo test --all-features`.

## Development

CI checks formatting, lints with warnings denied, runs tests, and uploads real coverage through Codecov OIDC.

## Versioning

`Cargo.toml` is authoritative. Release automation must update it and verify that the release tag matches; runtime code must not contain another version.

## Logging

This starts as a passive library. It may emit `tracing` events when warranted, but the calling application owns subscriber configuration.

## Security

Report vulnerabilities privately through GitHub Security Advisories. See
[SECURITY.md](SECURITY.md); do not disclose credentials or sensitive source data
in a public issue.

## Citation

See [CITATION.cff](CITATION.cff). Release-specific versions and identifiers are
added only when the release exists.

## License

Repository-authored starter material is MIT licensed; see [LICENSE](LICENSE).
Record third-party and source-data rights separately.