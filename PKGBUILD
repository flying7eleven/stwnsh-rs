# Maintainer: Tim Janke <tim+github@janke.biz>
pkgname=stwnsh-rs
pkgver=2022.9.25
pkgrel=1
license=('MIT')
#makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="A small and easy to use tool for hashing passwords with different hash functions."

build() {
    return 0
}

package() {
    cd $srcdir
    cargo install --root="$pkgdir/usr" --git=https://github.com/flying7eleven/stwnsh-rs --tag=2022.9.25
    rm "${pkgdir}/usr/.crates2.json"
    rm "${pkgdir}/usr/.crates.toml"
}
