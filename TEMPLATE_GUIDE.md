# 📚 Guía para Crear Templates Personalizados

Esta guía te explica paso a paso cómo crear nuevos templates para el CLI Frontend Generator y cómo funciona internamente el sistema de templates.

## 🎯 ¿Qué hace nuestro software?

El CLI Frontend Generator es una herramienta que automatiza la creación de código frontend mediante un sistema de plantillas (templates). El software:

1. **Lee templates** desde un directorio configurable
2. **Procesa variables** y aplicar transformaciones de nomenclatura
3. **Genera archivos** basados en las plantillas con el nombre proporcionado
4. **Crea estructuras** de carpetas organizadas
5. **Aplica convenciones** de nomenclatura automáticamente

## 🔧 ¿Cómo funciona internamente?

### 1. **Flujo de Ejecución**
```
Usuario ejecuta comando → CLI parsea argumentos → Carga configuración → 
Busca template → Procesa archivos → Aplica transformaciones → Genera output
```

### 2. **Sistema de Templates**
- Utiliza **Handlebars** como motor de plantillas
- Soporta **variables dinámicas** y **helpers personalizados**
- Procesa **nombres inteligentes** para diferentes tipos de archivos
- Aplica **transformaciones de caso** automáticamente

### 3. **Procesamiento de Nombres**
El software incluye lógica inteligente para procesar nombres:
- **Hooks**: Agrega `use` al inicio si no está presente
- **Contexts**: Agrega `Context` al final si no está presente
- **Providers**: Agrega `Provider` y maneja sufijos existentes
- **Pages**: Agrega `Page` al final si no está presente

## 📝 Paso a Paso: Crear un Nuevo Template

### Paso 1: Preparar el Directorio de Templates

1. **Localiza el directorio de templates**:
   ```bash
   # Por defecto en: ~/.cli-template/ o ./templates/
   ```

2. **Crea una nueva carpeta** para tu template:
   ```bash
   mkdir ~/.cli-template/mi-nuevo-template
   ```

### Paso 2: Crear Archivos de Template

Los archivos de template pueden usar las siguientes variables:

#### **Variables Básicas**
- `$FILE_NAME` - Nombre original proporcionado
- `{{name}}` - Nombre original (Handlebars)
- `{{pascal_name}}` - PascalCase (ej: `MyComponent`)
- `{{camel_name}}` - camelCase (ej: `myComponent`)
- `{{snake_name}}` - snake_case (ej: `my_component`)
- `{{kebab_name}}` - kebab-case (ej: `my-component`)
- `{{upper_name}}` - MAYÚSCULAS (ej: `MY_COMPONENT`)

#### **Variables Inteligentes**
- `{{hook_name}}` - Nombre optimizado para hooks
- `{{context_name}}` - Nombre optimizado para contextos
- `{{provider_name}}` - Nombre optimizado para proveedores
- `{{page_name}}` - Nombre optimizado para páginas

### Paso 3: Ejemplo Práctico - Template para Store (Redux)

Vamos a crear un template para generar stores de Redux:

#### 3.1 Crear la estructura
```bash
mkdir ~/.cli-template/store
cd ~/.cli-template/store
```

#### 3.2 Crear el archivo del store (`$FILE_NAME.store.ts`)
```typescript
import { createSlice, PayloadAction } from '@reduxjs/toolkit';

// Estado inicial para {{pascal_name}}
interface {{pascal_name}}State {
  data: any[];
  loading: boolean;
  error: string | null;
}

const initialState: {{pascal_name}}State = {
  data: [],
  loading: false,
  error: null,
};

// Slice para {{pascal_name}}
export const {{camel_name}}Slice = createSlice({
  name: '{{kebab_name}}',
  initialState,
  reducers: {
    set{{pascal_name}}Loading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
    },
    set{{pascal_name}}Data: (state, action: PayloadAction<any[]>) => {
      state.data = action.payload;
      state.loading = false;
      state.error = null;
    },
    set{{pascal_name}}Error: (state, action: PayloadAction<string>) => {
      state.error = action.payload;
      state.loading = false;
    },
    clear{{pascal_name}}: (state) => {
      state.data = [];
      state.error = null;
    },
  },
});

export const {
  set{{pascal_name}}Loading,
  set{{pascal_name}}Data,
  set{{pascal_name}}Error,
  clear{{pascal_name}},
} = {{camel_name}}Slice.actions;

export default {{camel_name}}Slice.reducer;
```

#### 3.3 Crear el archivo de tipos (`$FILE_NAME.types.ts`)
```typescript
// Tipos para {{pascal_name}}
export interface {{pascal_name}}Item {
  id: string | number;
  // Agrega propiedades específicas aquí
}

export interface {{pascal_name}}Response {
  data: {{pascal_name}}Item[];
  total: number;
  page: number;
}

export interface {{pascal_name}}Filters {
  search?: string;
  status?: string;
  // Agrega filtros específicos aquí
}
```

#### 3.4 Crear el archivo de acciones async (`$FILE_NAME.thunks.ts`)
```typescript
import { createAsyncThunk } from '@reduxjs/toolkit';
import { {{pascal_name}}Response, {{pascal_name}}Filters } from './{{kebab_name}}.types';

// Thunk para obtener {{pascal_name}}
export const fetch{{pascal_name}} = createAsyncThunk(
  '{{kebab_name}}/fetch{{pascal_name}}',
  async (filters: {{pascal_name}}Filters = {}) => {
    // Implementa tu lógica de API aquí
    const response = await fetch('/api/{{kebab_name}}', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(filters),
    });
    
    if (!response.ok) {
      throw new Error('Failed to fetch {{kebab_name}}');
    }
    
    const data: {{pascal_name}}Response = await response.json();
    return data;
  }
);

// Thunk para crear {{pascal_name}}
export const create{{pascal_name}} = createAsyncThunk(
  '{{kebab_name}}/create{{pascal_name}}',
  async (newItem: Omit<{{pascal_name}}Item, 'id'>) => {
    const response = await fetch('/api/{{kebab_name}}', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(newItem),
    });
    
    if (!response.ok) {
      throw new Error('Failed to create {{kebab_name}}');
    }
    
    return await response.json();
  }
);
```

#### 3.5 Crear archivo de tests (`$FILE_NAME.store.test.ts`)
```typescript
import { configureStore } from '@reduxjs/toolkit';
import {{camel_name}}Reducer, {
  set{{pascal_name}}Loading,
  set{{pascal_name}}Data,
  set{{pascal_name}}Error,
  clear{{pascal_name}},
} from './{{kebab_name}}.store';

describe('{{pascal_name}} Store', () => {
  let store: ReturnType<typeof configureStore>;

  beforeEach(() => {
    store = configureStore({
      reducer: {
        {{camel_name}}: {{camel_name}}Reducer,
      },
    });
  });

  it('should handle set{{pascal_name}}Loading', () => {
    store.dispatch(set{{pascal_name}}Loading(true));
    const state = store.getState().{{camel_name}};
    expect(state.loading).toBe(true);
  });

  it('should handle set{{pascal_name}}Data', () => {
    const mockData = [{ id: 1, name: 'Test' }];
    store.dispatch(set{{pascal_name}}Data(mockData));
    const state = store.getState().{{camel_name}};
    
    expect(state.data).toEqual(mockData);
    expect(state.loading).toBe(false);
    expect(state.error).toBe(null);
  });

  it('should handle set{{pascal_name}}Error', () => {
    const errorMessage = 'Test error';
    store.dispatch(set{{pascal_name}}Error(errorMessage));
    const state = store.getState().{{camel_name}};
    
    expect(state.error).toBe(errorMessage);
    expect(state.loading).toBe(false);
  });

  it('should handle clear{{pascal_name}}', () => {
    // Primero agregamos datos
    store.dispatch(set{{pascal_name}}Data([{ id: 1, name: 'Test' }]));
    // Luego los limpiamos
    store.dispatch(clear{{pascal_name}}());
    const state = store.getState().{{camel_name}};
    
    expect(state.data).toEqual([]);
    expect(state.error).toBe(null);
  });
});
```

### Paso 4: Actualizar el CLI (si es necesario)

Si quieres que tu nuevo template aparezca en la lista de tipos disponibles, actualiza el enum en `src/cli.rs`:

```rust
#[derive(ValueEnum, Clone, Debug)]
pub enum TemplateType {
    Component,
    Hook, 
    Service,
    Context,
    Page,
    Store,  // ← Agregar aquí
}
```

### Paso 5: Probar el Nuevo Template

```bash
# Generar un store
cli-frontend UserStore --type store

# O si no agregaste el enum, usa el directorio:
cli-frontend UserStore --type store
```

## 📋 Convenciones y Mejores Prácticas

### 1. **Nomenclatura de Archivos**
- Usa `$FILE_NAME` para reemplazos simples
- Usa `{{variable}}` para transformaciones con Handlebars
- Nombra archivos descriptivamente: `$FILE_NAME.store.ts`, `$FILE_NAME.types.ts`

### 2. **Estructura de Carpetas**
```
mi-template/
├── $FILE_NAME.main.ext       # Archivo principal
├── $FILE_NAME.types.ext      # Definiciones de tipos
├── $FILE_NAME.test.ext       # Archivos de testing
├── $FILE_NAME.styles.ext     # Estilos (si aplica)
└── subfolder/                # Subcarpetas si es necesario
    └── $FILE_NAME.helper.ext
```

### 3. **Documentación en Templates**
Incluye comentarios descriptivos en tus templates:
```typescript
/**
 * {{pascal_name}} Component
 * Generado automáticamente por CLI Frontend Generator
 * 
 * @description TODO: Describe qué hace este componente
 * @author {{author}} - Fecha: {{date}}
 */
```

### 4. **Variables de Configuración Avanzadas**
Puedes usar variables de entorno o configuración en tus templates:
```typescript
const API_BASE_URL = process.env.REACT_APP_API_URL || 'http://localhost:3000';
const {{upper_name}}_ENDPOINT = '{{kebab_name}}';
```

## 🚀 Templates Avanzados

### Helper Personalizado en Templates
Si necesitas lógica más compleja, puedes usar helpers de Handlebars:

```handlebars
{{#if (eq template_type "component")}}
// Código específico para componentes
{{/if}}

{{#each props}}
  {{@key}}: {{this}};
{{/each}}
```

### Plantillas Condicionales
```typescript
{{#if enable_hooks}}
import { useState, useEffect } from 'react';
{{/if}}

{{#if include_styles}}
import styles from './{{kebab_name}}.module.scss';
{{/if}}
```

## 🔍 Debugging y Troubleshooting

### Problemas Comunes

1. **Template no encontrado**:
   - Verifica que la carpeta existe en el directorio de templates
   - Revisa la configuración de `templates_dir`

2. **Variables no se reemplazan**:
   - Usa `$FILE_NAME` para reemplazos directos
   - Usa `{{variable}}` para Handlebars
   - Verifica la sintaxis de Handlebars

3. **Archivos no se generan**:
   - Revisa los permisos del directorio de output
   - Verifica que no hay errores en la sintaxis del template

### Modo Debug
Para ver qué está pasando internamente:
```bash
RUST_LOG=debug cli-frontend MyComponent --type component
```

## 🎉 Conclusión

Crear templates personalizados te permite:
- **Automatizar** patrones específicos de tu proyecto
- **Mantener consistencia** en el código
- **Acelerar el desarrollo** eliminando código repetitivo
- **Estandarizar** estructuras de archivos y nomenclatura

¡Experimenta con diferentes tipos de templates y adapta el CLI a las necesidades específicas de tu proyecto! 🚀

---

## 📚 Recursos Adicionales

- [Documentación de Handlebars](https://handlebarsjs.com/)
- [Convenciones de nomenclatura](https://developer.mozilla.org/en-US/docs/MDN/Writing_guidelines/Writing_style_guide/Code_style_guide)
- [Mejores prácticas de React](https://react.dev/learn/thinking-in-react)