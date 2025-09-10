# 🐧 Arch Linux / Manjaro Installation Guide

Esta guía cubre las diferentes opciones de instalación para Arch Linux y distribuciones basadas en Arch (Manjaro, EndeavourOS, etc.).

## 🚀 Opciones de Instalación

### 1. Instalación Rápida (Recomendada)

```bash
# Descarga e instala automáticamente desde releases
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install.sh | bash
```

**Características:**
- ✅ Detecta automáticamente el sistema Arch
- ✅ Descarga binarios precompilados optimizados
- ✅ Configuración completa automática
- ✅ Instalación ultra-rápida

### 2. Instalación desde Código Fuente

```bash
# Clonar el repositorio
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust

# Ejecutar instalador (compila automáticamente)
./install.sh
```

**Características:**
- ✅ Compilación local con optimizaciones específicas
- ✅ Control total sobre el proceso
- ✅ Ideal para desarrollo

### 3. Instalación Manual

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

- **pacman** - Gestor oficial de Arch Linux (para dependencias)
- **yay** - AUR helper popular (futuro soporte)
- **paru** - AUR helper moderno (futuro soporte)
- **curl/wget** - Para descarga de binarios optimizados

### Dependencias

**Automáticamente instaladas:**
- `rust` - Compilador Rust
- `curl` - Para descargas
- `unzip` - Para extraer archivos

**Opcionales pero recomendadas:**
- `git` - Para clonar repositorio
- `base-devel` - Para compilación

## 📦 Características Específicas de Arch

### 1. **Gestión de Dependencias Automática**
```bash
# El instalador detecta y usa pacman automáticamente
sudo pacman -S rust curl unzip git base-devel
```

### 2. **Desktop Entry Automática**
```bash
# Se crea automáticamente para instalaciones system-wide
cat /usr/share/applications/cli-frontend.desktop
```

### 3. **Configuración XDG Compliant**
```bash
# Configuración en ~/.config/cli-frontend/config.toml
~/.config/cli-frontend/config.toml
```

### 4. **Integración con AUR Helpers**
```bash
# Compatible con AUR helpers populares
yay -S cli-frontend-git  # Cuando esté disponible en AUR
paru -S cli-frontend-git # Alternativa moderna
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

## 🚀 AUR Package (Planeado)

El proyecto se publicará en AUR cuando haya demanda suficiente:

```bash
# Instalación futura vía AUR:
yay -S cli-frontend-git
# o
paru -S cli-frontend-git
```

**Mientras tanto, los instaladores automáticos proporcionan una experiencia superior:**
- ✅ Instalación más rápida (binarios precompilados)
- ✅ Configuración automática completa
- ✅ Soporte multi-arquitectura
- ✅ Actualizaciones automáticas
- ✅ Sin dependencias de compilación

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
| **Instaladores inteligentes** | ✅ Optimizado | ✅ Optimizado | ✅ Optimizado |
| **Binarios precompilados** | ✅ Disponible | ✅ Disponible | ✅ Disponible |

## 🎉 Próximas Funcionalidades

- [ ] Shell completions (bash, zsh, fish)
- [ ] Man pages
- [ ] Publicación oficial en AUR (cuando haya demanda)
- [ ] Integración nativa con package managers
- [ ] Binarios firmados digitalmente

## 💡 Tips para Usuarios de Arch

1. **Usa instaladores automáticos**: Más rápido que compilar
2. **Instalación system-wide**: Disponible para todos los usuarios
3. **Configuración XDG**: Todo en `~/.config/cli-frontend/`
4. **Binarios optimizados**: Mejor rendimiento que compilación local
5. **Rolling release**: Siempre la última versión disponible vía GitHub Releases