import { createAsyncThunk } from '@reduxjs/toolkit';
import { {{pascal_name}}Response, {{pascal_name}}Filters, {{pascal_name}}Item } from './{{kebab_name}}.types';

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