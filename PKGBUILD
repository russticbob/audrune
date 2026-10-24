pkgname=audrune
pkgver=0.7.1
_pkgver=0.7.1-beta
pkgrel=1
pkgdesc="A terminal music player written in Rust"
arch=('x86_64')
url="https://github.com/russticbob/audrune"
license=('GPL-3.0-only')
depends=('alsa-lib' 'glibc' 'gcc-libs')
makedepends=('cargo')
source=(
    "audrune-${_pkgver}.tar.gz::${url}/archive/refs/tags/v${_pkgver}.tar.gz"
)
sha256sums=('SKIP')

prepare() {
    cd "$srcdir/audrune-${_pkgver}"
    export CARGO_HOME="$srcdir/cargo-home"
    cargo fetch --locked
}

build() {
    cd "$srcdir/audrune-${_pkgver}"
    export CARGO_HOME="$srcdir/cargo-home"
    export CARGO_TARGET_DIR="$srcdir/cargo-target"
    cargo build --release --locked --offline
}

package() {
    cd "$srcdir/audrune-${_pkgver}"

    install -Dm755 "$srcdir/cargo-target/release/audrune" \
        "$pkgdir/usr/bin/audrune"

    install -Dm644 assets/demo/audrune-demo.flac \
        "$pkgdir/usr/share/audrune/demo/audrune-demo.flac"

    install -Dm644 assets/demo/README.md \
        "$pkgdir/usr/share/audrune/demo/README.md"

    install -Dm644 README.md \
        "$pkgdir/usr/share/doc/audrune/README.md"

    install -Dm644 LICENSE \
        "$pkgdir/usr/share/licenses/audrune/LICENSE"
}
