# Correcciones de Instalación para Windows

## Problema Identificado

El sistema de instalación en Windows tenía varios problemas que impedían que el ejecutable estuviera disponible inmediatamente después de la instalación:

1. **Ubicación inadecuada**: Se instalaba en `%USERPROFILE%\.cli-template` que no es una ubicación estándar para ejecutables en Windows
2. **PATH no actualizado en sesión actual**: Requería reiniciar el terminal para usar el comando
3. **Falta de verificación**: No se validaba que la instalación funcionara correctamente

## Soluciones Implementadas

### 1. Nueva Ubicación de Instalación
- **Antes**: `%USERPROFILE%\.cli-template`
- **Después**: `%LOCALAPPDATA%\Programs\cli-frontend`

La nueva ubicación sigue las mejores prácticas de Windows para aplicaciones instaladas por usuario.

### 2. Manejo Mejorado del PATH
```powershell
# Actualizar PATH persistente (para futuras sesiones)
[Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallPath", "User")

# Actualizar PATH de la sesión actual (uso inmediato)
$env:PATH += ";$InstallPath"
```

### 3. Verificación de Instalación
Se agregó una prueba automática que ejecuta `cli-frontend --version` para confirmar que la instalación funciona correctamente:

```powershell
try {
    $version = & "$InstallPath\cli-frontend.exe" --version 2>$null
    if ($version) {
        Write-ColorOutput $Green "✅ Installation test successful: $version"
        Write-ColorOutput $Green "✅ You can now use 'cli-frontend' from anywhere!"
    }
} catch {
    Write-ColorOutput $Yellow "⚠️  Could not test installation."
}
```

## Archivos Modificados

### 1. `.github/scripts/generate-installers.sh`
- Cambiada la ubicación por defecto de instalación
- Mejorado el manejo del PATH para incluir sesión actual
- Agregada verificación post-instalación

### 2. `install.ps1`
- Actualizada la ubicación por defecto
- Simplificado y limpiado código duplicado
- Mejorada la lógica de PATH
- Agregada verificación de instalación

### 3. `install-quick.ps1`
- Actualizada la ubicación por defecto
- Eliminado código duplicado (ahora delega al script principal)
- Simplificada la lógica para mayor mantenibilidad

## Beneficios de los Cambios

1. **Uso inmediato**: El comando `cli-frontend` está disponible inmediatamente sin reiniciar terminal
2. **Ubicación estándar**: Sigue las convenciones de Windows para aplicaciones de usuario
3. **Menos duplicación**: Código más mantenible y consistente
4. **Verificación automática**: Confirma que la instalación funciona correctamente
5. **Mejor experiencia**: Mensajes más claros sobre el estado de la instalación

## Pruebas Recomendadas

Para verificar que los cambios funcionen correctamente:

1. **Instalación desde fuente**:
   ```powershell
   .\install.ps1
   cli-frontend --version
   ```

2. **Instalación rápida**:
   ```powershell
   .\install-quick.ps1
   cli-frontend --version
   ```

3. **Verificar ubicación**:
   ```powershell
   where.exe cli-frontend
   # Should show: C:\Users\[usuario]\AppData\Local\Programs\cli-frontend\cli-frontend.exe
   ```

## Compatibilidad

Los cambios son completamente compatibles con versiones anteriores. Los usuarios que ya tengan instalaciones en la ubicación antigua pueden ejecutar el nuevo installer para migrar automáticamente a la nueva ubicación.