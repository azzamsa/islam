# Release Checklist

- Run linting:

``` bash
cargo clippy --all-features -- --deny warnings --deny clippy::pedantic --deny clippy::nursery
```

- Run `cargo update` and review dependency updates.
- Update the CHANGELOG.
- Update version numbers in `Cargo.toml` and `README.md`, Run `cargo update -p islam` so that the Cargo.lock is updated.
- Create a commit with a message format: `v[0-9]+.[0-9]+.[0-9]+`, and push.
- Wait for a checks to pass, tag a commit with a release tag, then push the tag.
