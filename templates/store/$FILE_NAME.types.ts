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