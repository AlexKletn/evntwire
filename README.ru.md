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

## Устройства ввода Linux

Evntwire подключает устройства evdev по точному имени, в том числе после
старта. Чтение не захватывает устройство эксклюзивно. Для кнопки можно
указать `source`, по умолчанию используется имя устройства. Для
переключателя демон читает начальное состояние при подключении, обновляет
его по событиям и сбрасывает при отключении устройства.

```toml
[[physical_button]]
device_name = "My gesture device"
source = "gestures"
key = "KEY_F24"
button = "tilt"

[[physical_switch]]
device_name = "My cover device"
switch = "SW_LID"
state = "lid.closed"
```

Для drop-in-файлов используйте именованные группы вроде
`[[input.sensors.button]]` и `[[input.sensors.switch]]` с теми же полями.
Задавайте отдельную группу для каждого компонента. Таблицы TOML
объединяются, а массивы верхнего уровня `[[physical_button]]` и
`[[physical_switch]]` заменяют предыдущие массивы с тем же именем.

`[[input.sensors.gesture]]` сопоставляет значение `EV_MSC / MSC_GESTURE`
от указанного устройства с именем кнопки. В привязке для этой кнопки
укажите `event = "gesture"`.

```toml
[[input.sensors.gesture]]
device_name = "My gesture device"
source = "sensors"
value = 2
button = "tilt"
```
