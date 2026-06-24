# qrcode-gen

Responsive, lightweight, fast, flexible but not complicated.

## Windows, macOS



## Linux

```bash
# glibc
cargo fetch --target "$(uname -m)-unknown-linux-gnu"
# musl
cargo fetch --target "$(uname -m)-unknown-linux-musl"

# Qt6
cargo build --release --frozen -F qt
# GTK4
cargo build --release --frozen -F gtk
```