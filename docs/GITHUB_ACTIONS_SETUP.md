# GitHub Actions Setup Guide

Esta guía documenta el sistema de GitHub Actions implementado para compilar automáticamente binarios para Windows, Linux y macOS, con **instaladores inteligentes generados dinámicamente**.

> 🎉 **Sistema Actual** - Generación automática de instaladores con configuración completa

## 🚀 Arquitectura del Sistema

### Archivos del Sistema GitHub Actions

```
.github/
├── workflows/
│   ├── ci.yml                          # Integración continua (tests, clippy, fmt)
│   └── build-and-release.yml          # Build y release automático
└── scripts/
    ├── common.sh                       # Funciones comunes para instaladores
    └── generate-installers.sh         # Generador dinámico de instaladores
```

### Archivos de Instalación Local

```
install-quick.sh                        # Instalador rápido Unix (legacy)
install-quick.ps1                       # Instalador rápido Windows (legacy)
install.sh                             # Instalador desde código fuente Unix
install.ps1                            # Instalador desde código fuente Windows
```

> ✨ **Diferencia clave**: Los instaladores del release se generan dinámicamente y son más completos que los archivos locales

## 🔧 Funcionamiento del Sistema

### Workflows Implementados

#### 1. **Continuous Integration (`ci.yml`)**
- **Triggers**: Push a `main/develop`, Pull Requests
- **Plataformas**: Linux, Windows, macOS
- **Acciones**:
  - Tests con `cargo test --verbose`
  - Linting con `cargo clippy -- -D warnings`
  - Format check con `cargo fmt -- --check`
  - Build verification en múltiples plataformas
  - Cache inteligente de dependencias

#### 2. **Build and Release (`build-and-release.yml`)**
- **Triggers**: Tags que empiecen con `v*` (ej: `v1.0.0`)
- **Plataformas y Targets**:
  - Linux: `x86_64-unknown-linux-gnu`
  - Windows: `x86_64-pc-windows-msvc`
  - macOS Intel: `x86_64-apple-darwin`
  - macOS ARM: `aarch64-apple-darwin`

### Proceso de Release Automático

1. **Build Phase**: Compilación para todas las plataformas
2. **Artifact Upload**: Subida de binarios como artifacts
3. **Installer Generation**: Generación dinámica de instaladores usando `generate-installers.sh`
4. **Release Creation**: Creación automática del release con:
   - Binarios para todas las plataformas
   - Checksums SHA256 para verificación de seguridad
   - Instaladores específicos por plataforma
   - Instalador universal
   - Documentación de instalación

## 🎯 Instaladores Generados

El sistema genera automáticamente los siguientes instaladores en cada release:

### Instaladores Disponibles en Release
- `install.sh` - Instalador universal (detecta Linux/macOS)
- `install-linux.sh` - Instalador específico para Linux
- `install-macos.sh` - Instalador específico para macOS (detecta Intel/ARM)
- `install-windows.ps1` - Instalador para Windows PowerShell
- `INSTALL_README.md` - Guía completa de instalación
- `checksums.txt` - Checksums SHA256 para verificación

### Características de los Instaladores Generados

#### Funcionalidades Avanzadas
- ✅ **Detección automática de arquitectura** (Intel/ARM en macOS)
- ✅ **Configuración automática** del archivo `.cli-frontend.conf`
- ✅ **Descarga con retry logic** (3 intentos automáticos)
- ✅ **Verificación de integridad** del binario descargado
- ✅ **Gestión automática del PATH** según el shell detectado
- ✅ **Descarga de templates y arquitecturas** desde el repositorio
- ✅ **Colores en la salida** para mejor experiencia de usuario
- ✅ **Testing automático** de la instalación completada

#### Rutas de Instalación
- **Linux/macOS**: `$HOME/.local/bin/` (binario), `$HOME/.cli-template/` (assets)
- **Windows**: `$env:LOCALAPPDATA\Programs\cli-frontend\` (todo incluido)

## 🚀 Uso del Sistema

### Para crear un nuevo release:

```bash
# 1. Crear un tag de versión
git tag v1.3.0
git push origin v1.3.0

# 2. El sistema automáticamente:
#    - Compila binarios para todas las plataformas
#    - Genera instaladores dinámicos
#    - Crea el release con todos los assets
#    - Publica con release notes automáticas
```

### Para usuarios finales:

**Instalación rápida desde GitHub Release (Recomendado):**
```bash
# Universal (detecta Linux/macOS automáticamente)
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install.sh | bash

# Específico por plataforma
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-linux.sh | bash
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-macos.sh | bash

# Windows (PowerShell como Administrador)
iwr -useb https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-windows.ps1 | iex
```

**Verificación de seguridad:**
```bash
# Descargar y verificar checksums
curl -L -O https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/checksums.txt
curl -L -O https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/cli-frontend-linux-x86_64
sha256sum -c checksums.txt
```

## 📋 Comparación de Métodos de Instalación

### Instalación desde GitHub Release (Recomendado)
✅ **Ventajas:**
- **Instalación ultra-rápida** (solo descarga, no compilación)
- **Binarios optimizados** compilados con flags de release
- **Configuración completamente automática** del archivo `.cli-frontend.conf`
- **Detección inteligente de plataforma y arquitectura**
- **Verificación de integridad** con checksums SHA256
- **Gestión automática del PATH** y variables de entorno
- **Templates y arquitecturas incluidos** automáticamente
- **Testing post-instalación** para verificar funcionamiento
- **Rollback automático** en caso de fallas

❌ **Desventajas:**
- Requiere conexión a internet
- Dependencia de GitHub Releases

### Instalación desde Código Fuente (Local)
✅ **Ventajas:**
- **Control total** sobre el proceso de compilación
- **Personalización** de flags de compilación
- **Funcionamiento offline** (una vez clonado el repo)
- **Compilación con optimizaciones específicas** del hardware local

❌ **Desventajas:**
- **Requiere Rust y Cargo** instalados
- **Tiempo de compilación** considerablemente mayor
- **Posibles errores de compilación** por dependencias del sistema
- **Configuración manual** de rutas y PATH

## 🔄 Workflow de Desarrollo

### Desarrollo Diario
```bash
git add .
git commit -m "feat: new feature"
git push origin main
# → Ejecuta CI: tests, clippy, format check, build verification
```

### Release de Nueva Versión
```bash
# Para versión mayor/menor
git tag v1.4.0
git push origin v1.4.0
# → Ejecuta CI + Build + Release automático

# Para hotfix
git tag v1.3.1
git push origin v1.3.1
# → Ejecuta CI + Build + Release automático
```

### Prereleases (Beta/RC)
```bash
git tag v1.4.0-beta.1
git push origin v1.4.0-beta.1
# → Crea prerelease marcado como "prerelease: true"
```

## 🧪 Testing y Verificación

### Verificar el Workflow
1. **Crear tag de prueba**:
   ```bash
   git tag v0.1.0-test
   git push origin v0.1.0-test
   ```

2. **Monitorear ejecución**:
   - Ve a `GitHub → Actions`
   - Observa ambos workflows (`CI` y `Build and Release`)
   - Verifica que todos los jobs pasen exitosamente

3. **Verificar release**:
   - Ve a `GitHub → Releases`
   - Confirma que se creó el release con todos los assets:
     - 4 binarios (Linux, Windows, macOS Intel, macOS ARM)
     - 4 instaladores específicos + 1 universal
     - Checksums y documentación

4. **Testing de instaladores**:
   ```bash
   # Probar instalador en un entorno limpio
   curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install.sh | bash
   
   # Verificar instalación
   cli-frontend --version
   cli-frontend --help
   ```

## 🚨 Troubleshooting

### CI Workflow Issues

**Error: "Tests failed"**
```bash
# Ejecutar localmente para debugear
cargo test --verbose
cargo clippy -- -D warnings
cargo fmt -- --check
```

**Error: "Build failed on specific platform"**
- Revisa dependencias específicas de plataforma en `Cargo.toml`
- Verifica features condicionales por OS
- Comprueba que el código sea compatible cross-platform

### Release Workflow Issues

**Error: "Installer generation failed"**
- Verifica que `.github/scripts/generate-installers.sh` tenga permisos de ejecución
- Revisa que `common.sh` esté correctamente estructurado
- Confirma que las rutas en el script generador sean correctas

**Error: "Release not created"**
- Asegúrate que el tag empiece con 'v': `v1.0.0`
- Verifica que el build workflow completó exitosamente
- Confirma permisos de `contents: write` en el workflow

**Error: "Binary not executable"**
- El workflow automáticamente hace `chmod +x` en binarios Unix
- Para Windows, verifica que la extensión `.exe` esté presente

### Installers Issues

**Error: "Download failed"**
- Los instaladores incluyen retry logic (3 intentos)
- Verifica conectividad de red
- Confirma que el release contiene los assets esperados

**Error: "PATH not updated"**
- Los instaladores detectan el shell automáticamente
- Para casos edge, recarga el shell: `source ~/.bashrc` (o equivalente)

## 💡 Optimizaciones y Mejores Prácticas

### Cache Strategy
- El workflow usa caché inteligente basado en `Cargo.lock` y SHA del commit
- Cache separado por plataforma para optimizar tiempo de build
- Limpieza automática de cache obsoleto

### Security Measures
- **Checksums SHA256** generados para todos los binarios
- **Verificación de integridad** en instaladores
- **Permisos mínimos** necesarios en workflows
- **Artifacts cleanup** automático después del release

### Release Notes Automation
- Release notes automáticas generadas por GitHub
- Incluye **quick install commands** en el cuerpo del release
- **Ejemplos de uso** y **verificación de seguridad** incluidos
- **Lista de features** y **soporte de plataformas** documentado

### Maintenance
```bash
# Limpiar tags de testing
git tag -d v0.1.0-test
git push --delete origin v0.1.0-test

# Ver todos los workflows
gh run list

# Cancelar workflows en progreso si es necesario  
gh run cancel [RUN_ID]
```

## 🎯 Próximas Mejoras Planificadas

- [ ] Soporte para más arquitecturas (ARM32, RISC-V)
- [ ] Builds para Alpine Linux (musl)
- [ ] Distribución via package managers (brew, scoop, apt)
- [ ] Instaladores con actualización automática
- [ ] Telemetría opcional de instalación
- [ ] Docker images automáticas
````