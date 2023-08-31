# Development Guide

## Debugging the app

Use [libfaketime](https://github.com/wolfcw/libfaketime) To mock the time in your system.

```bash
sudo dnf install --assumeyes libfaketime
```

```bash
$ # before midnight
$ faketime '2023-8-30 20:00:00' cargo run --example salah
$ # after midnight
$ faketime '2023-8-31 02:00:00' cargo run --example salah
```

## Commit Message Format

This repo is using [Agular's commit message format][commit-message]

[commit-message]: https://github.com/angular/angular/blob/2095a08781167e91a60a4cec65c694688b319cd0/CONTRIBUTING.md#-commit-message-format
