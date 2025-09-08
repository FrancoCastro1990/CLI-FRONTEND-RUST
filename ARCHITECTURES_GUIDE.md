# 🏗️ Guía de Arquitecturas de Software

Esta guía documenta las **11 arquitecturas de software** disponibles en el CLI Frontend Generator. Cada arquitectura está diseñada para diferentes escenarios y necesidades de desarrollo.

## 📖 Índice

1. [¿Qué son las Arquitecturas de Features?](#qué-son-las-arquitecturas-de-features)
2. [Cómo Usar las Arquitecturas](#cómo-usar-las-arquitecturas)
3. [Arquitecturas Disponibles](#arquitecturas-disponibles)
   - [Screaming Architecture](#screaming-architecture-por-defecto)
   - [MVC (Model-View-Controller)](#mvc-model-view-controller)
   - [MVP (Model-View-Presenter)](#mvp-model-view-presenter)
   - [MVVM (Model-View-ViewModel)](#mvvm-model-view-viewmodel)
   - [Flux Architecture](#flux-architecture)
   - [Redux Architecture](#redux-architecture)
   - [Clean Architecture](#clean-architecture)
   - [Component-Based Architecture](#component-based-architecture)
   - [Atomic Design](#atomic-design)
   - [Micro-Frontends](#micro-frontends)
   - [Event-Driven Architecture](#event-driven-architecture)
4. [¿Qué Arquitectura Elegir?](#qué-arquitectura-elegir)
5. [Crear Arquitecturas Personalizadas](#crear-arquitecturas-personalizadas)

---

## ¿Qué son las Arquitecturas de Features?

Las **arquitecturas de features** permiten generar estructuras completas de código que siguen patrones arquitectónicos específicos. En lugar de generar archivos individuales, generas una **feature completa** con:

- 📁 **Estructura de carpetas** organizada según el patrón elegido
- 📄 **Múltiples archivos** trabajando en conjunto
- 🔗 **Separación de responsabilidades** clara
- 📝 **Documentación** sobre beneficios y limitaciones

---

## Cómo Usar las Arquitecturas

### Sintaxis Básica
```bash
cli-frontend [NombreFeature] --type feature --architecture [nombre-arquitectura]
```

### Ejemplos
```bash
# Arquitectura por defecto (Screaming Architecture)
cli-frontend UserAuth --type feature

# Especificar arquitectura
cli-frontend UserAuth --type feature --architecture mvc
cli-frontend ShoppingCart --type feature --architecture atomic-design
cli-frontend PaymentSystem --type feature --architecture clean-architecture
```

### Ver Arquitecturas Disponibles
```bash
cli-frontend --help
```

---

## Arquitecturas Disponibles

### Screaming Architecture (Por Defecto)

**📐 Filosofía:** "La arquitectura debe gritar el propósito del sistema"

**🎯 Cuándo usar:**
- Equipos grandes que trabajan en diferentes features
- Aplicaciones que crecerán significativamente
- Cuando quieres que el código "grite" qué hace el negocio
- Proyectos donde las features son independientes

**✅ Beneficios:**
- Muy claro para grandes equipos
- Grita el negocio
- Fácil escalabilidad
- Estructura que refleja el dominio

**⚠️ Limitaciones:**
- Puede generar duplicación si no hay shared modules
- Requiere disciplina para naming y convenciones

**📁 Estructura Generada:**
```bash
cli-frontend UserAuth --type feature
```
```
UserAuth/
├── components/
│   ├── UserAuth.tsx
│   ├── UserAuth.spec.tsx
│   └── UserAuth.module.scss
├── pages/
│   ├── UserAuthPage.tsx
│   ├── UserAuthPage.spec.tsx
│   └── UserAuthPage.module.scss
├── hooks/
│   ├── useUserAuth.ts
│   └── useUserAuth.test.ts
└── types.ts
```

---

### MVC (Model-View-Controller)

**📐 Filosofía:** Separación clásica en tres capas: datos, vista y lógica

**🎯 Cuándo usar:**
- Aplicaciones tradicionales donde la lógica de negocio es compleja
- Equipos familiares con patrones MVC
- Aplicaciones con mucha interacción de formularios
- Sistemas CRUD tradicionales

**✅ Beneficios:**
- Claridad en separación de responsabilidades
- Fácil de entender para principiantes
- Patrón ampliamente conocido

**⚠️ Limitaciones:**
- En apps modernas con mucho estado dinámico puede volverse rígido
- Acoplamiento entre controlador y vista

**📁 Estructura Generada:**
```bash
cli-frontend UserManagement --type feature --architecture mvc
```
```
UserManagement/
├── models/
│   └── UserManagementModel.ts      # Lógica de datos
├── views/
│   ├── UserManagementView.tsx      # Interfaz de usuario
│   ├── UserManagementView.spec.tsx
│   └── UserManagementView.module.scss
├── controllers/
│   └── UserManagementController.ts # Lógica de negocio
└── types.ts                        # Interfaces
```

---

### MVP (Model-View-Presenter)

**📐 Filosofía:** El Presenter actúa como intermediario entre Vista y Modelo

**🎯 Cuándo usar:**
- Cuando necesitas alta testabilidad
- Vistas muy complejas que requieren mucha lógica de presentación
- Aplicaciones donde la lógica de UI es independiente de los datos

**✅ Beneficios:**
- Testable
- Desacopla la vista de la lógica de negocio
- Facilita testing unitario

**⚠️ Limitaciones:**
- Más boilerplate
- Puede volverse verboso en proyectos grandes

**📁 Estructura Generada:**
```bash
cli-frontend ProductCatalog --type feature --architecture mvp
```
```
ProductCatalog/
├── models/
│   └── ProductCatalogModel.ts      # Modelo de datos
├── views/
│   ├── ProductCatalogView.tsx      # Vista pasiva
│   ├── ProductCatalogView.spec.tsx
│   └── ProductCatalogView.module.scss
├── presenters/
│   └── ProductCatalogPresenter.ts  # Lógica de presentación
└── types.ts
```

---

### MVVM (Model-View-ViewModel)

**📐 Filosofía:** Data binding bidireccional entre Vista y ViewModel

**🎯 Cuándo usar:**
- Aplicaciones React/Vue/Angular con mucho estado reactivo
- Formularios complejos con validación en tiempo real
- Dashboards interactivos
- Aplicaciones con mucho data binding

**✅ Beneficios:**
- Facilita data binding
- Ideal para frameworks reactivos (React, Vue, Angular)
- Separación clara de responsabilidades

**⚠️ Limitaciones:**
- Puede generar dependencias implícitas si se abusa del binding
- Más complejo de depurar

**📁 Estructura Generada:**
```bash
cli-frontend Dashboard --type feature --architecture mvvm
```
```
Dashboard/
├── models/
│   └── DashboardModel.ts           # Modelo de datos
├── views/
│   ├── DashboardView.tsx           # Vista reactiva
│   ├── DashboardView.spec.tsx
│   └── DashboardView.module.scss
├── viewmodels/
│   └── DashboardViewModel.ts       # ViewModel con estado
├── hooks/
│   ├── useDashboard.ts             # Hook para binding
│   └── useDashboard.test.ts
└── types.ts
```

---

### Flux Architecture

**📐 Filosofía:** Flujo de datos unidireccional con Actions, Dispatcher y Stores

**🎯 Cuándo usar:**
- Aplicaciones con estado complejo y muchas interacciones
- Cuando necesitas flujo de datos predecible
- Aplicaciones que manejan muchos eventos de usuario
- Sistemas en tiempo real

**✅ Beneficios:**
- Control del estado global
- Predictibilidad
- Flujo de datos unidireccional
- Herramientas de debugging

**⚠️ Limitaciones:**
- Puede ser excesivo para apps pequeñas
- Boilerplate y curva de aprendizaje

**📁 Estructura Generada:**
```bash
cli-frontend ChatApp --type feature --architecture flux
```
```
ChatApp/
├── actions/
│   └── ChatAppActions.ts           # Actions para modificar estado
├── stores/
│   ├── ChatAppStore.ts             # Store que mantiene estado
│   ├── ChatAppStore.types.ts
│   ├── ChatAppStore.thunks.ts
│   └── ChatAppStore.store.test.ts
├── views/
│   ├── ChatAppView.tsx             # Vista que consume store
│   ├── ChatAppView.spec.tsx
│   └── ChatAppView.module.scss
├── hooks/
│   ├── useChatApp.ts               # Hook para acceder al store
│   └── useChatApp.test.ts
└── types.ts
```

---

### Redux Architecture

**📐 Filosofía:** Estado predecible con reducers, actions y estado inmutable

**🎯 Cuándo usar:**
- Aplicaciones grandes con estado complejo
- Cuando necesitas time-travel debugging
- Aplicaciones que requieren persistencia de estado
- Equipos grandes que necesitan patrones consistentes

**✅ Beneficios:**
- Estado predecible
- Time-travel debugging
- Middleware ecosystem
- DevTools excelentes

**⚠️ Limitaciones:**
- Boilerplate extenso
- Curva de aprendizaje pronunciada

**📁 Estructura Generada:**
```bash
cli-frontend ShoppingCart --type feature --architecture redux
```
```
ShoppingCart/
├── store/
│   ├── ShoppingCartStore.ts        # Store principal
│   ├── ShoppingCartStore.types.ts
│   ├── ShoppingCartStore.thunks.ts
│   └── ShoppingCartStore.store.test.ts
├── actions/
│   └── ShoppingCartActions.ts      # Action creators
├── reducers/
│   └── ShoppingCartReducer.ts      # Reducers para estado
├── components/
│   ├── ShoppingCart.tsx            # Componente conectado
│   ├── ShoppingCart.spec.tsx
│   └── ShoppingCart.module.scss
├── hooks/
│   ├── useShoppingCart.ts          # useSelector/useDispatch
│   └── useShoppingCart.test.ts
└── types.ts
```

---

### Clean Architecture

**📐 Filosofía:** Arquitectura por capas con dependencias hacia adentro

**🎯 Cuándo usar:**
- Aplicaciones empresariales complejas
- Sistemas que deben ser altamente testeable
- Aplicaciones que cambiarán frameworks/tecnologías
- Proyectos a largo plazo con múltiples equipos

**✅ Beneficios:**
- Alta mantenibilidad
- Altamente testeable
- Desacoplada
- Independiente de frameworks

**⚠️ Limitaciones:**
- Puede ser demasiado estructurada para apps simples
- Más código inicial
- Curva de aprendizaje

**📁 Estructura Generada:**
```bash
cli-frontend PaymentSystem --type feature --architecture clean-architecture
```
```
PaymentSystem/
├── domain/
│   ├── entities/
│   │   └── PaymentSystemEntity.ts      # Entidades del dominio
│   └── repositories/
│       └── PaymentSystemRepository.ts  # Interfaces de repos
├── application/
│   └── usecases/
│       └── PaymentSystemUseCase.ts     # Casos de uso
├── infrastructure/
│   └── repositories/
│       └── PaymentSystemRepositoryImpl.ts # Implementaciones
├── presentation/
│   ├── components/
│   │   ├── PaymentSystem.tsx           # Componentes UI
│   │   ├── PaymentSystem.spec.tsx
│   │   └── PaymentSystem.module.scss
│   └── hooks/
│       ├── usePaymentSystem.ts         # Hooks de presentación
│       └── usePaymentSystem.test.ts
└── types.ts
```

---

### Component-Based Architecture

**📐 Filosofía:** Organización por componentes reutilizables y su jerarquía

**🎯 Cuándo usar:**
- Aplicaciones con muchos componentes reutilizables
- Design systems
- Aplicaciones que priorizan la reutilización
- Equipos enfocados en UI/UX

**✅ Beneficios:**
- Escalable
- Reutilizable
- Fácil de testear y mantener
- Encapsulación clara

**⚠️ Limitaciones:**
- Si no hay convenciones, puede volverse desordenada
- Acoplamiento implícito si los props/estado no se manejan bien

**📁 Estructura Generada:**
```bash
cli-frontend UILibrary --type feature --architecture component-based
```
```
UILibrary/
├── components/
│   ├── common/
│   │   ├── UILibrary.tsx               # Componentes reutilizables
│   │   ├── UILibrary.spec.tsx
│   │   └── UILibrary.module.scss
│   ├── containers/
│   │   ├── UILibraryContainer.tsx      # Componentes con lógica
│   │   ├── UILibraryContainer.spec.tsx
│   │   └── UILibraryContainer.module.scss
│   └── presentation/
│       ├── UILibraryPresentation.tsx   # Componentes puros
│       ├── UILibraryPresentation.spec.tsx
│       └── UILibraryPresentation.module.scss
├── hooks/
│   ├── useUILibrary.ts                 # Hooks personalizados
│   └── useUILibrary.test.ts
└── types.ts
```

---

### Atomic Design

**📐 Filosofía:** Jerarquía de componentes: Átomos → Moléculas → Organismos → Plantillas → Páginas

**🎯 Cuándo usar:**
- Design systems complejos
- Aplicaciones que requieren consistencia visual
- Equipos de diseño y desarrollo trabajando juntos
- Sistemas con muchos componentes reutilizables

**✅ Beneficios:**
- Facilita diseño consistente
- Modularización UI clara
- Reutilización sistemática
- Design System natural

**⚠️ Limitaciones:**
- No cubre lógica de negocio
- Requiere disciplina para no mezclar niveles

**📁 Estructura Generada:**
```bash
cli-frontend DesignSystem --type feature --architecture atomic-design
```
```
DesignSystem/
├── atoms/
│   ├── DesignSystemAtom.tsx        # Elementos básicos (botones, inputs)
│   ├── DesignSystemAtom.spec.tsx
│   └── DesignSystemAtom.module.scss
├── molecules/
│   ├── DesignSystemMolecule.tsx    # Combinación de átomos
│   ├── DesignSystemMolecule.spec.tsx
│   └── DesignSystemMolecule.module.scss
├── organisms/
│   ├── DesignSystemOrganism.tsx    # Secciones complejas
│   ├── DesignSystemOrganism.spec.tsx
│   └── DesignSystemOrganism.module.scss
├── templates/
│   ├── DesignSystemTemplate.tsx    # Layout sin contenido
│   ├── DesignSystemTemplate.spec.tsx
│   └── DesignSystemTemplate.module.scss
├── pages/
│   ├── DesignSystemPage.tsx        # Páginas completas
│   ├── DesignSystemPage.spec.tsx
│   └── DesignSystemPage.module.scss
└── types.ts
```

---

### Micro-Frontends

**📐 Filosofía:** Cada feature es un "mini-frontend" independiente

**🎯 Cuándo usar:**
- Equipos grandes trabajando en paralelo
- Aplicaciones que necesitan despliegues independientes
- Migración gradual de sistemas legacy
- Diferentes tecnologías por equipo

**✅ Beneficios:**
- Escalabilidad de equipos grandes
- Despliegues independientes
- Tecnologías independientes
- Isolación de fallos

**⚠️ Limitaciones:**
- Complejidad de integración
- Duplicación de dependencias
- Más infraestructura

**📁 Estructura Generada:**
```bash
cli-frontend OrderModule --type feature --architecture micro-frontends
```
```
OrderModule/
├── shell/
│   ├── OrderModuleShell.tsx        # Container del microfrontend
│   ├── OrderModuleShell.spec.tsx
│   └── OrderModuleShell.module.scss
├── components/
│   ├── OrderModule.tsx             # Componentes internos
│   ├── OrderModule.spec.tsx
│   └── OrderModule.module.scss
├── services/
│   └── OrderModuleService.ts       # Servicios del módulo
├── hooks/
│   ├── useOrderModule.ts           # Hooks del módulo
│   └── useOrderModule.test.ts
├── api/
│   └── OrderModuleApi.ts           # API del módulo
└── types.ts                        # Tipos públicos
```

---

### Event-Driven Architecture

**📐 Filosofía:** Comunicación entre features mediante eventos

**🎯 Cuándo usar:**
- Aplicaciones altamente interactivas
- Sistemas en tiempo real
- Aplicaciones con muchos módulos independientes
- Chat apps, juegos, dashboards en vivo

**✅ Beneficios:**
- Bajo acoplamiento
- Flexible
- Ideal para apps altamente interactivas
- Escalabilidad horizontal

**⚠️ Limitaciones:**
- Difícil de depurar
- Seguimiento de flujo de eventos complejo

**📁 Estructura Generada:**
```bash
cli-frontend NotificationSystem --type feature --architecture event-driven
```
```
NotificationSystem/
├── events/
│   └── NotificationSystemEvents.ts    # Definición de eventos
├── listeners/
│   └── NotificationSystemListeners.ts # Listeners de eventos
├── publishers/
│   └── NotificationSystemPublisher.ts # Publicadores de eventos
├── components/
│   ├── NotificationSystem.tsx         # Componentes que consumen eventos
│   ├── NotificationSystem.spec.tsx
│   └── NotificationSystem.module.scss
├── hooks/
│   ├── useNotificationSystemEvents.ts # Hooks para eventos
│   └── useNotificationSystemEvents.test.ts
└── types.ts                           # Tipos de eventos y payloads
```

---

## ¿Qué Arquitectura Elegir?

### 🎯 Para Proyectos Pequeños/Medianos (< 10 desarrolladores)
- **Screaming Architecture** - Para features bien definidas
- **Component-Based** - Para aplicaciones UI-heavy
- **MVVM** - Para formularios complejos

### 🏢 Para Proyectos Empresariales (> 10 desarrolladores)
- **Clean Architecture** - Para sistemas complejos a largo plazo
- **Micro-Frontends** - Para equipos grandes independientes
- **MVC/MVP** - Para sistemas tradicionales

### 🎨 Para Design Systems
- **Atomic Design** - Para bibliotecas de componentes
- **Component-Based** - Para sistemas de componentes

### ⚡ Para Aplicaciones Interactivas
- **Event-Driven** - Para sistemas en tiempo real
- **Flux/Redux** - Para estado complejo

### 📊 Tabla de Decisión

| Escenario | Arquitectura Recomendada | Alternativa |
|-----------|-------------------------|-------------|
| App pequeña con features claras | Screaming | Component-Based |
| Sistema empresarial complejo | Clean Architecture | MVC |
| Design System | Atomic Design | Component-Based |
| App con estado complejo | Redux | Flux |
| Equipos grandes independientes | Micro-Frontends | Clean Architecture |
| App en tiempo real | Event-Driven | Flux |
| Formularios complejos | MVVM | MVP |
| Sistema CRUD tradicional | MVC | MVP |

---

## Crear Arquitecturas Personalizadas

Puedes crear tus propias arquitecturas agregando archivos JSON en el directorio `architectures/`:

### 1. Crear archivo JSON
```bash
# Crear nueva arquitectura
nano architectures/mi-arquitectura.json
```

### 2. Definir estructura
```json
{
  "name": "Mi Arquitectura Personalizada",
  "description": "Descripción de mi arquitectura",
  "benefits": [
    "Beneficio 1",
    "Beneficio 2"
  ],
  "limitations": [
    "Limitación 1",
    "Limitación 2"
  ],
  "structure": [
    {
      "path": "carpeta1",
      "template": "component",
      "filename_pattern": "{name}Custom",
      "description": "Descripción de esta parte"
    },
    {
      "path": "carpeta2", 
      "template": "service",
      "filename_pattern": "{name}Service",
      "description": "Descripción del servicio"
    }
  ]
}
```

### 3. Usar tu arquitectura
```bash
cli-frontend MiFeature --type feature --architecture mi-arquitectura
```

### Variables disponibles en `filename_pattern`:
- `{name}` - Nombre original
- `use{name}` - Para hooks (agrega 'use' automáticamente)
- `{name}Context` - Para contextos
- `{name}Provider` - Para proveedores  
- `{name}Page` - Para páginas

### Templates disponibles para reutilizar:
- `component` - Componente React
- `hook` - Hook personalizado
- `service` - Servicio/clase
- `context` - Contexto React
- `page` - Página
- `store` - Store Redux
- `api` - Servicio API
- `types` - Archivo de tipos

---

## 🤝 Contribuir

¿Tienes ideas para nuevas arquitecturas? ¡Las contribuciones son bienvenidas!

1. Crea una nueva arquitectura JSON
2. Documenta beneficios y limitaciones
3. Añade ejemplos de uso
4. Haz un Pull Request

---

## 📚 Referencias

- [Clean Architecture - Uncle Bob](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Flux Architecture](https://facebook.github.io/flux/)
- [Atomic Design - Brad Frost](https://atomicdesign.bradfrost.com/)
- [Screaming Architecture](https://blog.cleancoder.com/uncle-bob/2011/09/30/Screaming-Architecture.html)
- [Micro-Frontends](https://micro-frontends.org/)

---

**💡 ¿Necesitas ayuda eligiendo una arquitectura?** Considera el tamaño de tu equipo, la complejidad del proyecto y los requisitos futuros. Cuando tengas dudas, **Screaming Architecture** es una excelente opción por defecto.