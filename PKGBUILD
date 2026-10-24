pkgname=audrune
pkgver=0.7.0
pkgrel=1
pkgdesc="A terminal music player written in Rust"
arch=('x86_64')
url="https://github.com/russticbob/audrune"
license=('GPL-3.0-only')
depends=('alsa-lib')
makedepends=('cargo' 'rust')

build() {
    cd "$srcdir"
    cargo build --release --locked --offline \
        --manifest-path "$startdir/Cargo.toml"
}

package() {
    install -Dm755 "$startdir/target/release/audrune" \
        "$pkgdir/usr/bin/audrune"

    install -Dm644 "$startdir/assets/demo/audrune-demo.flac" \
        "$pkgdir/usr/share/audrune/demo/audrune-demo.flac"

    install -Dm644 "$startdir/assets/demo/README.md" \
        "$pkgdir/usr/share/audrune/demo/README.md"

    install -Dm644 "$startdir/README.md" \
        "$pkgdir/usr/share/doc/audrune/README.md"

    install -Dm644 "$startdir/LICENSE" \
        "$pkgdir/usr/share/licenses/audrune/LICENSE"
}
