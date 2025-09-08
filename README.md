# CLI Fr- 🎯 **Generación automática** de componentes React con TypeScript
- 🪝 **Hooks personalizados** con patrones optimizados
- 🔧 **Servicios** para lógica de negocio
- 🌐 **Contextos** con proveedores automáticos
- 📄 **Páginas** con estructura completa
- 🏗️ **Features completos** con 11 arquitecturas de software configurables
- 📐 **Patrones arquitectónicos** (MVC, MVP, MVVM, Clean Architecture, Atomic Design y más)
- 🎨 **SCSS Modules** incluidos por defecto
- 🧪 **Archivos de testing** (.spec.tsx) automáticos
- ⚙️ **Configuración automática** - Los instaladores crean la configuración por ti
- 📝 **Múltiples convenciones de nomenclatura** (PascalCase, camelCase, snake_case, kebab-case)
- 🔄 **Sistema de templates** avanzado con Handlebars
- ✨ **Detección automática de templates y arquitecturas** - No requiere recompilación
- 🚀 **Templates dinámicos** - El CLI descubre automáticamente carpetas de templates
- 📚 **Arquitecturas documentadas** con beneficios y limitaciones
- 💾 **Instaladores inteligentes** - Configuración automática según la plataformar 🚀

Un potente generador CLI escrito en Rust para crear componentes React, hooks, servicios, contextos, páginas y **features completos con múltiples arquitecturas de software** con soporte completo para TypeScript, SCSS modules y plantillas de testing.

> 🎉 **Versión 1.2.2** - Ahora con configuración automática y instaladores mejorados

## ✨ Características

- 🎯 **Generación automática** de componentes React con TypeScript
- 🪝 **Hooks personalizados** con patrones optimizados
- 🔧 **Servicios** para lógica de negocio
- 🌐 **Contextos** con proveedores automáticos
- 📄 **Páginas** con estructura completa
- �️ **Features completos** con 11 arquitecturas de software configurables
- 📐 **Patrones arquitectónicos** (MVC, MVP, MVVM, Clean Architecture, Atomic Design y más)
- �🎨 **SCSS Modules** incluidos por defecto
- 🧪 **Archivos de testing** (.spec.tsx) automáticos
- ⚙️ **Configuración personalizable** mediante archivos .conf
- 📝 **Múltiples convenciones de nomenclatura** (PascalCase, camelCase, snake_case, kebab-case)
- 🔄 **Sistema de templates** avanzado con Handlebars
- ✨ **Detección automática de templates y arquitecturas** - No requiere recompilación
- 🚀 **Templates dinámicos** - El CLI descubre automáticamente carpetas de templates
- 📚 **Arquitecturas documentadas** con beneficios y limitaciones

## � Instalación

### Instalación rápida (Recomendada)

**Linux/macOS:**
```bash
curl -sSL https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install-quick.sh | bash
```

**Windows (PowerShell):**
```powershell
iwr -useb https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install-quick.ps1 | iex
```

> ℹ️ **Los instaladores v1.2.2** crean automáticamente la configuración necesaria. Descargan binarios precompilados cuando están disponibles, o compilan desde código fuente automáticamente.

> 🔧 **Configuración automática** - Los instaladores detectan la ubicación de instalación y crean los archivos de configuración automáticamente, ¡no requieres configuración manual!

### Instalación desde código fuente

**Linux/macOS:**
```bash
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust
chmod +x install.sh
./install.sh  # Compila automáticamente si no encuentra el binario
```

**Windows (PowerShell):**
```powershell
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust
./install.ps1  # Compila automáticamente si no encuentra el binario
```

> 💡 **Los instaladores v1.2.2** compilan automáticamente el proyecto si no encuentran el binario precompilado, y crean la configuración necesaria automáticamente.

```bash
# Clona el repositorio
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust

# Compila en modo release
cargo build --release

# El binario estará en target/release/cli-frontend.exe (Windows)
```

## 🚀 Uso Rápido

### Templates Básicos
```bash
# Ver templates y arquitecturas disponibles
cli-frontend --help

# Generar componente en el directorio actual
cli-frontend Button --type component

# Especificar directorio de salida
cli-frontend Header --type component --output-dir ./src/components

# Generar un hook
cli-frontend useAuth --type hook

# Generar un servicio
cli-frontend ApiService --type service

# Generar sin crear carpeta
cli-frontend Modal --type component --no-folder
```

### 🏗️ Features con Arquitecturas (¡NUEVO!)
```bash
# Generar feature con arquitectura por defecto (Screaming Architecture)
cli-frontend MyAuth --type feature

# Generar feature con arquitectura MVC
cli-frontend UserManagement --type feature --architecture mvc

# Generar feature con Atomic Design
cli-frontend ShoppingCart --type feature --architecture atomic-design

# Generar feature con Clean Architecture
cli-frontend PaymentSystem --type feature --architecture clean-architecture

# Ver todas las arquitecturas disponibles
cli-frontend --help
```

### Arquitecturas Disponibles
- **screaming-architecture** (por defecto) - Organización por features/dominios
- **mvc** - Model-View-Controller tradicional
- **mvp** - Model-View-Presenter
- **mvvm** - Model-View-ViewModel
- **flux** - Arquitectura Flux unidireccional
- **redux** - Patrón Redux con reducers
- **clean-architecture** - Clean Architecture con capas
- **component-based** - Arquitectura basada en componentes
- **atomic-design** - Atomic Design (átomos, moléculas, organismos)
- **micro-frontends** - Arquitectura de micro-frontends
- **event-driven** - Arquitectura dirigida por eventos

> 📖 Para más detalles sobre cada arquitectura, consulta [ARCHITECTURES_GUIDE.md](./ARCHITECTURES_GUIDE.md)

## 📋 Comandos Disponibles

```
cli-frontend [nombre] [OPCIONES]

Argumentos:
  <nombre>                    El nombre del template o feature a generar

Opciones:
  -t, --type <TIPO>           El tipo de template a generar (detección automática)
  -a, --architecture <ARCH>   Patrón de arquitectura para features (ej: mvc, atomic-design)
  --no-folder                 Generar archivos sin crear una carpeta
  -o, --output-dir <DIR>      Directorio de salida para archivos generados
  -c, --config <CONFIG>       Ruta a archivo de configuración personalizado
  --help                      Mostrar ayuda, templates y arquitecturas disponibles
```

## 🎯 Templates y Features Disponibles

> ✨ **Detección Automática**: El CLI detecta automáticamente todos los templates y arquitecturas disponibles. No necesitas recompilar para agregar nuevos templates.

### 🏗️ Features con Arquitecturas (¡NUEVO!)

El sistema de **Features** permite generar estructuras completas de código siguiendo patrones arquitectónicos específicos. Cada feature genera múltiples archivos organizados según la arquitectura elegida.

**Ejemplo - Feature con Screaming Architecture:**
```bash
cli-frontend MyAuth --type feature
```
Genera:
```
MyAuth/
├── components/
│   ├── MyAuth.tsx
│   ├── MyAuth.spec.tsx
│   └── MyAuth.module.scss
├── pages/
│   ├── MyAuthPage.tsx
│   ├── MyAuthPage.spec.tsx
│   └── MyAuthPage.module.scss
├── hooks/
│   ├── useMyAuth.ts
│   └── useMyAuth.test.ts
└── types.ts
```

**Ejemplo - Feature con MVC:**
```bash
cli-frontend UserManagement --type feature --architecture mvc
```
Genera:
```
UserManagement/
├── models/
│   └── UserManagementModel.ts
├── views/
│   ├── UserManagementView.tsx
│   ├── UserManagementView.spec.tsx
│   └── UserManagementView.module.scss
├── controllers/
│   └── UserManagementController.ts
└── types.ts
```

> 📖 **Consulta [ARCHITECTURES_GUIDE.md](./ARCHITECTURES_GUIDE.md)** para ver todas las arquitecturas disponibles, sus beneficios, limitaciones y cuándo usar cada una.

### 📦 Templates Básicos

Los siguientes templates están disponibles para generar elementos individuales:

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
architectures_dir=./architectures

# Features
default_architecture=screaming-architecture
```

## 🏗️ Estructura de Templates

> 🚀 **Templates Dinámicos**: El CLI detecta automáticamente cualquier carpeta en `templates/`. ¡Solo agrega una nueva carpeta y estará disponible inmediatamente!

Los templates se almacenan en el directorio `templates/` y las arquitecturas en `architectures/`, ambos utilizan el sistema de plantillas Handlebars:

```
templates/
├── component/           # Template para componentes React
│   ├── $FILE_NAME.tsx
│   ├── $FILE_NAME.module.scss
│   └── $FILE_NAME.spec.tsx
├── hook/                # Template para hooks personalizados
│   ├── use$FILE_NAME.ts
│   └── use$FILE_NAME.test.ts
├── service/             # Template para servicios
│   └── $FILE_NAME.ts
├── context/             # Template para contextos React
│   ├── $FILE_NAMEContext.tsx
│   └── $FILE_NAMEProvider.tsx
├── page/                # Template para páginas
│   ├── $FILE_NAMEPage.tsx
│   ├── $FILE_NAMEPage.module.scss
│   └── $FILE_NAMEPage.spec.tsx
├── store/               # Template para stores Redux
│   ├── $FILE_NAME.store.ts
│   ├── $FILE_NAME.types.ts
│   ├── $FILE_NAME.thunks.ts
│   └── $FILE_NAME.store.test.ts
├── api/                 # Template para servicios API
│   └── $FILE_NAME.api.ts
├── types/               # Template para archivos de tipos TypeScript
│   └── types.ts
└── [nuevo-template]/    # ¡Agrega tu propio template aquí!

architectures/           # 🏗️ Configuraciones de arquitecturas JSON
├── default.json         # Arquitectura por defecto (Screaming)
├── screaming-architecture.json # Features/dominios first
├── mvc.json            # Model-View-Controller
├── mvp.json            # Model-View-Presenter
├── mvvm.json           # Model-View-ViewModel
├── flux.json           # Arquitectura Flux unidireccional
├── redux.json          # Patrón Redux con reducers
├── clean-architecture.json # Clean Architecture por capas
├── component-based.json # Arquitectura basada en componentes
├── atomic-design.json  # Atomic Design (átomos → páginas)
├── micro-frontends.json # Micro-frontends independientes
└── event-driven.json   # Arquitectura dirigida por eventos
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
│   ├── config.rs            # Manejo de configuración y arquitecturas
│   ├── template_engine.rs   # Motor de templates con Handlebars y features
│   └── error.rs             # Manejo de errores
├── templates/               # Templates detectados automáticamente
│   ├── component/           # Template para componentes React
│   ├── hook/                # Template para hooks personalizados
│   ├── service/             # Template para servicios
│   ├── context/             # Template para contextos React
│   ├── page/                # Template para páginas
│   ├── store/               # Template para stores Redux
│   ├── api/                 # Template para servicios API
│   ├── types/               # Template para archivos de tipos
│   └── [nuevo-template]/    # ¡Agrega tu propio template aquí!
├── architectures/           # 🏗️ Configuraciones de arquitecturas (¡NUEVO!)
│   ├── default.json         # Arquitectura por defecto
│   ├── mvc.json            # Model-View-Controller
│   ├── atomic-design.json  # Atomic Design
│   └── [más arquitecturas]... # Ver ARCHITECTURES_GUIDE.md
├── Cargo.toml              # Configuración de Rust
├── README.md               # Documentación principal
├── ARCHITECTURES_GUIDE.md  # 📖 Guía completa de arquitecturas (¡NUEVO!)
└── TEMPLATE_GUIDE.md       # Guía para crear templates personalizados
```

## 🤝 Contribuciones

Las contribuciones son bienvenidas! Por favor:

1. Fork el proyecto
2. Crea una rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit tus cambios (`git commit -m 'Add some AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abre un Pull Request

## � Roadmap

- [x] ✅ **Detección dinámica de templates** - Implementado v1.0.0
- [x] ✅ **Sistema de templates extensible** - Implementado v1.0.0  
- [x] ✅ **Templates para Redux/API** - Implementado v1.1.0
- [x] ✅ **Features con arquitecturas configurables** - Implementado v1.2.0
- [x] ✅ **11 patrones arquitectónicos documentados** - Implementado v1.2.0
- [x] ✅ **Configuración automática en instaladores** - Implementado v1.2.2
- [x] ✅ **Instaladores inteligentes multiplataforma** - Implementado v1.2.2
- [ ] Soporte para más frameworks (Vue, Angular)
- [ ] Validación de templates con schemas
- [ ] Arquitecturas personalizadas definidas por usuario
- [ ] Integración con VS Code
- [ ] Plugin para diferentes editores
- [ ] Generación de tests automáticos más avanzados
- [ ] Soporte para Storybook
- [ ] Documentación automática
- [ ] Hot reload de templates en desarrollo
- [ ] Templates con configuración condicional
- [ ] Wizard interactivo para selección de arquitecturas

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