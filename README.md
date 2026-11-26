# evntwire

[Русская версия](README.ru.md)

[Contributing](CONTRIBUTING.md) · [GPL-2.0-only](LICENSE)

evntwire connects event sources to configurable actions on Linux.

The workspace contains `crates/evntwire`, `crates/evntwire-config`, and the GUI placeholder `crates/evntwire-gui`.

Run `evntwire SOCKET_PATH [CONFIG_PATH]` to receive events over a Unix datagram socket. Use an absolute socket path inside a private directory. The default configuration is `/etc/evntwire/evntwire.toml`.

On startup, evntwire creates an empty configuration file and any missing parent directories if the selected file is absent.

On Arch Linux, run `makepkg -s` from the checkout to build the package.

To check the workspace, run:

```sh
cargo test --workspace --locked
```

## Linux input devices

Evntwire reads configured evdev devices by exact name and can attach them
after startup. It observes input without taking an exclusive grab. A key
binding uses `source` if set, and otherwise uses the device name. A switch
publishes its current state when the device connects and updates it on
changes. The state becomes unknown when the device disconnects.

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

For drop-ins, use named input groups such as `[[input.sensors.button]]`
and `[[input.sensors.switch]]`, with the same fields as above.
Put each component in a distinct group. TOML tables merge recursively,
while a top-level `[[physical_button]]` or `[[physical_switch]]` array
replaces the previous array with the same name.

`[[input.sensors.gesture]]` maps an `EV_MSC / MSC_GESTURE` value from a
named device to a button name. Use `event = "gesture"` in a binding for
that button.

```toml
[[input.sensors.gesture]]
device_name = "My gesture device"
source = "sensors"
value = 2
button = "tilt"
```
