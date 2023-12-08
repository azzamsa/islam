#!/usr/bin/env -S just --justfile

alias d := dev
alias f := fmt
alias l := lint
alias t := test

# List available commands.
_default:
    just --list --unsorted

# Setup the development tools.
_setup-dev:
    just _cargo-install 'cargo-nextest git-cliff dprint cargo-edit cargo-outdated bacon typos-cli'

# Tasks to make the code-base comply with the rules. Mostly used in git hooks.
comply: _doc-check fmt lint test

# Check if the repository comply with the rules and ready to be pushed.
check: fmt-check lint test

# Develop the app.
dev:
    bacon

# Format the codebase.
fmt:
    cargo fmt --all
    dprint fmt --config configs/dprint.json

# Check is the codebase properly formatted.
fmt-check:
    cargo fmt --all -- --check
    dprint check --config configs/dprint.json

# Lint the codebase.
lint:
    cargo clippy --all-targets --tests
    typos --config configs/typos.toml

# Test the codebase.
test:
    cargo test --doc
    cargo nextest run

_test-ignored:
    faketime '2023-8-30 20:00:00' cargo nextest run --filter-expr 'test(before_midnight)' --run-ignored ignored-only

    faketime '2023-8-31 02:00:00' cargo nextest run --filter-expr 'test(after_midnight)' --run-ignored ignored-only

# Create a new release. Example `cargo-release release minor --tag-name v0.2.0`
release level:
    cargo-release release {{ level }} --execute

# Make sure the repo is ready for release
release-check level:
    just up
    cargo-release release {{ level }}

# Check the documentation.
_doc-check:
    cargo doc --all-features --no-deps

# Release hooks
_release-prepare version:
    git-cliff --config configs/cliff.toml --output CHANGELOG.md --tag {{ version }}
    just fmt

# Check dependencies health. Pass `--write` to uppgrade dependencies.
[unix]
up arg="":
    #!/usr/bin/env bash
    if [ "{{ arg }}" = "--write" ]; then
        cargo upgrade
        cargo update
    else
        cargo outdated --root-deps-only
    fi;

[windows]
up arg="":
    #!powershell.exe
    if ( "{{ arg }}" -eq "--write") {
        cargo upgrade
        cargo update
    }
    else {
        cargo outdated --root-deps-only
    }

#
# Helper
#

[unix]
_cargo-install tool:
    #!/usr/bin/env bash
    if command -v cargo-binstall >/dev/null 2>&1; then
        echo "cargo-binstall..."
        cargo binstall --no-confirm --no-symlinks {{ tool }}
    else
        echo "Building from source"
        carg
