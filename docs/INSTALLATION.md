# 🌍 Instalación Global del CLI

Esta guía explica cómo instalar el CLI Frontend Generator como herramienta global del sistema.

> 🎉 **Versión 1.2.2** - Instaladores mejorados con configuración automática

## 🚀 Instalación Automática (Recomendada)

Los instaladores automáticos de la v1.2.2 detectan tu plataforma, descargan/compilan el CLI, y **crean automáticamente la configuración necesaria**.

### Instalación Rápida

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install-quick.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install-quick.ps1 | iex
```

### Instalación desde Código Fuente

**Linux/macOS:**
```bash
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust
chmod +x install.sh
./install.sh  # Compila automáticamente y crea configuración
```

**Windows (PowerShell):**
```powershell
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust
./install.ps1  # Compila automáticamente y crea configuración
```

> ✨ **Nuevo en v1.2.2**: Los instaladores crean automáticamente:
> - Archivo de configuración `.cli-frontend.conf`
> - Configuración de rutas de templates y arquitecturas
> - Variables de entorno necesarias

### Instalación Manual

```bash
# 1. Copiar el binario a /usr/local/bin
sudo cp target/release/cli-frontend /usr/local/bin/

# 2. Crear directorio para templates
sudo mkdir -p /usr/local/share/cli-frontend/templates
sudo cp -r templates/* /usr/local/share/cli-frontend/templates/

# 3. Hacer ejecutable
sudo chmod +x /usr/local/bin/cli-frontend

# 4. Crear configuración global (opcional)
mkdir -p ~/.config/cli-frontend
cat > ~/.config/cli-frontend/config.conf << EOF
default_type=component
create_folder=true
enable_hooks=true
templates_dir=/usr/local/share/cli-frontend/templates
output_dir=.
EOF
```

## 🪟 Windows

### Usando PowerShell (Como Administrador)

```powershell
# 1. Crear directorio para el CLI
New-Item -ItemType Directory -Path "C:\Program Files\cli-frontend" -Force

# 2. Copiar el ejecutable
Copy-Item "target\release\cli-frontend.exe" "C:\Program Files\cli-frontend\"

# 3. Copiar templates
New-Item -ItemType Directory -Path "C:\Program Files\cli-frontend\templates" -Force
Copy-Item -Recurse "templates\*" "C:\Program Files\cli-frontend\templates\"

# 4. Agregar al PATH del sistema
$env:PATH += ";C:\Program Files\cli-frontend"
[Environment]::SetEnvironmentVariable("PATH", $env:PATH, "Machine")
```

### Instalación Usuario (Sin permisos de administrador)

```powershell
# 1. Crear directorio en el perfil del usuario
$userBin = "$env:USERPROFILE\.local\bin"
New-Item -ItemType Directory -Path $userBin -Force

# 2. Copiar ejecutable
Copy-Item "target\release\cli-frontend.exe" "$userBin\"

# 3. Copiar templates
$userTemplates = "$env:USERPROFILE\.local\share\cli-frontend\templates"
New-Item -ItemType Directory -Path $userTemplates -Force
Copy-Item -Recurse "templates\*" $userTemplates

# 4. Agregar al PATH del usuario
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$userBin*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$userBin", "User")
}
```

## 📋 Verificación de Instalación

Una vez instalado, verifica que funcione:

```bash
# Verificar que el CLI esté disponible
which cli-frontend  # Linux/macOS
where cli-frontend  # Windows

# Mostrar ayuda y templates disponibles
cli-frontend --help

# Probar generación desde cualquier directorio
cd /tmp  # Linux/macOS
cd %TEMP%  # Windows
cli-frontend TestComponent --type component --output-dir ./test-output
```

## 🔧 Configuración Global

### Ubicaciones de Configuración

El CLI busca configuración en este orden:

1. **Archivo específico**: `--config ./mi-config.conf`
2. **Directorio actual**: `.cli-frontend.conf`
3. **Usuario Linux/macOS**: `~/.config/cli-frontend/config.conf`
4. **Usuario Windows**: `%USERPROFILE%\.cli-frontend.conf`
5. **Home directory**: `~/.cli-frontend.conf`

### Ejemplo de Configuración Global

```ini
# ~/.config/cli-frontend/config.conf (Linux/macOS)
# %USERPROFILE%\.cli-frontend.conf (Windows)

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Global paths
templates_dir=/usr/local/share/cli-frontend/templates  # Linux/macOS
# templates_dir=C:\Program Files\cli-frontend\templates  # Windows
output_dir=.  # Genera en directorio actual
```

## 📁 Búsqueda de Templates

El CLI busca templates en estas ubicaciones (en orden):

### Linux/macOS:
1. `./templates` (directorio actual)
2. `~/.cli-template` (usuario)
3. `~/.config/cli-frontend/templates` (XDG config)
4. `/usr/local/share/cli-frontend/templates` (sistema)
5. `/usr/share/cli-frontend/templates` (sistema alternativo)

### Windows:
1. `.\templates` (directorio actual)
2. `%USERPROFILE%\.cli-template` (usuario)
3. `C:\Program Files\cli-frontend\templates` (sistema)

## 🌟 Uso Desde Cualquier Directorio

Una vez instalado globalmente, puedes usar el CLI desde cualquier ubicación:

```bash
# Ir a tu proyecto
cd /path/to/your/react/project

# Generar componentes directamente
cli-frontend Header --type component --output-dir ./src/components

# Generar hooks
cli-frontend useApi --type hook --output-dir ./src/hooks

# Generar stores
cli-frontend UserStore --type store --output-dir ./src/store

# Usar configuración específica del proyecto
cli-frontend Modal --type component --config ./project-config.conf
```

## 🔄 Actualización

Para actualizar a una nueva versión:

```bash
# 1. Actualizar el código
cd cli-frontend-rust
git pull origin main
cargo build --release

# 2. Reinstalar
sudo ./install.sh  # Linux/macOS
# O seguir los pasos de Windows nuevamente
```

## 🗑️ Desinstalación

### Linux/macOS:

```bash
# Remover binario
sudo rm /usr/local/bin/cli-frontend

# Remover templates
sudo rm -rf /usr/local/share/cli-frontend

# Remover configuración usuario (opcional)
rm -rf ~/.config/cli-frontend
rm ~/.cli-frontend.conf
```

### Windows:

```powershell
# Remover directorio completo
Remove-Item -Recurse -Force "C:\Program Files\cli-frontend"

# Remover del PATH (requiere reinicio o nueva sesión)
# Usar Panel de Control > Sistema > Variables de Entorno
```

## 🎯 Ventajas de la Instalación Global

✅ **Disponible desde cualquier directorio**
✅ **Configuración centralizada**
✅ **Templates compartidos entre proyectos**
✅ **Fácil actualización**
✅ **Integración con herramientas de desarrollo**
✅ **Soporte para múltiples proyectos**

¡Ahora puedes usar `cli-frontend` desde cualquier ubicación en tu sistema! 🚀