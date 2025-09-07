# GitHub Actions Setup Guide

Esta guía te ayuda a configurar GitHub Actions para compilar automáticamente binarios para Windows, Linux y macOS.

## 🚀 Configuración Automática

### 1. Subir los archivos al repositorio

Los archivos ya están creados en tu proyecto:

```
.github/workflows/build-and-release.yml   # Workflow principal
install-quick.sh                          # Instalador rápido Unix
install-quick.ps1                         # Instalador rápido Windows
install.sh                               # Instalador desde código fuente Unix
install.ps1                              # Instalador desde código fuente Windows
```

### 2. Hacer commit y push

```bash
git add .github/ install-quick.sh install-quick.ps1 install.ps1
git commit -m "Add GitHub Actions workflow and installers"
git push origin main
```

### 3. Crear un release para activar la compilación

```bash
# Crear un tag de versión
git tag v1.0.0
git push origin v1.0.0
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
3. **Release** - Si es un tag, crea release con binarios

## 🎯 Uso de los binarios

### Una vez que GitHub Actions crea el release:

**Instalación rápida (descarga binarios precompilados):**
```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/TU_USUARIO/cli-frontend-rust/main/install-quick.sh | bash

# Windows
iwr -useb https://raw.githubusercontent.com/TU_USUARIO/cli-frontend-rust/main/install-quick.ps1 | iex
```

**Instalación desde código fuente (compila localmente):**
```bash
# Linux/macOS
curl -sSL https://raw.githubusercontent.com/TU_USUARIO/cli-frontend-rust/main/install.sh | bash

# Windows
iwr -useb https://raw.githubusercontent.com/TU_USUARIO/cli-frontend-rust/main/install.ps1 | iex
```

## 📋 Ventajas de cada método

### GitHub Actions (Recomendado)
✅ **Pros:**
- No requiere Rust en la máquina del usuario
- Instalación super rápida (solo descarga)
- Soporte automático para múltiples plataformas
- Verificación automática de calidad de código
- Distribución consistente

❌ **Contras:**
- Dependes de GitHub para la compilación
- Requiere configuración inicial

### Compilación Local
✅ **Pros:**
- Control total sobre la compilación
- No dependes de servicios externos
- Puedes compilar versiones personalizadas

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