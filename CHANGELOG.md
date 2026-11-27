# Changelog

All notable changes to `muda` are documented in this file.

The format follows Keep a Changelog, and this project adheres to Semantic Versioning.

## [Unreleased]

### Added

- Least-privilege GitHub Actions permissions (`contents: read`) and non-persisted checkout credentials in `.github/workflows/ci.yml`.
- Tag-triggered multi-platform release workflow (`.github/workflows/release.yml`) with `SHA256SUMS` generation and POSIX `install.sh` verifier installing `muda` directly from GitHub Releases.
- Sample rework rate card (`examples/rates.json`) and Draft 2020-12 JSON Schemas (`schema/rates.schema.json`, `schema/statement.schema.json`).
- Comprehensive operator and library guide (`docs/GUIDE.md`) covering GitHub Actions `fetch-depth: 0` checkout configuration, UTC month boundaries, exact trailer/subject grammars, `unjoined` reason strings, and minor-unit pricing, alongside `SECURITY.md` and `CONTRIBUTING.md`.

## [0.1.0] - 2026-01-01

### Added

- Zero-dependency Rust CLI (`muda statement`) and library auditing Git commit history for monthly `revert`, `fix-forward`, and `redone` rework.
- Cause attribution (`causes`), transparent `unjoined` reporting, repository-interior `-o` output guard, and exact arbitrary-precision minor-unit rate-card pricing (`--rates`).
