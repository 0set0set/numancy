# Contributing to numancy

Thanks for your interest in contributing! `numancy` is a small numerology library
and CLI, and contributions are welcome.

## Layout

This is a Cargo workspace with two crates:

- `crates/numancy` — the library (pure, dependency-free; published to crates.io).
- `crates/numancy-cli` — a CLI on top of the library (`clap`, `time`); not published.

The shared version and lints live in `[workspace.package]` / `[workspace.lints]`
in the root `Cargo.toml`; the MSRV is the `rust-version` field there.

## Development setup

You only need a Rust toolchain (stable is fine). Build, test and run the CLI:

```bash
cargo build
cargo test
cargo run -p numancy-cli -- "Barbara Liskov" 1939-11-07 --system cabalistic --number motivation
```

## Checks before opening a PR

CI runs these on every pull request; run them locally first:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo deny check advisories bans sources   # optional, requires cargo-deny
```

Formatting is configured by `rustfmt.toml` using only stable options, so
`cargo fmt` works on the stable toolchain (no nightly required).

## Commit messages

Releases are fully automated with [semantic-release](https://semantic-release.gitbook.io/).
The next version, the `CHANGELOG.md`, the git tag, the GitHub release and the
`cargo publish` to crates.io are all derived from your commit messages, so
please use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` — minor release
- `fix:`, `perf:`, `refactor:`, `chore:` — patch release
- `docs:`, `ci:`, `test:` — no release
- a `!` (e.g. `feat!:`) or a `BREAKING CHANGE:` footer — major release

Do not edit `CHANGELOG.md` or create tags by hand: semantic-release owns them.

## Architecture

The crate is layered, with dependencies pointing in one direction only:

```
base       alphabet, name, reduction        letter tables and number reduction
domain  -> base                             pythagorean, cabalistic, date, address,
                                            cycles, pyramid, signature, compatibility
chart   -> domain, base                     high-level PythagoreanChart / CabalisticMap
```

The two numerology systems (Pythagorean and Brazilian cabalistic) are kept
strictly separate because they use different letter tables and accent rules.
Typed errors live in `crates/numancy/src/error.rs`. Unsafe code is forbidden
across the workspace (`unsafe_code = "forbid"` in `[workspace.lints.rust]`).

## Public API and docs

Public items must be documented (`#![warn(missing_docs)]` is enabled, and CI
denies warnings). If you add a public item, document it and, where useful, add a
runnable doctest.

## Examples must use public data only

Tests and documentation use only public figures (pioneering women in computing)
and their publicly known birth dates. Please do not add anyone's private data.
