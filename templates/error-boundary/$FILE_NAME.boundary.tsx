/**
 * {{pascal_name}} Error Boundary Component
 * {{description}}
 *
 * @generated {{generated}}
 * @generator {{generator_name}} v{{version}}
 * @timestamp {{timestamp}}
 * @author {{author}}
 * @license {{license}}
 *
 * Environment: {{environment}}
 */

import React, { Component, ReactNode } from 'react';

interface {{pascal_name}}Props {
  children: ReactNode;
  fallback?: ReactNode;
  onError?: (error: Error, errorInfo: React.ErrorInfo) => void;
}

interface {{pascal_name}}State {
  hasError: boolean;
  error?: Error;
}

/**
 * Error Boundary Component
 *
 * Catches JavaScript errors anywhere in the child component tree and
 * displays a fallback UI instead of crashing the entire application.
 */
export class {{pascal_name}} extends Component<{{pascal_name}}Props, {{pascal_name}}State> {
  constructor(props: {{pascal_name}}Props) {
    super(props);
    this.state = { hasError: false };
  }

  static getDerivedStateFromError(error: Error): {{pascal_name}}State {
    return { hasError: true, error };
  }

  componentDidCatch(error: Error, errorInfo: React.ErrorInfo) {
    {{#if enable_logging}}
    // Log error details
    console.error('[{{pascal_name}}] Error caught:', error);
    console.error('[{{pascal_name}}] Error info:', errorInfo);
    {{/if}}

    // Call optional error handler
    this.props.onError?.(error, errorInfo);
  }

  private handleRetry = () => {
    this.setState({ hasError: false, error: undefined });
  };

  render() {
    if (this.state.hasError) {
      // Custom fallback UI
      if (this.props.fallback) {
        return this.props.fallback;
      }

      // Default fallback UI
      return (
        <div className="error-boundary">
          <h2>{{fallback_message}}</h2>
          {{#if show_error_details}}
          {this.state.error && (
            <details style=\{\{ marginTop: '1rem' }}>
              <summary>Error Details</summary>
              <pre style=\{\{
                background: '#f5f5f5',
                padding: '1rem',
                borderRadius: '4px',
                overflow: 'auto'
              }}>
                {this.state.error.message}
                {'\n'}
                {this.state.error.stack}
              </pre>
            </details>
          )}
          {{/if}}
          <button
            onClick={this.handleRetry}
            style=\{\{
              marginTop: '1rem',
              padding: '0.5rem 1rem',
              backgroundColor: '#007bff',
              color: 'white',
              border: 'none',
              borderRadius: '4px',
              cursor: 'pointer'
            }}
          >
            Try Again
          </button>
        </div>
      );
    }

    return this.props.children;
  }
}

export default {{pascal_name}};