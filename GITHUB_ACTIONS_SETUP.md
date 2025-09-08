# GitHub Actions Setup Guide

Esta guía te ayuda a configurar GitHub Actions para compilar automáticamente binarios para Windows, Linux y macOS, con **instaladores inteligentes que incluyen configuración automática**.

> 🎉 **Actualizado para v1.2.3** - Instaladores con configuración automática de archivos `.conf`

## 🚀 Configuración Automática

### 1. Subir los archivos al repositorio

Los archivos ya están creados en tu proyecto:

```
.github/workflows/build-and-release.yml   # Workflow principal con instaladores mejorados
install-quick.sh                          # Instalador rápido Unix (con configuración automática)
install-quick.ps1                         # Instalador rápido Windows (con configuración automática)
install.sh                               # Instalador desde código fuente Unix
install.ps1                              # Instalador desde código fuente Windows
```

> ✨ **Nuevo en v1.2.3**: Los instaladores ahora crean automáticamente el archivo `.cli-frontend.conf` con las rutas correctas

### 2. Hacer commit y push

```bash
git add .github/ install-quick.sh install-quick.ps1 install.ps1
git commit -m "Add GitHub Actions workflow and installers v1.2.3"
git push origin main
```

### 3. Crear un release para activar la compilación

```bash
# Crear un tag de versión
git tag v1.2.3
git push origin v1.2.3
```

## 🔧 Cómo funciona

### Eventos que activan la compilación:

1. **Push a main** - Compila pero no crea release
2. **Pull Request** - Compila para verificar
3. **Tags v\*** - Compila y crea release automáticamente

### Plataformas soportadas:

- ✅ **Linux x86_64** - `cli-frontend-linux-x86_64`
- ✅ **Windows x86_64** - `cli-frontend-windows-x86_64.exe`
- ✅ **macOS Intel** - `cli-frontend-macos-x86_64`
- ✅ **macOS ARM (M1/M2)** - `cli-frontend-macos-aarch64`

### Proceso completo:

1. **Build** - Compila para todas las plataformas
2. **Test** - Ejecuta tests, clippy y formatting
3. **Create installers** - Genera instaladores con configuración automática integrada
4. **Release** - Si es un tag, crea release con binarios e instaladores

## 🎯 Uso de los binarios

### Una vez que GitHub Actions crea el release:

**Instalación rápida desde GitHub (descarga binarios precompilados con configuración automática):**
```bash
# Linux/macOS
curl -sSL https://github.com/TU_USUARIO/CLI-FRONTEND-RUST/releases/latest/download/install-linux.sh | bash

# Windows (PowerShell)
iwr -useb https://github.com/TU_USUARIO/CLI-FRONTEND-RUST/releases/latest/download/install-windows.ps1 | iex
```

> ✨ **Nuevo en v1.2.3**: Los instaladores del release incluyen creación automática del archivo `.cli-frontend.conf`

**Instalación desde código fuente (compila localmente):**
```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/TU_USUARIO/cli-frontend-rust/main/install.sh | bash

# Windows
iwr -useb https://raw.githubusercontent.com/TU_USUARIO/cli-frontend-rust/main/install.ps1 | iex
```

## 📋 Ventajas de cada método

### Instalación desde GitHub Release (Recomendado v1.2.3)
✅ **Pros:**
- No requiere Rust en la máquina del usuario
- Instalación super rápida (solo descarga)
- **Configuración automática** del archivo `.cli-frontend.conf`
- Soporte automático para múltiples plataformas
- Verificación automática de calidad de código
- Distribución consistente
- **Rutas correctas configuradas automáticamente**

❌ **Contras:**
- Dependes de GitHub para la compilación
- Requiere configuración inicial

### Instalación desde Código Fuente
✅ **Pros:**
- Control total sobre la compilación
- No dependes de servicios externos
- Puedes compilar versiones personalizadas
- **También incluye configuración automática** (v1.2.3)

❌ **Contras:**
- Requiere Rust instalado localmente
- Proceso de compilación más lento
- Posibles diferencias entre plataformas

> 🔧 **Nota v1.2.3**: Ambos métodos ahora crean automáticamente la configuración necesaria, incluyendo rutas correctas para templates y arquitecturas.

❌ **Contras:**
- Requiere Rust instalado
- Tiempo de compilación más largo
- Puede haber diferencias entre sistemas

## 🔄 Workflow de desarrollo recomendado

1. **Desarrollo diario** - Push a main activa compilación de verificación
2. **Nuevas versiones** - Crear tag `v1.x.x` para release automático
3. **Hotfixes** - Tag `v1.x.y` para parches rápidos

## 🧪 Testing del workflow

Para probar que todo funciona:

```bash
# 1. Crear un tag de prueba
git tag v0.1.0-test
git push origin v0.1.0-test

# 2. Verificar en GitHub Actions
# Ir a tu repositorio → Actions → Ver el workflow corriendo

# 3. Una vez completado, verificar los binarios en Releases
# Ir a tu repositorio → Releases → Verificar que se crearon los assets
```

## 🚨 Troubleshooting

### Error: "Workflow not running"
- Verifica que el archivo esté en `.github/workflows/`
- Asegúrate de hacer push del directorio `.github/`

### Error: "Build failed"
- Revisa que `Cargo.toml` esté correctamente configurado
- Verifica que no hay errores de clippy: `cargo clippy -- -D warnings`
- Verifica formatting: `cargo fmt -- --check`

### Error: "Release not created"
- Asegúrate de crear un tag que empiece con 'v': `v1.0.0`
- Verifica que el workflow de build pasó exitosamente

## 💡 Tips adicionales

1. **Semantic Versioning**: Usa versionado semántico (v1.0.0, v1.0.1, v1.1.0)
2. **Changelog**: Mantén un CHANGELOG.md para documentar cambios
3. **Pre-releases**: Usa `v1.0.0-beta.1` para versiones de prueba
4. **Draft releases**: El workflow puede configurarse para crear drafts primero