# evntwire

[Русская версия](README.ru.md)

evntwire connects event sources to configurable actions on Linux.

The workspace contains `crates/evntwire`, `crates/evntwire-config`, and the GUI placeholder `crates/evntwire-gui`.

Run `evntwire SOCKET_PATH [CONFIG_PATH]` to receive events over a Unix datagram socket. Use an absolute socket path inside a private directory. The default configuration is `/etc/evntwire/evntwire.toml`.

On startup, evntwire creates an empty configuration file and any missing parent directories if the selected file is absent.

On Arch Linux, run `makepkg -s` from the checkout to build the package.

To check the workspace, run:

```sh
cargo test --workspace --locked
```
