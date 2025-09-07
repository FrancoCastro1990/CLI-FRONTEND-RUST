/**
 * {{pascal_name}} API Service
 * Generado automáticamente por CLI Frontend Generator
 */

export class {{pascal_name}}Api {
  private readonly baseUrl: string;

  constructor(baseUrl: string = '/api/{{kebab_name}}') {
    this.baseUrl = baseUrl;
  }

  /**
   * Obtener todos los {{pascal_name}}
   */
  async getAll(): Promise<{{pascal_name}}[]> {
    const response = await fetch(this.baseUrl);
    if (!response.ok) {
      throw new Error(`Failed to fetch {{kebab_name}}: ${response.statusText}`);
    }
    return response.json();
  }

  /**
   * Obtener {{pascal_name}} por ID
   */
  async getById(id: string | number): Promise<{{pascal_name}}> {
    const response = await fetch(`${this.baseUrl}/${id}`);
    if (!response.ok) {
      throw new Error(`Failed to fetch {{kebab_name}} ${id}: ${response.statusText}`);
    }
    return response.json();
  }

  /**
   * Crear nuevo {{pascal_name}}
   */
  async create(data: Create{{pascal_name}}Request): Promise<{{pascal_name}}> {
    const response = await fetch(this.baseUrl, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    });
    
    if (!response.ok) {
      throw new Error(`Failed to create {{kebab_name}}: ${response.statusText}`);
    }
    return response.json();
  }

  /**
   * Actualizar {{pascal_name}}
   */
  async update(id: string | number, data: Update{{pascal_name}}Request): Promise<{{pascal_name}}> {
    const response = await fetch(`${this.baseUrl}/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data),
    });
    
    if (!response.ok) {
      throw new Error(`Failed to update {{kebab_name}} ${id}: ${response.statusText}`);
    }
    return response.json();
  }

  /**
   * Eliminar {{pascal_name}}
   */
  async delete(id: string | number): Promise<void> {
    const response = await fetch(`${this.baseUrl}/${id}`, {
      method: 'DELETE',
    });
    
    if (!response.ok) {
      throw new Error(`Failed to delete {{kebab_name}} ${id}: ${response.statusText}`);
    }
  }
}

// Tipos de interfaz
export interface {{pascal_name}} {
  id: string | number;
  // Agregar propiedades específicas aquí
}

export interface Create{{pascal_name}}Request {
  // Agregar campos requeridos para crear
}

export interface Update{{pascal_name}}Request {
  // Agregar campos para actualizar
}

// Instancia por defecto
export const {{camel_name}}Api = new {{pascal_name}}Api();