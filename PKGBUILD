pkgname=brighty
pkgver=0.1
pkgrel=0
pkgdesc="Brightness Controller For nVidia Laptop"
arch=("x86_64")
url="https://github.com/Mordanis/brighty"
license=("MIT")
makedepends=(cargo git)
source=('https://github.com/Mordanis/brighty/archive/refs/tags/v0.1.tar.gz')
sha256sums=('e6a396ce6be720e1193782ee2d298d92c3cec60e05fc2feab5b9a77a69046ada')
prepare() {
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}
build() {
    export RUSTUP_TOOLCHAIN=nightly
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release --all
}
package() {
    find target/release \
        -maxdepth 1 \
        -executable \
        -type f \
        -exec install -Dm0755 -t "$pkgdir/usr/bin/" {} +
}
