# 🏗️ Software Architecture Guide

This guide documents the **12 software architectures** available in the CLI Frontend Generator. Each architecture is designed for specific development scenarios and requirements, enabling professional developers to generate complete feature structures following established architectural patterns.

## 📖 Table of Contents

1. [What are Feature Architectures?](#what-are-feature-architectures)
2. [How to Use Architectures](#how-to-use-architectures)
3. [Available Architectures](#available-architectures)
   - [Screaming Architecture](#screaming-architecture-default)
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
4. [Architecture Selection Guide](#architecture-selection-guide)
5. [Creating Custom Architectures](#creating-custom-architectures)

---

## What are Feature Architectures?

**Feature architectures** enable the generation of complete code structures that follow specific architectural patterns. Instead of generating individual files, you generate a **complete feature** with:

- 📁 **Folder structure** organized according to the chosen pattern
- 📄 **Multiple files** working together cohesively
- 🔗 **Clear separation of concerns** and responsibilities
- 📝 **Technical documentation** covering benefits and limitations

---

## How to Use Architectures

### Basic Syntax
```bash
cli-frontend [FeatureName] --type feature --architecture [architecture-name]
```

### Examples
```bash
# Default architecture (Screaming Architecture)
cli-frontend UserAuth --type feature

# Specify architecture
cli-frontend UserAuth --type feature --architecture mvc
cli-frontend ShoppingCart --type feature --architecture atomic-design
cli-frontend PaymentSystem --type feature --architecture clean-architecture
```

### View Available Architectures
```bash
cli-frontend --help
```

---

## Available Architectures

### Screaming Architecture (Default)

**📐 Philosophy:** "Architecture should scream the system's purpose"

**🎯 When to use:**
- Large teams working on different features
- Applications with significant growth potential
- Projects where business domain should be immediately evident
- Feature-independent development scenarios

**✅ Benefits:**
- Excellent clarity for large development teams
- Domain-driven structure that reflects business needs
- Natural scalability and maintainability
- Self-documenting codebase organization

**⚠️ Limitations:**
- Potential code duplication without proper shared modules
- Requires disciplined naming conventions and team coordination

**📁 Generated Structure:**
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

**📐 Philosophy:** Classic three-layer separation: data, view, and business logic

**🎯 When to use:**
- Traditional applications with complex business logic
- Teams familiar with MVC architectural patterns
- Form-heavy applications with extensive user interactions
- Traditional CRUD systems with clear data flow

**✅ Benefits:**
- Clear separation of responsibilities across layers
- Beginner-friendly pattern with widespread recognition
- Well-established architectural pattern with proven results

**⚠️ Limitations:**
- Can become rigid in modern dynamic state applications
- Potential tight coupling between controller and view components

**📁 Generated Structure:**
```bash
cli-frontend UserManagement --type feature --architecture mvc
```
```
UserManagement/
├── models/
│   └── UserManagementModel.ts      # Data logic and business rules
├── views/
│   ├── UserManagementView.tsx      # User interface components
│   ├── UserManagementView.spec.tsx
│   └── UserManagementView.module.scss
├── controllers/
│   └── UserManagementController.ts # Business logic orchestration
└── types.ts                        # Interface definitions
```

---

### MVP (Model-View-Presenter)

**📐 Philosophy:** The Presenter acts as an intermediary between View and Model

**🎯 When to use:**
- Applications requiring high testability standards
- Complex views with extensive presentation logic requirements
- Applications where UI logic is independent from data sources

**✅ Benefits:**
- Highly testable architecture with isolated presentation logic
- Decouples view from business logic for better maintainability
- Facilitates comprehensive unit testing strategies

**⚠️ Limitations:**
- Increased boilerplate code for simple operations
- Can become verbose in large-scale projects

**📁 Generated Structure:**
```bash
cli-frontend ProductCatalog --type feature --architecture mvp
```
```
ProductCatalog/
├── models/
│   └── ProductCatalogModel.ts      # Data model definitions
├── views/
│   ├── ProductCatalogView.tsx      # Passive view components
│   ├── ProductCatalogView.spec.tsx
│   └── ProductCatalogView.module.scss
├── presenters/
│   └── ProductCatalogPresenter.ts  # Presentation logic coordination
└── types.ts
```

---

### MVVM (Model-View-ViewModel)

**📐 Philosophy:** Bidirectional data binding between View and ViewModel

**🎯 When to use:**
- React/Vue/Angular applications with reactive state management
- Complex forms with real-time validation requirements
- Interactive dashboards with dynamic data binding
- Applications prioritizing reactive data synchronization

**✅ Benefits:**
- Facilitates seamless data binding and state synchronization
- Optimal for reactive frameworks (React, Vue, Angular)
- Clear separation of concerns with reactive patterns

**⚠️ Limitations:**
- Can create implicit dependencies with excessive binding
- More complex debugging scenarios with reactive chains

**📁 Generated Structure:**
```bash
cli-frontend Dashboard --type feature --architecture mvvm
```
```
Dashboard/
├── models/
│   └── DashboardModel.ts           # Data model definitions
├── views/
│   ├── DashboardView.tsx           # Reactive view components
│   ├── DashboardView.spec.tsx
│   └── DashboardView.module.scss
├── viewmodels/
│   └── DashboardViewModel.ts       # ViewModel with reactive state
├── hooks/
│   ├── useDashboard.ts             # Data binding hooks
│   └── useDashboard.test.ts
└── types.ts
```

---

### Flux Architecture

**📐 Philosophy:** Unidirectional data flow with Actions, Dispatcher, and Stores

**🎯 When to use:**
- Applications with complex state and multiple user interactions
- Systems requiring predictable data flow patterns
- Real-time applications with event-driven architectures
- Applications handling numerous user events and state changes

**✅ Benefits:**
- Guaranteed unidirectional data flow with centralized Dispatcher coordination
- Eliminates circular dependencies and cascading update scenarios
- Predictable state mutations through sequential action processing
- Enhanced debugging with centralized action logging capabilities

**⚠️ Limitations:**
- Significant boilerplate overhead with Dispatcher, Actions, and Store setup
- Complex learning curve requiring understanding of unidirectional patterns
- May be excessive for simple state management requirements

**📁 Generated Structure:**
```bash
cli-frontend ChatApp --type feature --architecture flux
```
```
ChatApp/
├── actions/
│   └── ChatAppActions.ts           # Action creators for state mutations
├── dispatcher/
│   └── ChatAppDispatcher.ts        # Central dispatcher hub managing action distribution
├── stores/
│   ├── ChatAppStore.ts             # State containers with business logic
│   ├── ChatAppStore.types.ts
│   ├── ChatAppStore.thunks.ts
│   └── ChatAppStore.store.test.ts
├── views/
│   ├── ChatAppView.tsx             # React components consuming store state
│   ├── ChatAppView.spec.tsx
│   └── ChatAppView.module.scss
├── hooks/
│   ├── useChatApp.ts               # Custom hooks for store subscription
│   └── useChatApp.test.ts
└── types.ts
```

---

### Redux Architecture

**📐 Philosophy:** Predictable state management with reducers, actions, and immutable state

**🎯 When to use:**
- Large applications with complex state management needs
- Systems requiring time-travel debugging capabilities
- Applications needing state persistence and rehydration
- Large teams requiring consistent state management patterns

**✅ Benefits:**
- Predictable state updates through pure reducer functions
- Powerful time-travel debugging and state inspection capabilities
- Rich middleware ecosystem for async operations and side effects
- Excellent developer tooling with Redux DevTools integration

**⚠️ Limitations:**
- Extensive boilerplate code for simple state operations
- Steep learning curve with functional programming concepts
- Performance considerations with frequent state updates

**📁 Generated Structure:**
```bash
cli-frontend ShoppingCart --type feature --architecture redux
```
```
ShoppingCart/
├── store/
│   ├── ShoppingCartStore.ts        # Redux store with root reducer
│   ├── ShoppingCartStore.types.ts
│   ├── ShoppingCartStore.thunks.ts
│   └── ShoppingCartStore.store.test.ts
├── actions/
│   └── ShoppingCartActions.ts      # Action creators and type definitions
├── reducers/
│   └── ShoppingCartReducer.ts      # Pure reducer functions for state
├── selectors/
│   └── ShoppingCartSelectors.ts    # Memoized selector functions for data extraction
├── middleware/
│   └── ShoppingCartMiddleware.ts   # Custom middleware for async operations
├── constants/
│   └── ShoppingCartConstants.ts    # Action type constants and configuration
├── components/
│   ├── ShoppingCart.tsx            # Redux-connected React components
│   ├── ShoppingCart.spec.tsx
│   └── ShoppingCart.module.scss
├── hooks/
│   ├── useShoppingCart.ts          # Custom Redux hooks with useSelector/useDispatch
│   └── useShoppingCart.test.ts
└── types.ts
```

---

### Clean Architecture

**📐 Philosophy:** Simplified layered architecture with inward-facing dependencies

**🎯 When to use:**
- Enterprise applications with complex business requirements
- Systems requiring high testability and maintainability
- Applications that may change frameworks or technologies
- Long-term projects with multiple development teams

**✅ Benefits:**
- Clear separation between business logic and UI concerns
- Excellent testability with isolated domain and application layers
- Framework independence allowing easy technology migrations
- Maintainable codebase with well-defined layer responsibilities

**⚠️ Limitations:**
- Initial setup overhead for simple applications
- Requires understanding of layered architecture principles
- May introduce complexity for basic CRUD operations

**📁 Generated Structure:**
```bash
cli-frontend PaymentSystem --type feature --architecture clean-architecture
```
```
PaymentSystem/
├── domain/
│   ├── entities/
│   │   └── PaymentSystemEntity.ts      # Core business entities with rules
│   └── repositories/
│       └── PaymentSystemRepository.ts  # Abstract repository interfaces
├── application/
│   └── usecases/
│       └── PaymentSystemUseCase.ts     # Application use cases orchestration
├── infrastructure/
│   └── services/
│       └── PaymentSystemService.ts     # External service implementations
├── presentation/
│   ├── components/
│   │   ├── PaymentSystem.tsx           # UI presentation components
│   │   ├── PaymentSystem.spec.tsx
│   │   └── PaymentSystem.module.scss
│   └── hooks/
│       ├── usePaymentSystem.ts         # React hooks for UI integration
│       └── usePaymentSystem.test.ts
└── types.ts
```

---

### Component-Based Architecture

**📐 Philosophy:** Organization through reusable components and hierarchical structure

**🎯 When to use:**
- Applications with extensive reusable component requirements
- Design system development and implementation
- Applications prioritizing code reusability and modularity
- Teams focused on UI/UX development workflows

**✅ Benefits:**
- Highly scalable component-driven development approach
- Maximum code reusability across application features
- Clear encapsulation with maintainable component boundaries
- Natural testing and debugging at component level

**⚠️ Limitations:**
- Risk of organizational chaos without proper conventions
- Potential implicit coupling through props and state management

**📁 Generated Structure:**
```bash
cli-frontend UILibrary --type feature --architecture component-based
```
```
UILibrary/
├── components/
│   ├── common/
│   │   ├── UILibrary.tsx               # Reusable component library
│   │   ├── UILibrary.spec.tsx
│   │   └── UILibrary.module.scss
│   ├── containers/
│   │   ├── UILibraryContainer.tsx      # Logic-heavy container components
│   │   ├── UILibraryContainer.spec.tsx
│   │   └── UILibraryContainer.module.scss
│   └── presentation/
│       ├── UILibraryPresentation.tsx   # Pure presentation components
│       ├── UILibraryPresentation.spec.tsx
│       └── UILibraryPresentation.module.scss
├── hooks/
│   ├── useUILibrary.ts                 # Custom component hooks
│   └── useUILibrary.test.ts
└── types.ts
```

---

### Atomic Design

**📐 Philosophy:** Component hierarchy: Atoms → Molecules → Organisms → Templates → Pages

**🎯 When to use:**
- Complex design system implementation and maintenance
- Applications requiring visual consistency across features
- Design and development teams working collaboratively
- Systems with extensive reusable component requirements

**✅ Benefits:**
- Systematic design consistency across application layers
- Clear UI component modularization and reusability
- Natural design system development methodology
- Facilitates designer-developer collaboration workflows

**⚠️ Limitations:**
- Limited business logic coverage requiring additional patterns
- Requires discipline to maintain proper component hierarchy

**📁 Generated Structure:**
```bash
cli-frontend DesignSystem --type feature --architecture atomic-design
```
```
DesignSystem/
├── atoms/
│   ├── DesignSystemAtom.tsx        # Basic UI elements (buttons, inputs)
│   ├── DesignSystemAtom.spec.tsx
│   └── DesignSystemAtom.module.scss
├── molecules/
│   ├── DesignSystemMolecule.tsx    # Combined atomic components
│   ├── DesignSystemMolecule.spec.tsx
│   └── DesignSystemMolecule.module.scss
├── organisms/
│   ├── DesignSystemOrganism.tsx    # Complex component sections
│   ├── DesignSystemOrganism.spec.tsx
│   └── DesignSystemOrganism.module.scss
├── templates/
│   ├── DesignSystemTemplate.tsx    # Layout structures without content
│   ├── DesignSystemTemplate.spec.tsx
│   └── DesignSystemTemplate.module.scss
├── pages/
│   ├── DesignSystemPage.tsx        # Complete page implementations
│   ├── DesignSystemPage.spec.tsx
│   └── DesignSystemPage.module.scss
└── types.ts
```

---

### Micro-Frontends

**📐 Philosophy:** Each feature operates as an independent "mini-frontend" application

**🎯 When to use:**
- Large teams working in parallel on different features
- Applications requiring independent deployment capabilities
- Gradual migration from legacy systems and technologies
- Organizations with diverse technology stacks per team

**✅ Benefits:**
- Excellent scalability for large development teams
- Independent deployment and release cycles
- Technology stack freedom for different teams
- Fault isolation and system resilience

**⚠️ Limitations:**
- Complex integration and orchestration requirements
- Potential dependency duplication and bundle size issues
- Increased infrastructure and deployment complexity

**📁 Generated Structure:**
```bash
cli-frontend OrderModule --type feature --architecture micro-frontends
```
```
OrderModule/
├── shell/
│   ├── OrderModuleShell.tsx        # Microfrontend container and orchestration
│   ├── OrderModuleShell.spec.tsx
│   └── OrderModuleShell.module.scss
├── components/
│   ├── OrderModule.tsx             # Internal module components
│   ├── OrderModule.spec.tsx
│   └── OrderModule.module.scss
├── services/
│   └── OrderModuleService.ts       # Module-specific service implementations
├── hooks/
│   ├── useOrderModule.ts           # Module-scoped custom hooks
│   └── useOrderModule.test.ts
├── api/
│   └── OrderModuleApi.ts           # Module API interface and contracts
└── types.ts                        # Public type definitions and interfaces
```

---

### Event-Driven Architecture

**📐 Philosophy:** Feature communication through decoupled event publishing and subscription

**🎯 When to use:**
- Highly interactive applications with real-time requirements
- Real-time systems with live data synchronization needs
- Applications with independent modules requiring loose coupling
- Chat applications, gaming interfaces, and live dashboards

**✅ Benefits:**
- Extremely low coupling between system components
- High flexibility and extensibility for new features
- Optimal for highly interactive and real-time applications
- Natural horizontal scalability patterns

**⚠️ Limitations:**
- Complex debugging and event flow tracing
- Challenging event sequence and dependency management

**📁 Generated Structure:**
```bash
cli-frontend NotificationSystem --type feature --architecture event-driven
```
```
NotificationSystem/
├── events/
│   └── NotificationSystemEvents.ts    # Event type definitions and schemas
├── listeners/
│   └── NotificationSystemListeners.ts # Event listener implementations
├── publishers/
│   └── NotificationSystemPublisher.ts # Event publishing and dispatch logic
├── components/
│   ├── NotificationSystem.tsx         # Event-reactive UI components
│   ├── NotificationSystem.spec.tsx
│   └── NotificationSystem.module.scss
├── hooks/
│   ├── useNotificationSystemEvents.ts # Event subscription and handling hooks
│   └── useNotificationSystemEvents.test.ts
└── types.ts                           # Event payload and interface definitions
```

---

## Architecture Selection Guide

### 🎯 Small to Medium Projects (< 10 developers)
- **Screaming Architecture** - For well-defined feature boundaries
- **Component-Based** - For UI-heavy applications and design systems
- **MVVM** - For complex forms and reactive data requirements

### 🏢 Enterprise Projects (> 10 developers)
- **Clean Architecture** - For complex long-term systems requiring maintainability
- **Micro-Frontends** - For large independent teams with deployment autonomy
- **MVC/MVP** - For traditional enterprise systems with established patterns

### 🎨 Design System Development
- **Atomic Design** - For comprehensive component libraries and design systems
- **Component-Based** - For reusable component system development

### ⚡ Interactive Applications
- **Event-Driven** - For real-time systems and live data applications
- **Flux/Redux** - For complex state management and predictable data flow

### 📊 Decision Matrix

| Scenario | Recommended Architecture | Alternative |
|----------|-------------------------|-------------|
| Small app with clear feature boundaries | Screaming Architecture | Component-Based |
| Complex enterprise system | Clean Architecture | MVC |
| Design system development | Atomic Design | Component-Based |
| Complex state management | Redux | Flux |
| Large independent teams | Micro-Frontends | Clean Architecture |
| Real-time applications | Event-Driven | Flux |
| Complex form applications | MVVM | MVP |
| Traditional CRUD systems | MVC | MVP |

---

## Creating Custom Architectures

You can create custom architectures by adding JSON configuration files to the `architectures/` directory:

### 1. Create JSON Configuration File
```bash
# Create new architecture configuration
nano architectures/my-architecture.json
```

### 2. Define Architecture Structure
```json
{
  "name": "My Custom Architecture",
  "description": "Technical description of your custom architecture pattern",
  "benefits": [
    "Benefit 1: Specific technical advantage",
    "Benefit 2: Performance or maintainability improvement"
  ],
  "limitations": [
    "Limitation 1: Specific constraint or trade-off",
    "Limitation 2: Learning curve or complexity consideration"
  ],
  "structure": [
    {
      "path": "folder1",
      "template": "component",
      "filename_pattern": "{name}Custom",
      "description": "Technical description of this component's role"
    },
    {
      "path": "folder2", 
      "template": "service",
      "filename_pattern": "{name}Service",
      "description": "Service layer implementation details"
    }
  ]
}
```

### 3. Use Your Custom Architecture
```bash
cli-frontend MyFeature --type feature --architecture my-architecture
```

### Available Filename Pattern Variables:
- `{name}` - Original feature name
- `use{name}` - Automatically prefixed for React hooks
- `{name}Context` - Suffixed for React context providers
- `{name}Provider` - Suffixed for provider components  
- `{name}Page` - Suffixed for page components

### Available Template Types:
- `component` - React component with TypeScript and SCSS
- `hook` - Custom React hook with testing
- `service` - Service class or utility functions
- `context` - React context implementation
- `page` - Page component with routing integration
- `store` - Redux/state management store
- `api` - API service interface and implementation
- `types` - TypeScript type definitions and interfaces

---

## 🤝 Contributing

Have ideas for new architectures? Contributions are welcome!

1. Create a new architecture JSON configuration
2. Document technical benefits and limitations
3. Add comprehensive usage examples and scenarios
4. Submit a Pull Request with detailed implementation notes

---

## 📚 References

- [Clean Architecture - Uncle Bob](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html)
- [Flux Architecture Documentation](https://facebookarchive.github.io/flux/docs/in-depth-overview)
- [Atomic Design Methodology - Brad Frost](https://atomicdesign.bradfrost.com/)
- [Screaming Architecture Principles](https://blog.cleancoder.com/uncle-bob/2011/09/30/Screaming-Architecture.html)
- [Micro-Frontends Implementation Guide](https://micro-frontends.org/)

---

**💡 Need help choosing an architecture?** Consider your team size, project complexity, and future requirements. When in doubt, **Screaming Architecture** provides an excellent default foundation for most frontend applications.