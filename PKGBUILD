pkgname=evntwire
pkgver=0.1.0
pkgrel=1
pkgdesc='Linux event routing daemon'
arch=('x86_64' 'aarch64')
url='https://github.com/AlexKletn/evntwire'
license=('GPL-2.0-only')
depends=('glibc' 'libgcc')
makedepends=('rust')
checkdepends=('coreutils')
options=('!debug')
source=()
sha256sums=()

prepare() {
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu" \
        --manifest-path "$startdir/Cargo.toml"
}

build() {
    cargo build --frozen --release --package evntwire --bin evntwire \
        --manifest-path "$startdir/Cargo.toml" --target-dir "$srcdir/target"
}

check() {
    cargo test --frozen --workspace \
        --manifest-path "$startdir/Cargo.toml" --target-dir "$srcdir/target"
}

package() {
    install -Dm755 "$srcdir/target/release/evntwire" "$pkgdir/usr/bin/evntwire"
    install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
