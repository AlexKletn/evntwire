# evntwire

[English version](README.md)

[Как участвовать](CONTRIBUTING.ru.md) · [GPL-2.0-only](LICENSE)

evntwire связывает источники событий с настраиваемыми действиями в Linux.

В рабочем пространстве есть `crates/evntwire`, `crates/evntwire-config` и заглушка графического интерфейса `crates/evntwire-gui`.

Запустите `evntwire SOCKET_PATH [CONFIG_PATH]`, чтобы принимать события через Unix-сокет датаграмм. Укажите абсолютный путь к сокету в закрытом каталоге. Путь к конфигу по умолчанию — `/etc/evntwire/evntwire.toml`.

Если выбранного файла конфигурации нет, при запуске evntwire создаёт пустой файл и недостающие родительские каталоги.

На Arch Linux запустите `makepkg -s` из корня репозитория, чтобы собрать пакет.

Команда проверки рабочего пространства:

```sh
cargo test --workspace --locked
```
