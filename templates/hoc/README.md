# HOC Template Documentation

Template simple para crear Higher-Order Components (HOC) reutilizables con TypeScript y soporte opcional para ref forwarding.

## Características

- ✅ **TypeScript genérico** - Soporte completo para tipos
- ✅ **Ref forwarding opcional** - Configurable via .conf
- ✅ **Display name automático** - Para debugging en React DevTools
- ✅ **Tests completos** - Incluye casos de uso y edge cases
- ✅ **Lógica reutilizable** - Base para implementar patrones comunes
- ✅ **Simple y directo** - Principios KISS

## Uso

```bash
cli-frontend withAuth --type hoc
```

Esto genera:
```
withAuth/
├── withAuth.hoc.tsx        # HOC principal
├── withAuth.hoc.spec.tsx   # Tests unitarios
└── README.md               # Esta documentación
```

## Configuración (.conf)

```ini
# HOC configuration
var_use_ref=true

# Developer configuration
var_author=Frontend Team
var_license=MIT
var_description=Simple Higher-Order Component for reusable logic
```

### Variables disponibles:
- `var_use_ref` - true/false para habilitar ref forwarding
- Todas las variables estándar (timestamp, author, etc.)

## Ejemplos de Uso

### Uso Básico

```tsx
import { withAuth } from './withAuth.hoc';

// Componente que necesita autenticación
const Dashboard: React.FC = () => (
  <div>Dashboard content</div>
);

// Aplicar HOC
const ProtectedDashboard = withAuth(Dashboard);

// Usar componente protegido
function App() {
  return <ProtectedDashboard />;
}
```

### Con Ref Forwarding

```tsx
import React, { useRef } from 'react';
import { withAuth } from './withAuth.hoc';

const InputComponent = React.forwardRef<HTMLInputElement, { placeholder: string }>(
  ({ placeholder }, ref) => (
    <input ref={ref} placeholder={placeholder} />
  )
);

const ProtectedInput = withAuth(InputComponent);

function App() {
  const inputRef = useRef<HTMLInputElement>(null);

  return (
    <div>
      <ProtectedInput ref={inputRef} placeholder="Enter text" />
      <button onClick={() => inputRef.current?.focus()}>
        Focus Input
      </button>
    </div>
  );
}
```

## Patrones de Implementación

### 1. Authentication HOC

```tsx
export function withAuth<P extends object>(
  WrappedComponent: ComponentType<P>
): ComponentType<P & { isAuthenticated?: boolean }> {
  const Enhanced = forwardRef<any, P & { isAuthenticated?: boolean }>((props, ref) => {
    const { isAuthenticated = false, ...rest } = props;

    if (!isAuthenticated) {
      return <div>Please log in to access this content.</div>;
    }

    return <WrappedComponent {...(rest as P)} ref={ref} />;
  });

  Enhanced.displayName = `withAuth($\{WrappedComponent.displayName || WrappedComponent.name})`;
  return Enhanced;
}
```

### 2. Loading HOC

```tsx
export function withLoading<P extends object>(
  WrappedComponent: ComponentType<P>
): ComponentType<P & { isLoading?: boolean }> {
  const Enhanced = forwardRef<any, P & { isLoading?: boolean }>((props, ref) => {
    const { isLoading = false, ...rest } = props;

    if (isLoading) {
      return <div>Loading...</div>;
    }

    return <WrappedComponent {...(rest as P)} ref={ref} />;
  });

  Enhanced.displayName = `withLoading($\{WrappedComponent.displayName || WrappedComponent.name})`;
  return Enhanced;
}
```

### 3. Logging HOC

```tsx
export function withLogging<P extends object>(
  WrappedComponent: ComponentType<P>
): ComponentType<P> {
  const Enhanced = forwardRef<any, P>((props, ref) => {
    React.useEffect(() => {
      console.log(`[$\{WrappedComponent.name}] Component mounted`);
      return () => {
        console.log(`[$\{WrappedComponent.name}] Component unmounted`);
      };
    }, []);

    return <WrappedComponent {...props} ref={ref} />;
  });

  Enhanced.displayName = `withLogging($\{WrappedComponent.displayName || WrappedComponent.name})`;
  return Enhanced;
}
```

## Composición de HOCs

```tsx
import { withAuth } from './withAuth.hoc';
import { withLoading } from './withLoading.hoc';
import { withLogging } from './withLogging.hoc';

// Componer múltiples HOCs
const enhance = (Component) =>
  withAuth(
    withLoading(
      withLogging(Component)
    )
  );

const EnhancedComponent = enhance(MyComponent);
```

## Testing

El template incluye tests completos:

```bash
npm test -- withAuth.hoc
```

### Casos de prueba incluidos:
- ✅ Renderizado del componente envuelto
- ✅ Paso de props al componente hijo
- ✅ Display name correcto
- ✅ Ref forwarding (si está habilitado)
- ✅ Preservación de funcionalidad
- ✅ Manejo de componentes anónimos

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

- ✅ **Autenticación** - Proteger componentes que requieren login
- ✅ **Loading states** - Mostrar spinners mientras cargan datos
- ✅ **Logging/Analytics** - Trackear uso de componentes
- ✅ **Error boundaries** - Envolver componentes con manejo de errores
- ✅ **Permisos** - Verificar roles de usuario
- ✅ **Theming** - Inyectar temas o estilos
- ✅ **Performance** - Memoización y optimizaciones

## Mejores Prácticas

### 1. Naming Convention
```tsx
// Usar prefijo descriptivo
withAuth, withLoading, withTheme, etc.
```

### 2. TypeScript Generics
```tsx
// Preservar tipos del componente original
function withEnhancement<P extends object>(
  WrappedComponent: ComponentType<P>
): ComponentType<P & AdditionalProps>
```

### 3. Display Name
```tsx
// Siempre configurar para debugging
Enhanced.displayName = `withHOC($\{WrappedComponent.displayName || WrappedComponent.name})`;
```

### 4. Ref Forwarding
```tsx
// Usar forwardRef cuando sea necesario
const Enhanced = forwardRef<RefType, Props>((props, ref) => {
  return <WrappedComponent {...props} ref={ref} />;
});
```

Este template proporciona una base sólida y simple para crear HOCs reutilizables manteniendo las mejores prácticas de React y TypeScript.