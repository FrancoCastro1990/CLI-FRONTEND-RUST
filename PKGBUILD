# Maintainer: Franco Castro <franco.castro.dev@gmail.com>
pkgname=cli-frontend-git
_pkgname=cli-frontend-rust
pkgver=r1.0.0
pkgrel=1
pkgdesc="A powerful CLI tool for generating React components, hooks, services, and more"
arch=('x86_64' 'i686' 'aarch64')
url="https://github.com/FrancoCastro1990/cli-frontend-rust"
license=('MIT')
depends=()
makedepends=('rust' 'cargo')
provides=('cli-frontend')
conflicts=('cli-frontend')
source=("$_pkgname::git+https://github.com/FrancoCastro1990/cli-frontend-rust.git")
sha256sums=('SKIP')

pkgver() {
    cd "$_pkgname"
    printf "r%s.%s" "$(git rev-list --count HEAD)" "$(git rev-parse --short HEAD)"
}

prepare() {
    cd "$_pkgname"
    cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
    cd "$_pkgname"
    export RUSTUP_TOOLCHAIN=stable
    export CARGO_TARGET_DIR=target
    cargo build --frozen --release
}

check() {
    cd "$_pkgname"
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen
}

package() {
    cd "$_pkgname"
    
    # Install binary
    install -Dm755 target/release/cli-frontend "$pkgdir/usr/bin/cli-frontend"
    
    # Install templates
    install -dm755 "$pkgdir/usr/share/cli-frontend"
    cp -r templates "$pkgdir/usr/share/cli-frontend/"
    
    # Install documentation
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 TEMPLATE_GUIDE.md "$pkgdir/usr/share/doc/$pkgname/TEMPLATE_GUIDE.md"
    install -Dm644 INSTALLATION.md "$pkgdir/usr/share/doc/$pkgname/INSTALLATION.md"
    
    # Install license
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    
    # Install desktop entry
    install -Dm644 /dev/stdin "$pkgdir/usr/share/applications/cli-frontend.desktop" << EOF
[Desktop Entry]
Version=1.0
Type=Application
Name=CLI Frontend Generator
Comment=Generate React components, hooks, services, and more
Exec=cli-frontend
Icon=code
Terminal=true
Categories=Development;
EOF

    # Install shell completion (if available)
    # TODO: Add shell completion generation
    
    # Install man page (if available)
    # TODO: Add man page generation
}

# vim:set ts=2 sw=2 et: