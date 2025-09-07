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