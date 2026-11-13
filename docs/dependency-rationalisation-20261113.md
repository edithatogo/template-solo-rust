# Rust dependency rationalisation — template-solo-rust (2026-11-13)

**Status:** proposal and review contract; no code adoption, runtime verification or benchmark result is claimed here.

## Repository-specific decisions

- **Verified:** the template is intentionally dependency-free with a placeholder Cargo package; it should document options rather than force all 22 crates into new projects.
- **Provide architecture profiles:** `serde` for versioned data contracts; `thiserror` for typed libraries and `anyhow` at binary boundaries; `clap` for nontrivial CLIs; `tracing` for structured diagnostics.
- **Opt-in use cases:** Tokio/Reqwest/SQLx for network/database; Rayon/Crossbeam for measured concurrency; Polars for dataframe workloads; Ratatui for terminal UIs; PyO3 for Python adapters; Bytes for binary I/O; Syn/Quote only for macros.
- **Assurance profiles:** Proptest for invariants, Criterion for performance hot paths, Insta for reviewed compact contract snapshots. Avoid all three by default if no relevant behavior exists.
- **Template gate:** after creating a real consumer project, lock compatible MSRV and crate features, run formatting/clippy/tests/audit and record upstream licenses; no universal dependency lock-in.

## Evidence and release gate
1. Discover actual source, package boundary, MSRV, lockfile, existing tests and Conductor ownership; avoid speculative dependencies.
2. For runtime behaviour, write failing contract tests before implementation, run targeted and full suites; preserve stable schemas, errors, CLI semantics and deterministic outputs.
3. Record `cargo tree -d`, feature selections, security/licensing/advisory reviews, transitive dependency cost and rollback before accepting an adoption.
4. For performance changes, benchmark identical inputs and correctness/invariance oracles; do not claim improvements without measurements.
5. This document is a planning entry, NOT proof of compilation, hosted CI or scientific validation; update status only after a revision-bound receipt.
