# 📚 Custom Template Development Guide

**Comprehensive guide for creating production-ready templates in CLI Frontend Generator**

This guide provides technical documentation for extending the CLI Frontend Generator with custom templates. Learn how to create sophisticated template structures that integrate seamlessly with the existing architecture and provide value for professional development teams.

## 🎯 CLI Frontend Generator Architecture Overview

The CLI Frontend Generator is a sophisticated code generation tool that automates frontend development through an extensible template system. The software provides:

1. **Dynamic Template Discovery** - Automatically detects and loads custom templates without recompilation
2. **Intelligent Name Processing** - Applies context-aware transformations based on file types and conventions
3. **Handlebars Integration** - Leverages powerful templating with conditional logic and custom helpers
4. **Multi-Convention Support** - Handles PascalCase, camelCase, snake_case, kebab-case transformations
5. **Architecture Integration** - Seamlessly integrates with 12 software architecture patterns

## 🔧 Internal System Architecture

### 1. **Execution Flow**
```
Command Input → Argument Parsing → Configuration Loading → 
Template Discovery → File Processing → Variable Transformation → Output Generation
```

### 2. **Template Engine Architecture**
- **Handlebars Engine** - Industry-standard templating with conditional logic and loops
- **Dynamic Variable System** - Context-aware variable processing with intelligent naming
- **Custom Helper Functions** - Extended Handlebars functionality for specific use cases
- **File System Abstraction** - Cross-platform file operations with error handling

### 3. **Intelligent Name Processing**
The system includes sophisticated logic for processing different file types:
- **React Hooks**: Automatically prefixes `use` if not present (`useAuth` from `Auth`)
- **React Contexts**: Appends `Context` suffix when appropriate (`AuthContext` from `Auth`)
- **Provider Components**: Handles `Provider` suffix with existing suffix detection
- **Page Components**: Appends `Page` suffix for routing components (`HomePage` from `Home`)
- **Convention Detection**: Automatically detects and preserves existing naming patterns

### 4. **Template Resolution System**
```rust
// Template discovery hierarchy
1. Project-local templates  (./templates/)
2. User-global templates    (~/.cli-frontend/templates/)
3. System-default templates (built-in)
```

## 📝 Professional Template Development Workflow

### Step 1: Template Directory Setup

1. **Locate the templates directory**:
   ```bash
   # Default locations (in order of precedence):
   ./templates/          # Project-specific templates
   ~/.cli-frontend/      # User-global templates
   ```

2. **Create a new template directory**:
   ```bash
   mkdir ~/.cli-frontend/templates/my-custom-template
   ```

### Step 2: Template Variable System

The CLI provides a comprehensive variable system for maximum flexibility:

#### **Core Variables**
- `$FILE_NAME` - Direct filename replacement (legacy support)
- `{{name}}` - Original input name (Handlebars syntax)
- `{{pascal_name}}` - PascalCase transformation (`MyComponent`)
- `{{camel_name}}` - camelCase transformation (`myComponent`)
- `{{snake_name}}` - snake_case transformation (`my_component`)
- `{{kebab_name}}` - kebab-case transformation (`my-component`)
- `{{upper_name}}` - UPPER_CASE transformation (`MY_COMPONENT`)

#### **Context-Aware Smart Variables**
- `{{hook_name}}` - Intelligent hook naming (`useAuth` from `Auth`)
- `{{context_name}}` - Context naming with suffix (`AuthContext`)
- `{{provider_name}}` - Provider component naming (`AuthProvider`)
- `{{page_name}}` - Page component naming (`AuthPage`)

#### **🆕 Environment-Aware Variables (v1.2.3)**
- `{{environment}}` - Current environment (development/production/staging)
- `{{timestamp}}` - Current ISO 8601 timestamp (`2024-01-01T12:00:00.000Z`)
- `{{timestamp_iso}}` - ISO timestamp with milliseconds
- `{{date}}` - Current date (`2024-01-01`)
- `{{time}}` - Current time (`12:00:00`)
- `{{year}}` - Current year (`2024`)
- `{{uuid}}` - Generated UUID v4 (`550e8400-e29b-41d4-a716-446655440000`)
- `{{uuid_simple}}` - UUID without hyphens (`550e8400e29b41d4a716446655440000`)

#### **🆕 Generation Metadata Variables**
- `{{version}}` - CLI Frontend Generator version (`1.2.3`)
- `{{generator_name}}` - Generator name (`CLI Frontend Generator`)
- `{{generated}}` - Always `true` for generated files

#### **🆕 Template Configuration Variables**
Define custom variables in your template's `.conf` file:

```ini
# Template Configuration (.conf)
environment=production
var_api_version=v1
var_author=Frontend Team
var_license=MIT
```

Access in templates:
```typescript
// API Version: {{api_version}}
// Author: {{author}}  
// License: {{license}}
```

#### **Advanced Handlebars Variables**
```handlebars
{{#if (eq type "component")}}
// Component-specific code
{{/if}}

{{#unless (eq environment "production")}}
// Development-only code
{{/unless}}
```

#### **🆕 Advanced Handlebars Helpers (v1.2.3)**
```handlebars
{{timestamp format="ISO"}}        <!-- ISO 8601 format -->
{{timestamp format="date"}}       <!-- YYYY-MM-DD -->
{{timestamp format="time"}}       <!-- HH:MM:SS -->
{{uuid}}                          <!-- Full UUID v4 -->
{{env "NODE_ENV"}}                <!-- Environment variable -->
```

## 🆕 Template Configuration System

### Overview

CLI Frontend Generator v1.2.3 introduces a powerful template configuration system that allows templates to define their own variables, environment settings, and behavior through `.conf` files.

### Configuration File Structure

Create a `.conf` file in your template directory:

```
templates/
└── my-template/
    ├── .conf                    # ← Configuration file
    ├── $FILE_NAME.ts
    └── $FILE_NAME.test.ts
```

### Configuration Syntax

The `.conf` file uses INI-like format:

```ini
# Template Configuration for My Template
# This file defines variables and settings

# Environment configuration
environment=production
enable_timestamps=true
enable_uuid=true

# Custom variables (prefix with var_)
var_api_version=v2
var_author=Frontend Team
var_license=MIT
var_description=Advanced template with configuration
var_base_url=https://api.mycompany.com
var_timeout=5000
var_debug_mode=false
```

### Configuration Options

#### **Core Settings**
- `environment` - Environment name (development/production/staging)
- `enable_timestamps` - Include timestamp variables (true/false)
- `enable_uuid` - Include UUID variables (true/false)

#### **Custom Variables**
Prefix custom variables with `var_`:
- `var_api_version` → `{{api_version}}`
- `var_author` → `{{author}}`
- `var_license` → `{{license}}`
- Any `var_xyz` → `{{xyz}}`

### Using Configuration in Templates

```typescript
/**
 * {{pascal_name}} Template
 * {{description}}
 * 
 * @generated {{generated}}
 * @generator {{generator_name}} v{{version}}
 * @timestamp {{timestamp}}
 * @author {{author}}
 * @license {{license}}
 * @environment {{environment}}
 */

const API_CONFIG = {
  baseURL: '{{base_url}}/{{api_version}}',
  timeout: {{timeout}},
  debug: {{debug_mode}},
};

{{#if (eq environment "development")}}
console.log('Development mode configuration:', API_CONFIG);
{{/if}}

{{#if enable_timestamps}}
export const GENERATED_AT = '{{timestamp}}';
{{/if}}

{{#if enable_uuid}}
export const TEMPLATE_ID = '{{uuid}}';
{{/if}}
```

### Example: Complete Template with Configuration

#### 1. Template Configuration (`.conf`)
```ini
# Advanced API Client Template
environment=production
enable_timestamps=true
enable_uuid=true

# API Configuration
var_api_version=v1
var_base_url=https://jsonplaceholder.typicode.com
var_timeout=10000
var_retry_attempts=3

# Development settings
var_debug_mode=false
var_cache_enabled=true

# Metadata
var_author=API Team
var_license=MIT
var_description=Production-ready API client with retry logic
```

#### 2. Main Template File (`$FILE_NAME.client.ts`)
```typescript
/**
 * {{pascal_name}} API Client
 * {{description}}
 * 
 * @generated {{generated}}
 * @generator {{generator_name}} v{{version}}
 * @timestamp {{timestamp}}
 * @author {{author}}
 * @license {{license}}
 * @uuid {{uuid}}
 */

export interface {{pascal_name}}Config {
  baseURL: string;
  timeout: number;
  retryAttempts: number;
  debug: boolean;
  cacheEnabled: boolean;
}

const DEFAULT_CONFIG: {{pascal_name}}Config = {
  baseURL: '{{base_url}}/{{api_version}}',
  timeout: {{timeout}},
  retryAttempts: {{retry_attempts}},
  debug: {{debug_mode}},
  cacheEnabled: {{cache_enabled}},
};

export class {{pascal_name}}Client {
  private config: {{pascal_name}}Config;

  constructor(config: Partial<{{pascal_name}}Config> = {}) {
    this.config = { ...DEFAULT_CONFIG, ...config };
    
    {{#if debug_mode}}
    console.log('[{{pascal_name}}Client] Initialized with config:', this.config);
    {{/if}}
  }

  {{#if (eq environment "production")}}
  // Production-optimized methods
  {{else}}
  // Development methods with additional logging
  {{/if}}

  async getData<T>(endpoint: string): Promise<T> {
    {{#if debug_mode}}
    console.log(`[{{pascal_name}}Client] Fetching data from: ${endpoint}`);
    {{/if}}
    
    // Implementation here...
    
    {{#if enable_timestamps}}
    const requestTime = new Date().toISOString();
    {{/if}}
    
    return {} as T;
  }
}

{{#if enable_uuid}}
// Unique client instance ID
export const CLIENT_ID = '{{uuid_simple}}';
{{/if}}

{{#if enable_timestamps}}
// Template generation metadata
export const GENERATED_AT = '{{timestamp}}';
export const GENERATED_DATE = '{{date}}';
{{/if}}

// Environment information
export const ENVIRONMENT = '{{environment}}';
export const VERSION = '{{version}}';

// Default export
export default {{pascal_name}}Client;
```

### Benefits of Template Configuration

1. **🎯 Customizable Templates** - Each template can have unique behavior
2. **🔧 Environment Awareness** - Different behavior for dev/prod/staging  
3. **📝 Rich Variables** - Custom variables beyond basic name transformations
4. **🚀 Dynamic Content** - Conditional logic based on configuration
5. **📋 Self-Documenting** - Configuration serves as template documentation
6. **🔄 Reusable** - Share configurations across team projects

### Step 3: Production-Ready Example - Redux Store Template

Let's create a comprehensive Redux store template with modern patterns and TypeScript best practices:

#### 3.1 Create template structure
```bash
mkdir ~/.cli-frontend/templates/store
cd ~/.cli-frontend/templates/store
```

#### 3.2 Main store file (`$FILE_NAME.store.ts`)
```typescript
import { createSlice, PayloadAction, createSelector } from '@reduxjs/toolkit';
import { RootState } from '../store';

/**
 * {{pascal_name}} Store
 * Generated by CLI Frontend Generator
 * 
 * @description Redux store slice for {{pascal_name}} feature
 * @generated true
 */

// State interface for {{pascal_name}}
export interface {{pascal_name}}State {
  entities: Record<string, {{pascal_name}}Entity>;
  ids: string[];
  loading: boolean;
  error: string | null;
  lastUpdated: number | null;
  filters: {{pascal_name}}Filters;
}

// Initial state with proper typing
const initialState: {{pascal_name}}State = {
  entities: {},
  ids: [],
  loading: false,
  error: null,
  lastUpdated: null,
  filters: {
    search: '',
    status: 'all',
    sortBy: 'name',
    sortOrder: 'asc',
  },
};

// Redux Toolkit slice with comprehensive actions
export const {{camel_name}}Slice = createSlice({
  name: '{{kebab_name}}',
  initialState,
  reducers: {
    // Loading state management
    setLoading: (state, action: PayloadAction<boolean>) => {
      state.loading = action.payload;
      if (action.payload) {
        state.error = null;
      }
    },

    // Entity management with normalization
    setEntities: (state, action: PayloadAction<{{pascal_name}}Entity[]>) => {
      const entities: Record<string, {{pascal_name}}Entity> = {};
      const ids: string[] = [];
      
      action.payload.forEach((entity) => {
        entities[entity.id] = entity;
        ids.push(entity.id);
      });
      
      state.entities = entities;
      state.ids = ids;
      state.loading = false;
      state.error = null;
      state.lastUpdated = Date.now();
    },

    // Add single entity
    addEntity: (state, action: PayloadAction<{{pascal_name}}Entity>) => {
      const entity = action.payload;
      state.entities[entity.id] = entity;
      if (!state.ids.includes(entity.id)) {
        state.ids.push(entity.id);
      }
      state.lastUpdated = Date.now();
    },

    // Update entity with partial data
    updateEntity: (state, action: PayloadAction<{ id: string; changes: Partial<{{pascal_name}}Entity> }>) => {
      const { id, changes } = action.payload;
      if (state.entities[id]) {
        state.entities[id] = { ...state.entities[id], ...changes };
        state.lastUpdated = Date.now();
      }
    },

    // Remove entity
    removeEntity: (state, action: PayloadAction<string>) => {
      const id = action.payload;
      delete state.entities[id];
      state.ids = state.ids.filter(entityId => entityId !== id);
      state.lastUpdated = Date.now();
    },

    // Error handling
    setError: (state, action: PayloadAction<string>) => {
      state.error = action.payload;
      state.loading = false;
    },

    // Filter management
    setFilters: (state, action: PayloadAction<Partial<{{pascal_name}}Filters>>) => {
      state.filters = { ...state.filters, ...action.payload };
    },

    // Clear all data
    clearAll: () => initialState,
  },
});

// Action creators export
export const {
  setLoading,
  setEntities,
  addEntity,
  updateEntity,
  removeEntity,
  setError,
  setFilters,
  clearAll,
} = {{camel_name}}Slice.actions;

// Memoized selectors for performance
export const {{camel_name}}Selectors = {
  // Basic selectors
  selectState: (state: RootState) => state.{{camel_name}},
  selectEntities: (state: RootState) => state.{{camel_name}}.entities,
  selectIds: (state: RootState) => state.{{camel_name}}.ids,
  selectLoading: (state: RootState) => state.{{camel_name}}.loading,
  selectError: (state: RootState) => state.{{camel_name}}.error,
  selectFilters: (state: RootState) => state.{{camel_name}}.filters,

  // Computed selectors
  selectAllEntities: createSelector(
    [(state: RootState) => state.{{camel_name}}.entities, (state: RootState) => state.{{camel_name}}.ids],
    (entities, ids) => ids.map(id => entities[id])
  ),

  selectEntityById: createSelector(
    [(state: RootState) => state.{{camel_name}}.entities, (_: RootState, id: string) => id],
    (entities, id) => entities[id] || null
  ),

  selectFilteredEntities: createSelector(
    [
      (state: RootState) => state.{{camel_name}}.entities,
      (state: RootState) => state.{{camel_name}}.ids,
      (state: RootState) => state.{{camel_name}}.filters,
    ],
    (entities, ids, filters) => {
      let filtered = ids.map(id => entities[id]);

      // Apply search filter
      if (filters.search) {
        filtered = filtered.filter(entity =>
          entity.name.toLowerCase().includes(filters.search.toLowerCase())
        );
      }

      // Apply status filter
      if (filters.status !== 'all') {
        filtered = filtered.filter(entity => entity.status === filters.status);
      }

      // Apply sorting
      filtered.sort((a, b) => {
        const aValue = a[filters.sortBy as keyof {{pascal_name}}Entity];
        const bValue = b[filters.sortBy as keyof {{pascal_name}}Entity];
        const comparison = aValue < bValue ? -1 : aValue > bValue ? 1 : 0;
        return filters.sortOrder === 'asc' ? comparison : -comparison;
      });

      return filtered;
    }
  ),

  // Statistics selectors
  selectEntityCount: createSelector(
    [(state: RootState) => state.{{camel_name}}.ids],
    (ids) => ids.length
  ),

  selectHasData: createSelector(
    [(state: RootState) => state.{{camel_name}}.ids],
    (ids) => ids.length > 0
  ),
};

export default {{camel_name}}Slice.reducer;
```

#### 3.3 Type definitions (`$FILE_NAME.types.ts`)
```typescript
/**
 * {{pascal_name}} Type Definitions
 * Generated by CLI Frontend Generator
 */

// Core entity interface
export interface {{pascal_name}}Entity {
  id: string;
  name: string;
  status: '{{kebab_name}}-active' | '{{kebab_name}}-inactive' | '{{kebab_name}}-pending';
  createdAt: string;
  updatedAt: string;
  // Add domain-specific properties here
}

// API Response interfaces
export interface {{pascal_name}}ListResponse {
  data: {{pascal_name}}Entity[];
  total: number;
  page: number;
  limit: number;
  hasMore: boolean;
}

export interface {{pascal_name}}CreateRequest {
  name: string;
  // Add required creation fields
}

export interface {{pascal_name}}UpdateRequest {
  id: string;
  changes: Partial<Omit<{{pascal_name}}Entity, 'id' | 'createdAt' | 'updatedAt'>>;
}

// Filter and sorting interfaces
export interface {{pascal_name}}Filters {
  search: string;
  status: 'all' | '{{kebab_name}}-active' | '{{kebab_name}}-inactive' | '{{kebab_name}}-pending';
  sortBy: keyof {{pascal_name}}Entity;
  sortOrder: 'asc' | 'desc';
}

// Error interfaces
export interface {{pascal_name}}ApiError {
  code: string;
  message: string;
  details?: Record<string, any>;
}

// Hook return types
export interface Use{{pascal_name}}Return {
  entities: {{pascal_name}}Entity[];
  loading: boolean;
  error: string | null;
  filters: {{pascal_name}}Filters;
  actions: {
    fetch: (filters?: Partial<{{pascal_name}}Filters>) => Promise<void>;
    create: (data: {{pascal_name}}CreateRequest) => Promise<{{pascal_name}}Entity>;
    update: (data: {{pascal_name}}UpdateRequest) => Promise<{{pascal_name}}Entity>;
    remove: (id: string) => Promise<void>;
    setFilters: (filters: Partial<{{pascal_name}}Filters>) => void;
    clearError: () => void;
  };
}
```

#### 3.4 Async thunks (`$FILE_NAME.thunks.ts`)
```typescript
import { createAsyncThunk } from '@reduxjs/toolkit';
import { 
  {{pascal_name}}Entity, 
  {{pascal_name}}ListResponse, 
  {{pascal_name}}CreateRequest,
  {{pascal_name}}UpdateRequest,
  {{pascal_name}}Filters,
  {{pascal_name}}ApiError 
} from './{{kebab_name}}.types';

// Base API configuration
const API_BASE_URL = process.env.REACT_APP_API_URL || '/api';
const {{upper_name}}_ENDPOINT = `${API_BASE_URL}/{{kebab_name}}`;

// Utility function for API calls
const apiCall = async <T>(url: string, options: RequestInit = {}): Promise<T> => {
  const response = await fetch(url, {
    headers: {
      'Content-Type': 'application/json',
      ...options.headers,
    },
    ...options,
  });

  if (!response.ok) {
    const error: {{pascal_name}}ApiError = await response.json();
    throw new Error(error.message || `HTTP ${response.status}: ${response.statusText}`);
  }

  return response.json();
};

// Fetch entities with filtering and pagination
export const fetch{{pascal_name}}Entities = createAsyncThunk<
  {{pascal_name}}Entity[],
  Partial<{{pascal_name}}Filters> & { page?: number; limit?: number },
  { rejectValue: string }
>(
  '{{kebab_name}}/fetchEntities',
  async (params = {}, { rejectWithValue }) => {
    try {
      const searchParams = new URLSearchParams();
      Object.entries(params).forEach(([key, value]) => {
        if (value !== undefined && value !== null && value !== '') {
          searchParams.append(key, String(value));
        }
      });

      const url = `${{{upper_name}}_ENDPOINT}?${searchParams.toString()}`;
      const response = await apiCall<{{pascal_name}}ListResponse>(url);
      
      return response.data;
    } catch (error) {
      return rejectWithValue(error instanceof Error ? error.message : 'Failed to fetch {{kebab_name}} entities');
    }
  }
);

// Create new entity
export const create{{pascal_name}}Entity = createAsyncThunk<
  {{pascal_name}}Entity,
  {{pascal_name}}CreateRequest,
  { rejectValue: string }
>(
  '{{kebab_name}}/createEntity',
  async (createData, { rejectWithValue }) => {
    try {
      return await apiCall<{{pascal_name}}Entity>({{upper_name}}_ENDPOINT, {
        method: 'POST',
        body: JSON.stringify(createData),
      });
    } catch (error) {
      return rejectWithValue(error instanceof Error ? error.message : 'Failed to create {{kebab_name}} entity');
    }
  }
);

// Update existing entity
export const update{{pascal_name}}Entity = createAsyncThunk<
  {{pascal_name}}Entity,
  {{pascal_name}}UpdateRequest,
  { rejectValue: string }
>(
  '{{kebab_name}}/updateEntity',
  async ({ id, changes }, { rejectWithValue }) => {
    try {
      return await apiCall<{{pascal_name}}Entity>(`${{{upper_name}}_ENDPOINT}/${id}`, {
        method: 'PATCH',
        body: JSON.stringify(changes),
      });
    } catch (error) {
      return rejectWithValue(error instanceof Error ? error.message : 'Failed to update {{kebab_name}} entity');
    }
  }
);

// Delete entity
export const delete{{pascal_name}}Entity = createAsyncThunk<
  string,
  string,
  { rejectValue: string }
>(
  '{{kebab_name}}/deleteEntity',
  async (id, { rejectWithValue }) => {
    try {
      await apiCall(`${{{upper_name}}_ENDPOINT}/${id}`, {
        method: 'DELETE',
      });
      return id;
    } catch (error) {
      return rejectWithValue(error instanceof Error ? error.message : 'Failed to delete {{kebab_name}} entity');
    }
  }
);

// Bulk operations
export const bulk{{pascal_name}}Operation = createAsyncThunk<
  {{pascal_name}}Entity[],
  { operation: 'delete' | 'update'; ids: string[]; data?: Partial<{{pascal_name}}Entity> },
  { rejectValue: string }
>(
  '{{kebab_name}}/bulkOperation',
  async ({ operation, ids, data }, { rejectWithValue }) => {
    try {
      return await apiCall<{{pascal_name}}Entity[]>(`${{{upper_name}}_ENDPOINT}/bulk`, {
        method: 'POST',
        body: JSON.stringify({ operation, ids, data }),
      });
    } catch (error) {
      return rejectWithValue(error instanceof Error ? error.message : `Failed to perform bulk ${operation} operation`);
    }
  }
);
```

#### 3.5 Comprehensive test suite (`$FILE_NAME.store.test.ts`)
```typescript
import { configureStore } from '@reduxjs/toolkit';
import {{camel_name}}Reducer, {
  setLoading,
  setEntities,
  addEntity,
  updateEntity,
  removeEntity,
  setError,
  setFilters,
  clearAll,
  {{camel_name}}Selectors,
} from './{{kebab_name}}.store';
import { {{pascal_name}}Entity, {{pascal_name}}Filters } from './{{kebab_name}}.types';

// Mock data
const mockEntity: {{pascal_name}}Entity = {
  id: '1',
  name: 'Test {{pascal_name}}',
  status: '{{kebab_name}}-active',
  createdAt: '2024-01-01T00:00:00Z',
  updatedAt: '2024-01-01T00:00:00Z',
};

const mockEntities: {{pascal_name}}Entity[] = [
  mockEntity,
  {
    id: '2',
    name: 'Another {{pascal_name}}',
    status: '{{kebab_name}}-inactive',
    createdAt: '2024-01-02T00:00:00Z',
    updatedAt: '2024-01-02T00:00:00Z',
  },
];

describe('{{pascal_name}} Store', () => {
  let store: ReturnType<typeof configureStore>;

  beforeEach(() => {
    store = configureStore({
      reducer: {
        {{camel_name}}: {{camel_name}}Reducer,
      },
    });
  });

  describe('Actions', () => {
    it('should handle setLoading', () => {
      store.dispatch(setLoading(true));
      const state = store.getState().{{camel_name}};
      
      expect(state.loading).toBe(true);
      expect(state.error).toBe(null);
    });

    it('should handle setEntities', () => {
      store.dispatch(setEntities(mockEntities));
      const state = store.getState().{{camel_name}};
      
      expect(state.entities['1']).toEqual(mockEntity);
      expect(state.ids).toContain('1');
      expect(state.ids).toContain('2');
      expect(state.loading).toBe(false);
      expect(state.error).toBe(null);
      expect(state.lastUpdated).toBeTruthy();
    });

    it('should handle addEntity', () => {
      store.dispatch(addEntity(mockEntity));
      const state = store.getState().{{camel_name}};
      
      expect(state.entities['1']).toEqual(mockEntity);
      expect(state.ids).toContain('1');
      expect(state.lastUpdated).toBeTruthy();
    });

    it('should handle updateEntity', () => {
      store.dispatch(setEntities([mockEntity]));
      store.dispatch(updateEntity({ 
        id: '1', 
        changes: { name: 'Updated Name' } 
      }));
      
      const state = store.getState().{{camel_name}};
      expect(state.entities['1'].name).toBe('Updated Name');
      expect(state.lastUpdated).toBeTruthy();
    });

    it('should handle removeEntity', () => {
      store.dispatch(setEntities(mockEntities));
      store.dispatch(removeEntity('1'));
      
      const state = store.getState().{{camel_name}};
      expect(state.entities['1']).toBeUndefined();
      expect(state.ids).not.toContain('1');
      expect(state.ids).toContain('2');
    });

    it('should handle setError', () => {
      const errorMessage = 'Test error message';
      store.dispatch(setError(errorMessage));
      
      const state = store.getState().{{camel_name}};
      expect(state.error).toBe(errorMessage);
      expect(state.loading).toBe(false);
    });

    it('should handle setFilters', () => {
      const newFilters: Partial<{{pascal_name}}Filters> = {
        search: 'test',
        status: '{{kebab_name}}-active',
      };
      
      store.dispatch(setFilters(newFilters));
      const state = store.getState().{{camel_name}};
      
      expect(state.filters.search).toBe('test');
      expect(state.filters.status).toBe('{{kebab_name}}-active');
    });

    it('should handle clearAll', () => {
      store.dispatch(setEntities(mockEntities));
      store.dispatch(setError('Some error'));
      store.dispatch(clearAll());
      
      const state = store.getState().{{camel_name}};
      expect(state.entities).toEqual({});
      expect(state.ids).toEqual([]);
      expect(state.error).toBe(null);
      expect(state.loading).toBe(false);
    });
  });

  describe('Selectors', () => {
    beforeEach(() => {
      store.dispatch(setEntities(mockEntities));
    });

    it('should select all entities', () => {
      const state = store.getState();
      const entities = {{camel_name}}Selectors.selectAllEntities(state);
      
      expect(entities).toHaveLength(2);
      expect(entities[0]).toEqual(mockEntity);
    });

    it('should select entity by id', () => {
      const state = store.getState();
      const entity = {{camel_name}}Selectors.selectEntityById(state, '1');
      
      expect(entity).toEqual(mockEntity);
    });

    it('should select filtered entities', () => {
      store.dispatch(setFilters({ status: '{{kebab_name}}-active' }));
      const state = store.getState();
      const filtered = {{camel_name}}Selectors.selectFilteredEntities(state);
      
      expect(filtered).toHaveLength(1);
      expect(filtered[0].status).toBe('{{kebab_name}}-active');
    });

    it('should select entity count', () => {
      const state = store.getState();
      const count = {{camel_name}}Selectors.selectEntityCount(state);
      
      expect(count).toBe(2);
    });

    it('should select hasData', () => {
      const state = store.getState();
      const hasData = {{camel_name}}Selectors.selectHasData(state);
      
      expect(hasData).toBe(true);
    });
  });
});
```

### Step 4: CLI Integration (Optional)

For complete integration with the CLI's type system, update the template type enum in `src/cli.rs`:

```rust
#[derive(ValueEnum, Clone, Debug)]
pub enum TemplateType {
    Component,
    Hook, 
    Service,
    Context,
    Page,
    Store,  // ← Add your new template type here
    Api,
    Types,
}
```

### Step 5: Template Testing and Validation

```bash
# Test your new template
cli-frontend UserManagement --type store

# Verify generated structure
ls -la UserManagement/
# Expected output:
# UserManagement.store.ts
# UserManagement.types.ts
# UserManagement.thunks.ts
# UserManagement.store.test.ts
```

## 📋 Professional Development Standards

### 1. **File Naming Conventions**
```typescript
// Use descriptive, consistent naming
$FILE_NAME.store.ts           // Main implementation
$FILE_NAME.types.ts           // Type definitions
$FILE_NAME.test.ts            // Unit tests
$FILE_NAME.integration.test.ts // Integration tests
$FILE_NAME.stories.ts         // Storybook stories
$FILE_NAME.doc.md            // Documentation
```

### 2. **Template Directory Structure**
```
professional-template/
├── core/
│   └── $FILE_NAME.ts         # Core implementation
├── types/
│   └── $FILE_NAME.types.ts   # Type definitions
├── tests/
│   ├── $FILE_NAME.test.ts    # Unit tests
│   └── $FILE_NAME.integration.test.ts
├── docs/
│   └── $FILE_NAME.md         # Auto-generated documentation
└── stories/
    └── $FILE_NAME.stories.ts # Storybook integration
```

### 3. **Documentation Standards**
Include comprehensive documentation headers in templates:
```typescript
/**
 * {{pascal_name}} Template
 * Generated by CLI Frontend Generator v1.2.3
 * 
 * @description Production-ready {{template_type}} following {{architecture_pattern}} architecture
 * @author Generated on {{generation_date}}
 * @version 1.0.0
 * @generated true
 * 
 * @example
 * ```typescript
 * // Usage example for {{pascal_name}}
 * import { {{pascal_name}} } from './{{kebab_name}}';
 * 
 * const instance = new {{pascal_name}}();
 * ```
 */
```

### 4. **Production Configuration Variables**
```typescript
// Environment-aware configuration
const config = {
  API_BASE_URL: process.env.REACT_APP_API_URL || 'http://localhost:3000',
  {{upper_name}}_ENDPOINT: process.env.REACT_APP_{{upper_name}}_ENDPOINT || '/{{kebab_name}}',
  ENABLE_LOGGING: process.env.NODE_ENV === 'development',
  CACHE_TTL: parseInt(process.env.REACT_APP_CACHE_TTL || '300000'), // 5 minutes
  MAX_RETRIES: parseInt(process.env.REACT_APP_MAX_RETRIES || '3'),
};
```

## 🚀 Advanced Template Features

### Conditional Template Logic
```handlebars
{{#if (eq architecture "clean-architecture")}}
// Clean architecture specific imports
import { UseCase } from '../core/UseCase';
import { Repository } from '../core/Repository';
{{else if (eq architecture "mvc")}}
// MVC specific imports
import { Controller } from '../core/Controller';
import { Model } from '../core/Model';
{{/if}}

{{#unless (eq environment "production")}}
// Development-only code
console.log('Debug: {{pascal_name}} initialized');
{{/unless}}
```

### Custom Handlebars Helpers
```typescript
// Custom helper for generating timestamps
{{timestamp format="ISO"}}  // Outputs: 2024-01-01T00:00:00Z

// Custom helper for generating UUIDs
{{uuid}}  // Outputs: 550e8400-e29b-41d4-a716-446655440000

// Custom helper for environment checks
{{#if (env "development")}}
// Development code
{{/if}}
```

### Multi-Framework Support
```handlebars
{{#if (eq framework "react")}}
import React from 'react';
export const {{pascal_name}}: React.FC = () => {
{{else if (eq framework "vue")}}
import { defineComponent } from 'vue';
export default defineComponent({
  name: '{{pascal_name}}',
{{else if (eq framework "angular")}}
import { Component } from '@angular/core';
@Component({
  selector: '{{kebab_name}}',
{{/if}}
```

