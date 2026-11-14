# AGENTS.md

## Repository map

- `Cargo.toml`: crate identity, Rust 2024 edition and authoritative package version.
- `src/lib.rs`: library starter and its tests.
- `docs/dependency-rationalisation-20261113.md`: existing dependency decisions.
- `.github/workflows/ci.yml`: pinned shared Rust CI caller.

## Development and validation

Read [CONTRIBUTING.md](CONTRIBUTING.md) for the required setup, exact local
commands and evidence requirements. Run formatting, Clippy with warnings denied and tests; preserve the separate hosted coverage gate.
`Cargo.toml` is the version authority. Preserve the dependency decision record and justify additions against the actual capability needed.

## Operating model

This repository is maintained by one developer. Automated review, tests and
security gates replace mandatory human approvals. Do not add CODEOWNERS approval
requirements, team assignments or a second-maintainer dependency. Use a focused
branch and reviewable pull request; preserve unrelated and unmerged work.

## Boundaries

Never commit credentials or sensitive data. Keep generated files reproducible.
Prefer one authoritative configuration and link to it from tool-specific files.
Preserve existing release, data, credential and privacy gates. An unavailable
check is not a pass, and a configured provider is not evidence of its execution.

Use [BOOTSTRAP.md](BOOTSTRAP.md) for adoption decisions. Keep the applicability
declarations in `.github/community-bootstrap.json` aligned with actual project
configuration; they do not substitute for observed GitHub settings.
