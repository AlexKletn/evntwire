# Contributing to evntwire

[Русская версия](CONTRIBUTING.ru.md)

Follow the [code quality agreement](CODE_QUALITY.md) and use English [Conventional Commits](COMMIT_CONVENTIONS.md). Keep shared event routing in evntwire and device-specific profiles in the project that uses them.

Run these checks from the workspace root before submitting a pull request.

```sh
cargo fmt --all --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Describe what changed and how you verified it. For changes that affect physical inputs or `/dev/uinput`, include the Linux environment and hardware used for verification when available.
