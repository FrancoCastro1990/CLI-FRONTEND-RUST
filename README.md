# CLI Frontend Generator 🚀

Un potente generador CLI escrito en Rust para crear componentes React, hooks, servicios, contextos y páginas con soporte completo para TypeScript, SCSS modules y plantillas de testing.

## ✨ Características

- 🎯 **Generación automática** de componentes React con TypeScript
- 🪝 **Hooks personalizados** con patrones optimizados
- 🔧 **Servicios** para lógica de negocio
- 🌐 **Contextos** con proveedores automáticos
- 📄 **Páginas** con estructura completa
- 🎨 **SCSS Modules** incluidos por defecto
- 🧪 **Archivos de testing** (.spec.tsx) automáticos
- ⚙️ **Configuración personalizable** mediante archivos .conf
- 📝 **Múltiples convenciones de nomenclatura** (PascalCase, camelCase, snake_case, kebab-case)
- 🔄 **Sistema de templates** avanzado con Handlebars
- ✨ **Detección automática de templates** - No requiere recompilación para agregar nuevos templates
- 🚀 **Templates dinámicos** - El CLI descubre automáticamente carpetas de templates

## 🛠️ Instalación

### Desde los binarios compilados

1. Descarga el binario para tu sistema desde las releases
2. Coloca el ejecutable en tu PATH
3. ¡Ya está listo para usar!

### Compilación desde el código fuente

```bash
# Clona el repositorio
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust

# Compila en modo release
cargo build --release

# El binario estará en target/release/cli-frontend.exe (Windows)
```

## 🚀 Uso Rápido

```bash
# Ver templates disponibles
cli-frontend --help

# Generar un componente
cli-frontend Button --type component

# Generar un hook
cli-frontend useAuth --type hook

# Generar un servicio
cli-frontend ApiService --type service

# Generar un contexto
cli-frontend UserContext --type context

# Generar una página
cli-frontend HomePage --type page

# Generar un store Redux
cli-frontend UserStore --type store

# Generar una API service
cli-frontend UserApi --type api

# Generar sin crear carpeta
cli-frontend Modal --type component --no-folder

# Usar configuración personalizada
cli-frontend Login --config ./mi-config.conf
```

## 📋 Comandos Disponibles

```
cli-frontend [nombre] [OPCIONES]

Argumentos:
  <nombre>              El nombre del template a generar

Opciones:
  -t, --type <TIPO>     El tipo de template a generar (detección automática de templates disponibles)
  --no-folder           Generar archivos sin crear una carpeta
  -c, --config <CONFIG> Ruta a archivo de configuración personalizado
  --help                Mostrar este mensaje de ayuda
```

## 🎯 Templates Disponibles

> ✨ **Detección Automática**: El CLI detecta automáticamente todos los templates disponibles en tu directorio de templates. No necesitas recompilar para agregar nuevos templates.

### 📦 Component
Genera un componente React completo con:
- `ComponentName.tsx` - Componente principal con TypeScript
- `ComponentName.module.scss` - Estilos SCSS modulares
- `ComponentName.spec.tsx` - Archivo de testing

### 🪝 Hook
Genera un hook personalizado:
- `useHookName.ts` - Hook principal
- `useHookName.test.ts` - Tests del hook

### 🔧 Service
Genera un servicio para lógica de negocio:
- `ServiceName.ts` - Clase o funciones del servicio

### 🌐 Context
Genera un contexto React completo:
- `ContextName.tsx` - Definición del contexto
- `ContextNameProvider.tsx` - Proveedor del contexto

### 📄 Page
Genera una página completa:
- `PageName.tsx` - Componente de la página
- `PageName.module.scss` - Estilos de la página
- `PageName.spec.tsx` - Tests de la página

### 🏪 Store (Redux)
Genera un store Redux completo:
- `StoreName.store.ts` - Slice con acciones y reducers
- `StoreName.types.ts` - Interfaces TypeScript
- `StoreName.thunks.ts` - Acciones asíncronas
- `StoreName.store.test.ts` - Tests completos

### 🌐 API
Genera un servicio de API:
- `ApiName.api.ts` - Clase con métodos CRUD y tipos

### 🔧 Templates Personalizados
¡Puedes crear tus propios templates! Solo agrega una carpeta en el directorio `templates/` y el CLI la detectará automáticamente.

```bash
# Ver todos los templates disponibles
cli-frontend --help
```

## ⚙️ Configuración

El CLI busca un archivo de configuración en este orden:

1. `.cli-frontend.conf` en el directorio actual
2. `~/.cli-frontend.conf` en el directorio home
3. Archivo especificado con `--config`

### Ejemplo de configuración

```ini
# CLI Frontend Generator Configuration

# Configuración general
default_type=component
create_folder=true
enable_hooks=true

# Configuración de rutas
templates_dir=./templates
output_dir=./src
```

## 🏗️ Estructura de Templates

> 🚀 **Templates Dinámicos**: El CLI detecta automáticamente cualquier carpeta en `templates/`. ¡Solo agrega una nueva carpeta y estará disponible inmediatamente!

Los templates se almacenan en el directorio `templates/` y utilizan el sistema de plantillas Handlebars:

```
templates/
├── component/
│   ├── $FILE_NAME.tsx
│   ├── $FILE_NAME.module.scss
│   └── $FILE_NAME.spec.tsx
├── hook/
│   ├── use$FILE_NAME.ts
│   └── use$FILE_NAME.test.ts
├── service/
│   └── $FILE_NAME.ts
├── context/
│   ├── $FILE_NAMEContext.tsx
│   └── $FILE_NAMEProvider.tsx
├── page/
│   ├── $FILE_NAMEPage.tsx
│   ├── $FILE_NAMEPage.module.scss
│   └── $FILE_NAMEPage.spec.tsx
├── store/
│   ├── $FILE_NAME.store.ts
│   ├── $FILE_NAME.types.ts
│   ├── $FILE_NAME.thunks.ts
│   └── $FILE_NAME.store.test.ts
├── api/
│   └── $FILE_NAME.api.ts
└── tu-template-personalizado/
    └── archivo-personalizado.ext
```

> 💡 **Tip**: Para agregar un nuevo template, simplemente crea una carpeta en `templates/` con tus archivos template. ¡El CLI lo detectará automáticamente!

### Variables disponibles en templates

- `$FILE_NAME` - Nombre original
- `{{pascal_name}}` - PascalCase (ej: `MyComponent`)
- `{{camel_name}}` - camelCase (ej: `myComponent`)
- `{{snake_name}}` - snake_case (ej: `my_component`)
- `{{kebab_name}}` - kebab-case (ej: `my-component`)
- `{{upper_name}}` - MAYÚSCULAS (ej: `MY_COMPONENT`)
- `{{hook_name}}` - Nombre inteligente para hooks (agrega `use` si es necesario)
- `{{context_name}}` - Nombre inteligente para contextos (agrega `Context` si es necesario)
- `{{provider_name}}` - Nombre inteligente para proveedores
- `{{page_name}}` - Nombre inteligente para páginas (agrega `Page` si es necesario)

## 🔧 Tecnologías Utilizadas

- **Rust** - Lenguaje principal
- **Clap** - Parsing de argumentos CLI
- **Handlebars** - Motor de plantillas
- **Tokio** - Runtime asíncrono
- **Serde** - Serialización/deserialización
- **Anyhow** - Manejo de errores
- **Colored** - Output colorizado

## 📁 Estructura del Proyecto

```
cli-frontend-rust/
├── src/
│   ├── main.rs              # Punto de entrada principal
│   ├── cli.rs               # Definición de argumentos CLI con detección dinámica
│   ├── config.rs            # Manejo de configuración
│   ├── template_engine.rs   # Motor de templates con Handlebars
│   └── error.rs             # Manejo de errores
├── templates/               # Templates detectados automáticamente
│   ├── component/           # Template para componentes React
│   ├── hook/                # Template para hooks personalizados
│   ├── service/             # Template para servicios
│   ├── context/             # Template para contextos React
│   ├── page/                # Template para páginas
│   ├── store/               # Template para stores Redux
│   ├── api/                 # Template para servicios API
│   └── [nuevo-template]/    # ¡Agrega tu propio template aquí!
├── Cargo.toml              # Configuración de Rust
├── README.md               # Documentación principal
└── TEMPLATE_GUIDE.md       # Guía para crear templates personalizados
```

## 🤝 Contribuciones

Las contribuciones son bienvenidas! Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## 📝 Roadmap

- [x] ✅ **Detección dinámica de templates** - Implementado
- [x] ✅ **Sistema de templates extensible** - Implementado  
- [x] ✅ **Templates para Redux/API** - Implementado
- [ ] Soporte para más frameworks (Vue, Angular)
- [ ] Validación de templates con schemas
- [ ] Integración con VS Code
- [ ] Plugin para diferentes editores
- [ ] Generación de tests automáticos más avanzados
- [ ] Soporte para Storybook
- [ ] Documentación automática
- [ ] Hot reload de templates en desarrollo
- [ ] Templates con configuración condicional

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Ver el archivo `LICENSE` para más detalles.

## � Creación de Templates Personalizados

Una de las características más poderosas del CLI es la **detección automática de templates**. Puedes crear tus propios templates sin modificar el código fuente:

### Pasos para crear un template personalizado:

1. **Crea una carpeta** en `templates/` con el nombre de tu template:
   ```bash
   mkdir templates/mi-template
   ```

2. **Agrega archivos template** usando variables de Handlebars:
   ```typescript
   // templates/mi-template/$FILE_NAME.ts
   export class {{pascal_name}} {
     // Tu código aquí
   }
   ```

3. **¡Úsalo inmediatamente!** No necesitas recompilar:
   ```bash
   cli-frontend MiElemento --type mi-template
   ```

### Variables disponibles:
- `$FILE_NAME` y `{{name}}` - Nombre original
- `{{pascal_name}}` - PascalCase
- `{{camel_name}}` - camelCase  
- `{{kebab_name}}` - kebab-case
- `{{snake_name}}` - snake_case
- `{{upper_name}}` - MAYÚSCULAS

Para más detalles, consulta el [TEMPLATE_GUIDE.md](./TEMPLATE_GUIDE.md).

## �👨‍💻 Autor

**Franco Castro** - [GitHub](https://github.com/FrancoCastro1990)

## 🙏 Agradecimientos

- A la comunidad de Rust por las excelentes herramientas
- A todos los desarrolladores que contribuyen al ecosistema frontend
- A los usuarios que proporcionan feedback y mejoras

---

⭐ Si este proyecto te resulta útil, ¡no olvides darle una estrella!