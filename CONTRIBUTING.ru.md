# Как участвовать в разработке evntwire

[English version](CONTRIBUTING.md)

Соблюдайте [соглашение о качестве кода](CODE_QUALITY.ru.md) и пишите сообщения коммитов по [Conventional Commits](COMMIT_CONVENTIONS.ru.md) на английском. Общую маршрутизацию событий размещайте в evntwire, а профили конкретных устройств — в проектах, которые их используют.

Перед отправкой pull request запустите проверки из корня рабочего пространства.

```sh
cargo fmt --all --check
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Опишите, что изменилось и как вы это проверили. Для изменений, затрагивающих физические устройства ввода или `/dev/uinput`, укажите окружение Linux и оборудование, если проверяли работу на устройстве.
