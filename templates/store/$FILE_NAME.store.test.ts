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