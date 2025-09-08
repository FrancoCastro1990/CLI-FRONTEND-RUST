# API Service Template Documentation

Este template genera un servicio API completo con TypeScript, manejo de errores, caché, reintentos automáticos y configuración avanzada.

## Características

- ✅ **TypeScript completo** con tipos e interfaces
- ✅ **Configuración personalizable** mediante archivo `.conf`
- ✅ **Variables de entorno dinámicas** (timestamp, UUID, environment)
- ✅ **Manejo de errores avanzado** con reintentos automáticos
- ✅ **Sistema de caché** configurable
- ✅ **Rate limiting** para control de peticiones
- ✅ **Autenticación opcional** con tokens
- ✅ **Tests unitarios completos** con Jest
- ✅ **Interceptores de Axios** para logging y depuración
- ✅ **Documentación automática** generada

## Uso

```bash
cli-frontend UserAPI --type api-service
```

Esto genera:
- `UserAPI.service.ts` - Servicio principal
- `UserAPI.service.test.ts` - Tests unitarios

## Configuración (.conf)

El archivo `.conf` permite personalizar el template con variables específicas:

```ini
# Configuración de entorno
environment=production

# Toggles de funcionalidades
enable_timestamps=true
enable_uuid=true

# Variables personalizadas (prefijo var_)
var_api_version=v1
var_base_url=https://api.example.com
var_timeout=5000
var_retry_attempts=3
var_author=Frontend Team
var_license=MIT

# Configuración específica del servicio
var_auth_required=true
var_rate_limit=1000
var_cache_enabled=true
var_debug_mode=false

# Variables de documentación
var_description=Auto-generated API service for data access
var_swagger_url=/api/docs
```

## Variables Disponibles

### Variables Core
- `{{name}}` - Nombre original
- `{{pascal_name}}` - PascalCase (UserAPI)
- `{{camel_name}}` - camelCase (userAPI)
- `{{kebab_name}}` - kebab-case (user-api)
- `{{snake_name}}` - snake_case (user_api)
- `{{upper_name}}` - UPPER_CASE (USER_API)

### Variables de Entorno
- `{{environment}}` - Entorno actual (development/production)
- `{{timestamp}}` - Timestamp ISO 8601
- `{{timestamp_iso}}` - Timestamp ISO con milisegundos
- `{{date}}` - Fecha actual (YYYY-MM-DD)
- `{{time}}` - Hora actual (HH:MM:SS)
- `{{year}}` - Año actual
- `{{uuid}}` - UUID v4 completo
- `{{uuid_simple}}` - UUID v4 sin guiones

### Variables de Generación
- `{{version}}` - Versión del CLI
- `{{generator_name}}` - Nombre del generador
- `{{generated}}` - Boolean true

### Variables Personalizadas
Cualquier variable definida en `.conf` con prefijo `var_`:
- `{{api_version}}` - Versión de la API
- `{{base_url}}` - URL base del servicio
- `{{timeout}}` - Timeout en milisegundos
- `{{author}}` - Autor del código
- `{{license}}` - Licencia del proyecto
- Y muchas más...

## Lógica Condicional

El template soporta lógica condicional con Handlebars:

```typescript
{{#if auth_required}}
// Código cuando autenticación es requerida
'Authorization': `Bearer ${this.getAuthToken()}`,
{{/if}}

{{#if debug_mode}}
console.log('Debug info:', data);
{{/if}}

// Comparaciones
{{#if (eq environment "production")}}
// Código solo para producción
{{/if}}
```

## Ejemplo de Código Generado

### Para: `cli-frontend UserAPI --type api-service`

```typescript
/**
 * UserAPI API Service
 * Auto-generated API service for data access
 * 
 * @generated true
 * @generator CLI Frontend Generator v1.2.3
 * @timestamp 2024-01-01T12:00:00.000Z
 * @author Frontend Team
 * @license MIT
 * 
 * Environment: production
 * API Version: v1
 * Base URL: https://api.example.com
 */

export class UserAPIService {
  // ... implementación completa
}

export const userAPIService = new UserAPIService();
```

## Funcionalidades Incluidas

### 1. **CRUD Completo**
- `getAll()` - Obtener todas las entidades
- `getById(id)` - Obtener por ID
- `create(data)` - Crear nueva entidad
- `update(id, data)` - Actualizar entidad
- `delete(id)` - Eliminar entidad

### 2. **Sistema de Caché**
- Caché automático de respuestas GET
- Timeout configurable (5 minutos por defecto)
- Invalidación automática en operaciones de modificación

### 3. **Manejo de Errores**
- Reintentos automáticos para errores 5xx
- Manejo específico de errores de red
- Mensajes de error descriptivos

### 4. **Rate Limiting**
- Control de peticiones por hora
- Reset automático del contador
- Prevención de abuso de API

### 5. **Tests Unitarios**
- Tests para todas las operaciones CRUD
- Mocking de Axios
- Tests de configuración y errores
- Cobertura completa de funcionalidades

## Personalización Avanzada

### Agregar Nuevas Variables

En `.conf`:
```ini
var_my_custom_variable=valor personalizado
```

En el template:
```typescript
// Usar la variable
const config = {
  customValue: '{{my_custom_variable}}'
};
```

### Agregar Lógica Condicional

```typescript
{{#if my_feature_enabled}}
// Funcionalidad opcional
{{else}}
// Funcionalidad alternativa
{{/if}}
```

### Modificar Templates

Los archivos del template se encuentran en:
- `templates/api-service/.conf` - Configuración
- `templates/api-service/$FILE_NAME.service.ts` - Servicio principal
- `templates/api-service/$FILE_NAME.service.test.ts` - Tests

## Dependencias Requeridas

Para usar el código generado, tu proyecto debe tener:

```json
{
  "dependencies": {
    "axios": "^1.0.0"
  },
  "devDependencies": {
    "@types/jest": "^29.0.0",
    "jest": "^29.0.0"
  }
}
```

## Casos de Uso Ideales

- ✅ **APIs REST** con operaciones CRUD
- ✅ **Servicios con autenticación** JWT/Bearer
- ✅ **Aplicaciones con caché** de datos
- ✅ **Sistemas con rate limiting** de APIs
- ✅ **Desarrollo con TDD** (tests incluidos)
- ✅ **Documentación automática** de servicios

Este template demuestra la potencia del sistema de configuración `.conf` y las variables de entorno dinámicas del CLI Frontend Generator.