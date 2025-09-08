/**
 * {{pascal_name}} API Service
 * {{description}}
 * 
 * @generated {{generated}}
 * @generator {{generator_name}} v{{version}}
 * @timestamp {{timestamp}}
 * @author {{author}}
 * @license {{license}}
 * 
 * Environment: {{environment}}
 * API Version: {{api_version}}
 * Base URL: {{base_url}}
 * 
 * Features:
 * - Authentication: {{#if auth_required}}Required{{else}}Not required{{/if}}
 * - Rate Limiting: {{rate_limit}} requests/hour
 * - Caching: {{#if cache_enabled}}Enabled{{else}}Disabled{{/if}}
 * - Debug Mode: {{#if debug_mode}}Enabled{{else}}Disabled{{/if}}
 * 
 * @see {{swagger_url}}
 */

import axios, { AxiosInstance, AxiosRequestConfig, AxiosResponse } from 'axios';

// Types and interfaces
export interface {{pascal_name}}RequestConfig extends AxiosRequestConfig {
  skipAuth?: boolean;
  skipCache?: boolean;
  retries?: number;
}

export interface {{pascal_name}}Response<T = any> {
  data: T;
  success: boolean;
  message?: string;
  timestamp: string;
  requestId: string;
}

export interface {{pascal_name}}ErrorResponse {
  error: {
    code: string;
    message: string;
    details?: Record<string, any>;
  };
  timestamp: string;
  requestId: string;
}

export interface {{pascal_name}}Entity {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
}

// API Configuration
const API_CONFIG = {
  baseURL: '{{base_url}}/{{api_version}}',
  timeout: {{timeout}},
  retryAttempts: {{retry_attempts}},
  rateLimit: {{rate_limit}},
  cacheEnabled: {{cache_enabled}},
  debugMode: {{debug_mode}},
  authRequired: {{auth_required}},
} as const;

/**
 * {{pascal_name}} API Service Class
 * 
 * Provides a comprehensive API service for {{kebab_name}} operations
 * with built-in error handling, retry logic, and caching capabilities.
 * 
 * @example
 * ```typescript
 * const {{camel_name}}Service = new {{pascal_name}}Service();
 * const entities = await {{camel_name}}Service.getAll();
 * ```
 */
export class {{pascal_name}}Service {
  private readonly client: AxiosInstance;
  private readonly cache = new Map<string, { data: any; timestamp: number }>();
  private requestCount = 0;
  private lastReset = Date.now();

  constructor(config?: Partial<typeof API_CONFIG>) {
    const finalConfig = { ...API_CONFIG, ...config };

    this.client = axios.create({
      baseURL: finalConfig.baseURL,
      timeout: finalConfig.timeout,
      headers: {
        'Content-Type': 'application/json',
        'X-Service-Version': '{{version}}',
        'X-Generated-At': '{{timestamp}}',
        'X-Request-ID': '{{uuid}}',
        {{#if auth_required}}
        'Authorization': `Bearer ${this.getAuthToken()}`,
        {{/if}}
      },
    });

    this.setupInterceptors();
    this.logInitialization();
  }

  /**
   * Setup axios request and response interceptors
   */
  private setupInterceptors(): void {
    // Request interceptor
    this.client.interceptors.request.use(
      (config) => {
        this.checkRateLimit();
        
        {{#if debug_mode}}
        console.log(`[{{pascal_name}}Service] Request:`, {
          method: config.method?.toUpperCase(),
          url: config.url,
          timestamp: new Date().toISOString(),
        });
        {{/if}}

        return config;
      },
      (error) => {
        {{#if debug_mode}}
        console.error(`[{{pascal_name}}Service] Request Error:`, error);
        {{/if}}
        return Promise.reject(error);
      }
    );

    // Response interceptor
    this.client.interceptors.response.use(
      (response) => {
        {{#if debug_mode}}
        console.log(`[{{pascal_name}}Service] Response:`, {
          status: response.status,
          url: response.config.url,
          timestamp: new Date().toISOString(),
        });
        {{/if}}

        return response;
      },
      async (error) => {
        {{#if debug_mode}}
        console.error(`[{{pascal_name}}Service] Response Error:`, error);
        {{/if}}

        // Implement retry logic
        const config = error.config as {{pascal_name}}RequestConfig;
        const retries = config.retries ?? API_CONFIG.retryAttempts;

        if (retries > 0 && this.shouldRetry(error)) {
          config.retries = retries - 1;
          await this.delay(1000 * (API_CONFIG.retryAttempts - retries + 1));
          return this.client.request(config);
        }

        return Promise.reject(error);
      }
    );
  }

  /**
   * Get all {{kebab_name}} entities
   */
  async getAll(): Promise<{{pascal_name}}Entity[]> {
    const cacheKey = 'getAll';
    
    if (this.shouldUseCache(cacheKey)) {
      return this.getFromCache(cacheKey);
    }

    try {
      const response = await this.client.get<{{pascal_name}}Response<{{pascal_name}}Entity[]>>(
        '/{{kebab_name}}'
      );

      const entities = response.data.data;
      this.setCache(cacheKey, entities);
      
      return entities;
    } catch (error) {
      throw this.handleError(error);
    }
  }

  /**
   * Get {{kebab_name}} entity by ID
   */
  async getById(id: string): Promise<{{pascal_name}}Entity> {
    const cacheKey = `getById:${id}`;
    
    if (this.shouldUseCache(cacheKey)) {
      return this.getFromCache(cacheKey);
    }

    try {
      const response = await this.client.get<{{pascal_name}}Response<{{pascal_name}}Entity>>(
        `/{{kebab_name}}/${id}`
      );

      const entity = response.data.data;
      this.setCache(cacheKey, entity);
      
      return entity;
    } catch (error) {
      throw this.handleError(error);
    }
  }

  /**
   * Create new {{kebab_name}} entity
   */
  async create(data: Omit<{{pascal_name}}Entity, 'id' | 'createdAt' | 'updatedAt'>): Promise<{{pascal_name}}Entity> {
    try {
      const response = await this.client.post<{{pascal_name}}Response<{{pascal_name}}Entity>>(
        '/{{kebab_name}}',
        data
      );

      // Invalidate cache
      this.clearCache();
      
      return response.data.data;
    } catch (error) {
      throw this.handleError(error);
    }
  }

  /**
   * Update existing {{kebab_name}} entity
   */
  async update(id: string, data: Partial<{{pascal_name}}Entity>): Promise<{{pascal_name}}Entity> {
    try {
      const response = await this.client.put<{{pascal_name}}Response<{{pascal_name}}Entity>>(
        `/{{kebab_name}}/${id}`,
        data
      );

      // Invalidate cache
      this.clearCache();
      
      return response.data.data;
    } catch (error) {
      throw this.handleError(error);
    }
  }

  /**
   * Delete {{kebab_name}} entity
   */
  async delete(id: string): Promise<void> {
    try {
      await this.client.delete(`/{{kebab_name}}/${id}`);
      
      // Invalidate cache
      this.clearCache();
    } catch (error) {
      throw this.handleError(error);
    }
  }

  // Utility methods
  {{#if auth_required}}
  private getAuthToken(): string {
    const token = localStorage.getItem('auth_token') || process.env.API_TOKEN;
    if (!token) {
      throw new Error('Authentication token not found');
    }
    return token;
  }
  {{/if}}

  private shouldRetry(error: any): boolean {
    return error.response?.status >= 500 || error.code === 'NETWORK_ERROR';
  }

  private delay(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  private checkRateLimit(): void {
    const now = Date.now();
    const timeWindow = 60 * 60 * 1000; // 1 hour

    if (now - this.lastReset > timeWindow) {
      this.requestCount = 0;
      this.lastReset = now;
    }

    if (this.requestCount >= API_CONFIG.rateLimit) {
      throw new Error('Rate limit exceeded');
    }

    this.requestCount++;
  }

  private shouldUseCache(key: string): boolean {
    if (!API_CONFIG.cacheEnabled) return false;
    
    const cached = this.cache.get(key);
    if (!cached) return false;
    
    const cacheTimeout = 5 * 60 * 1000; // 5 minutes
    return Date.now() - cached.timestamp < cacheTimeout;
  }

  private getFromCache<T>(key: string): T {
    const cached = this.cache.get(key);
    return cached?.data;
  }

  private setCache(key: string, data: any): void {
    if (!API_CONFIG.cacheEnabled) return;
    
    this.cache.set(key, {
      data,
      timestamp: Date.now(),
    });
  }

  private clearCache(): void {
    this.cache.clear();
  }

  private handleError(error: any): Error {
    if (error.response) {
      const errorResponse: {{pascal_name}}ErrorResponse = error.response.data;
      return new Error(
        `{{pascal_name}} API Error: ${errorResponse.error.message} (${errorResponse.error.code})`
      );
    }
    
    return new Error(`{{pascal_name}} Service Error: ${error.message}`);
  }

  private logInitialization(): void {
    {{#if debug_mode}}
    console.log(`[{{pascal_name}}Service] Initialized with config:`, {
      baseURL: API_CONFIG.baseURL,
      environment: '{{environment}}',
      timestamp: '{{timestamp}}',
      version: '{{version}}',
      features: {
        auth: API_CONFIG.authRequired,
        cache: API_CONFIG.cacheEnabled,
        debug: API_CONFIG.debugMode,
      },
    });
    {{/if}}
  }
}

// Export singleton instance
export const {{camel_name}}Service = new {{pascal_name}}Service();

// Export default
export default {{pascal_name}}Service;