# Error Boundary Template Documentation

Template simple para crear Error Boundary components que capturen errores en el árbol de componentes React y muestren fallbacks profesionales.

## Características

- ✅ **Class Component** - Error Boundaries requieren class components
- ✅ **Fallback UI personalizable** - Mensaje de error configurable
- ✅ **Retry functionality** - Botón para reintentar
- ✅ **Error logging opcional** - Configurable via .conf
- ✅ **Error details opcionales** - Para desarrollo/debugging
- ✅ **Callback de error** - Para integración con servicios de monitoreo
- ✅ **Tests completos** - Casos de error y recuperación

## Uso

```bash
cli-frontend AppErrorBoundary --type error-boundary
```

Esto genera:
```
AppErrorBoundary/
├── AppErrorBoundary.boundary.tsx      # Error Boundary principal
├── AppErrorBoundary.boundary.spec.tsx # Tests unitarios
└── README.md                          # Esta documentación
```

## Configuración (.conf)

```ini
# Error Boundary configuration
var_show_error_details=false
var_enable_logging=true
var_fallback_message=Something went wrong

# Developer configuration
var_author=Frontend Team
var_license=MIT
var_description=Simple Error Boundary component for catching React errors
```

### Variables disponibles:
- `var_show_error_details` - true/false para mostrar detalles técnicos del error
- `var_enable_logging` - true/false para logging automático a consola
- `var_fallback_message` - Mensaje de error personalizado
- Todas las variables estándar (timestamp, author, etc.)

## Ejemplos de Uso

### Uso Básico

```tsx
import { AppErrorBoundary } from './AppErrorBoundary.boundary';

function App() {
  return (
    <AppErrorBoundary>
      <Header />
      <MainContent />
      <Footer />
    </AppErrorBoundary>
  );
}
```

### Con Fallback Personalizado

```tsx
import { AppErrorBoundary } from './AppErrorBoundary.boundary';

const CustomErrorUI = (
  <div style=\{\{ textAlign: 'center', padding: '2rem' }}>
    <h2>Oops! Algo salió mal</h2>
    <p>Por favor, actualiza la página o contacta soporte.</p>
  </div>
);

function App() {
  return (
    <AppErrorBoundary fallback={CustomErrorUI}>
      <MyComponent />
    </AppErrorBoundary>
  );
}
```

### Con Error Handler

```tsx
import { AppErrorBoundary } from './AppErrorBoundary.boundary';

const handleError = (error: Error, errorInfo: React.ErrorInfo) => {
  // Enviar a servicio de monitoreo
  console.error('Error capturado:', error);

  // Opcional: enviar a Sentry, LogRocket, etc.
  // Sentry.captureException(error, { extra: errorInfo });
};

function App() {
  return (
    <AppErrorBoundary onError={handleError}>
      <MyComponent />
    </AppErrorBoundary>
  );
}
```

### Error Boundaries Anidados

```tsx
import { AppErrorBoundary } from './AppErrorBoundary.boundary';

function App() {
  return (
    <AppErrorBoundary>
      <Header />

      {/* Error boundary específico para el sidebar */}
      <AppErrorBoundary fallback={<div>Sidebar no disponible</div>}>
        <Sidebar />
      </AppErrorBoundary>

      {/* Error boundary específico para el contenido principal */}
      <AppErrorBoundary fallback={<div>Contenido no disponible</div>}>
        <MainContent />
      </AppErrorBoundary>

      <Footer />
    </AppErrorBoundary>
  );
}
```

## Props del Componente

### AppErrorBoundaryProps

| Prop | Tipo | Descripción |
|------|------|-------------|
| `children` | `ReactNode` | Componentes hijos a proteger |
| `fallback` | `ReactNode?` | UI personalizada para mostrar en caso de error |
| `onError` | `(error: Error, errorInfo: React.ErrorInfo) => void?` | Callback cuando ocurre un error |

## Funcionalidades

### 1. **Captura de Errores**
- Captura errores JavaScript en componentes hijos
- Previene que la app completa se crashee
- Funciona solo con errores en el render y lifecycle methods

### 2. **Fallback UI**
- Mensaje de error configurable
- Botón "Try Again" para reintentar
- Detalles del error (opcional, solo en desarrollo)

### 3. **Error Logging**
- Log automático a consola (configurable)
- Información detallada del error y stack trace
- Integración fácil con servicios de monitoreo

### 4. **Recovery**
- Botón para resetear el estado de error
- Permite al usuario intentar de nuevo
- Mantiene el resto de la aplicación funcionando

## Limitaciones de Error Boundaries

Los Error Boundaries **NO** capturan errores en:
- Event handlers
- Código asíncrono (setTimeout, requestAnimationFrame, etc.)
- Errores durante Server-side rendering
- Errores en el propio Error Boundary

Para estos casos, usa try-catch tradicional:

```tsx
// Para event handlers
const handleClick = () => {
  try {
    // código que puede fallar
    riskyOperation();
  } catch (error) {
    console.error('Error en click handler:', error);
  }
};

// Para código asíncrono
const fetchData = async () => {
  try {
    const data = await api.getData();
    setData(data);
  } catch (error) {
    setError(error);
  }
};
```

## Testing

El template incluye tests completos:

```bash
npm test -- AppErrorBoundary.boundary
```

### Casos de prueba incluidos:
- ✅ Renderizado normal sin errores
- ✅ Captura y display de errores
- ✅ Fallback UI personalizado
- ✅ Callback onError
- ✅ Logging de errores
- ✅ Funcionalidad de retry
- ✅ Accesibilidad del botón retry
- ✅ Múltiples errores

## Dependencias

```json
{
  "dependencies": {
    "react": "^18.0.0"
  },
  "devDependencies": {
    "@testing-library/jest-dom": "^6.0.0",
    "@testing-library/react": "^14.0.0",
    "@types/react": "^18.0.0"
  }
}
```

## Casos de Uso Ideales

- ✅ **Aplicaciones críticas** - Prevenir crashes completos
- ✅ **Componentes de terceros** - Aislar código externo no confiable
- ✅ **Funcionalidades experimentales** - Proteger features nuevas
- ✅ **Secciones opcionales** - Sidebar, widgets, etc.
- ✅ **Integración con monitoreo** - Sentry, LogRocket, Bugsnag
- ✅ **Experiencia de usuario** - Mantener app funcional

## Mejores Prácticas

### 1. **Granularidad**
```tsx
// ✅ Bueno - Error boundaries específicos
<ErrorBoundary fallback={<SidebarError />}>
  <Sidebar />
</ErrorBoundary>

// ❌ Malo - Un error boundary para toda la app
<ErrorBoundary>
  <EntireApp />
</ErrorBoundary>
```

### 2. **Fallback Útiles**
```tsx
// ✅ Bueno - Fallback específico y útil
const fallback = (
  <div>
    <p>Error loading comments</p>
    <button onClick={reloadComments}>Reload Comments</button>
  </div>
);

// ❌ Malo - Fallback genérico
const fallback = <div>Error</div>;
```

### 3. **Error Reporting**
```tsx
// ✅ Bueno - Reportar errores para análisis
const handleError = (error, errorInfo) => {
  analytics.track('error_boundary_triggered', {
    error: error.message,
    component: errorInfo.componentStack
  });
};
```

Este template proporciona una base sólida y simple para implementar Error Boundaries efectivos en aplicaciones React.