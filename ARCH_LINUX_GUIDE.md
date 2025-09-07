# 🐧 Arch Linux / Manjaro Installation Guide

Esta guía cubre las diferentes opciones de instalación para Arch Linux y distribuciones basadas en Arch (Manjaro, EndeavourOS, etc.).

## 🚀 Opciones de Instalación

### 1. Instalación Rápida (Recomendada)

```bash
# Descarga e instala automáticamente
curl -sSL https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install-arch.sh | bash
```

**Características:**
- ✅ Detecta automáticamente el sistema Arch
- ✅ Instala dependencias con pacman
- ✅ Compila automáticamente si no hay binario
- ✅ Configuración completa automática

### 2. Instalación Manual con Script Especializado

```bash
# Clonar el repositorio
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust

# Hacer ejecutable el instalador de Arch
chmod +x install-arch.sh

# Instalación para usuario actual
./install-arch.sh

# O instalación system-wide (requiere sudo)
sudo ./install-arch.sh
```

### 3. Instalación desde AUR (PKGBUILD)

```bash
# Clonar el repositorio
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust

# Usar makepkg para instalar
makepkg -si

# O con un AUR helper
yay -S cli-frontend-git
# o
paru -S cli-frontend-git
```

### 4. Compilación Manual

```bash
# Instalar dependencias
sudo pacman -S rust

# Clonar y compilar
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust
cargo build --release

# Instalación manual
sudo cp target/release/cli-frontend /usr/local/bin/
sudo mkdir -p /usr/local/share/cli-frontend
sudo cp -r templates /usr/local/share/cli-frontend/
```

## 🔧 Configuración Específica para Arch

### Instalación System-wide vs User

**System-wide (sudo):**
- Binary: `/usr/local/bin/cli-frontend`
- Templates: `/usr/local/share/cli-frontend/templates`
- Desktop entry: `/usr/share/applications/cli-frontend.desktop`
- Disponible para todos los usuarios

**User installation:**
- Binary: `~/.local/bin/cli-frontend`
- Templates: `~/.cli-template/templates`
- Config: `~/.config/cli-frontend/config.toml`
- Solo para el usuario actual

### Gestores de Paquetes Soportados

El instalador detecta y usa automáticamente:

- **pacman** - Gestor oficial de Arch Linux
- **yay** - AUR helper popular
- **paru** - AUR helper moderno
- **makepkg** - Para compilación manual de PKGBUILD

### Dependencias

**Automáticamente instaladas:**
- `rust` - Compilador Rust
- `curl` - Para descargas
- `unzip` - Para extraer archivos

**Opcionales pero recomendadas:**
- `git` - Para clonar repositorio
- `base-devel` - Para compilación

## 📦 Características Específicas de Arch

### 1. **PKGBUILD incluido**
```bash
# El proyecto incluye un PKGBUILD para fácil instalación
makepkg -si
```

### 2. **Información de paquete estilo AUR**
```bash
# Se crea información de paquete en /var/lib/cli-frontend/PKGINFO
cat /var/lib/cli-frontend/PKGINFO
```

### 3. **Desktop Entry automática**
```bash
# Se crea automáticamente para instalaciones system-wide
cat /usr/share/applications/cli-frontend.desktop
```

### 4. **Configuración XDG compliant**
```bash
# Configuración en ~/.config/cli-frontend/config.toml
~/.config/cli-frontend/config.toml
```

## 🧪 Verificación de Instalación

```bash
# Verificar que el comando está disponible
which cli-frontend
cli-frontend --version

# Verificar templates disponibles
cli-frontend --help

# Crear componente de prueba
mkdir test-project && cd test-project
cli-frontend TestComponent --type component
```

## 🚀 AUR Package (Futuro)

El proyecto está preparado para ser publicado en AUR:

```bash
# Una vez publicado en AUR, será posible instalar con:
yay -S cli-frontend-git
# o
paru -S cli-frontend-git
```

**PKGBUILD features:**
- ✅ Compilación automática desde Git
- ✅ Verificación con cargo test
- ✅ Instalación de documentación
- ✅ Desktop entry
- ✅ Preparado para shell completions
- ✅ Preparado para man pages

## 🔧 Troubleshooting

### Error: "pacman: command not found"
```bash
# No estás en Arch Linux. Usa otro instalador:
curl -sSL https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install.sh | bash
```

### Error: "cargo: command not found"
```bash
# Instalar Rust
sudo pacman -S rust
# o con rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Error: "Permission denied"
```bash
# Para instalación system-wide, usar sudo
sudo ./install-arch.sh
```

### Templates no encontrados
```bash
# Verificar configuración
cat ~/.config/cli-frontend/config.toml

# O reinstalar
./install-arch.sh
```

## 🎯 Diferencias con Otras Distribuciones

| Característica | Arch/Manjaro | Ubuntu/Debian | Fedora |
|----------------|-------------|--------------|---------|
| **Gestor de paquetes** | pacman/yay | apt | dnf |
| **Instalación system-wide** | `/usr/local/` | `/usr/local/` | `/usr/local/` |
| **Config directory** | `~/.config/` | `~/.config/` | `~/.config/` |
| **Desktop entry** | ✅ Automática | ✅ Automática | ✅ Automática |
| **PKGBUILD** | ✅ Incluido | ❌ N/A | ❌ N/A |
| **AUR ready** | ✅ Preparado | ❌ N/A | ❌ N/A |

## 🎉 Próximas Funcionalidades

- [ ] Shell completions (bash, zsh, fish)
- [ ] Man pages
- [ ] Publicación oficial en AUR
- [ ] Integración con makepkg
- [ ] Arch Linux package signing

## 💡 Tips para Usuarios de Arch

1. **Usa AUR helpers**: yay o paru hacen todo automático
2. **Instalación system-wide**: Disponible para todos los usuarios
3. **Configuración XDG**: Todo en `~/.config/cli-frontend/`
4. **PKGBUILD**: Perfecto para desarrollo y testing
5. **Rolling release**: Siempre la última versión disponible