# CLAUDE.md

Guidance for AI agents (and humans) working in the **template-rust-cli** repo.

## What template-rust-cli is

Rust CLI template with CI, releases and docs. A Rust CLI built with `clap` (derive) in a Cargo workspace; logic lives in a library crate so it stays unit-testable.

## Commands

Tool versions are pinned in `mise.toml` (`mise install`, then `lefthook install` runs automatically). The toolchain channel is `stable` via `rust-toolchain.toml`.

- `mise run check`: `cargo fmt --check` + `cargo clippy --workspace --all-targets --locked -- -D warnings`. **Must be clean before committing.**
- `mise run test`: `cargo test --workspace --locked`. **Must be green before committing.**
- `mise run build`: release build. `mise run fmt` formats.
- Run the CLI: `cargo run -p template-rust-cli-cli -- Allie`.

## Architecture

- `crates/template-rust-cli-core/`: library crate, all logic and the `Error` type (`thiserror`). No printing, no process exit.
- `crates/template-rust-cli-cli/`: the `template-rust-cli` binary: `clap` parsing, I/O, `anyhow` at the edge. Integration tests in `tests/` use `assert_cmd`.
- Root `Cargo.toml` owns versions (`[workspace.package]`), shared deps (`[workspace.dependencies]`) and lints (`[workspace.lints]`).

## Conventions that bite

- **Workspace lints only apply to crates that opt in** with `[lints] workspace = true`. Every new crate must add it.
- `unwrap()` warns outside tests (clippy.toml allows it in tests). Return errors; use `anyhow` only in the binary crate.
- Always pass `--locked`: CI does, and `Cargo.lock` is committed.
- Add dependencies to `[workspace.dependencies]` and reference them with `.workspace = true`.

## Things that bit us

- (none yet)

## Releasing

Bump `[workspace.package].version`, add the CHANGELOG entry, merge, then `git tag vX.Y.Z && git push origin vX.Y.Z`. `release.yml` verifies tag == Cargo version, builds 4 targets, attaches tarballs, and bumps the formula in `alliecatowo/homebrew-tap` (needs the `HOMEBREW_TAP_DEPLOY_KEY` secret, setup steps are at the top of the workflow). The formula is rendered from `packaging/homebrew/formula.rb.tmpl` each time; never patch a rendered formula.

## Changelog

`CHANGELOG.md` follows [Keep a Changelog](https://keepachangelog.com). Every user-facing change adds a bullet under `## [Unreleased]` in the same change as the code.

## Git workflow

- Branch off `main`. Conventional commits (enforced by lefthook `commit-msg`). PRs are draft by default.
- **Worktrees go in `.claude/worktrees/<branch-with-dashes>` inside this repo.** Never under `/tmp` or a scratchpad. Remove after merge.
- Don't tag or publish unless asked. Dependabot patch/minor PRs auto-merge on green.

## Docs site

`docs/` is a [Zola](https://www.getzola.org) site (single binary, no Node): `zola --root docs serve` to preview, `zola --root docs check` to verify links. Content lives in `docs/content/`, the theme in `docs/templates/`. `.github/workflows/docs.yml` builds it on every change and deploys it to GitHub Pages from `main` (needs Settings > Pages > Source: "GitHub Actions"; until then the deploy job skips itself). A dead internal link fails the build, so link repo files via github.com URLs.
