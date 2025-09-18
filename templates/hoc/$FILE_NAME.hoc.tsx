/**
 * {{pascal_name}} Higher-Order Component
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

import React, { ComponentType{{#if use_ref}}, forwardRef{{/if}} } from 'react';

/**
 * Higher-Order Component that wraps a component with additional functionality
 *
 * @param WrappedComponent - The component to wrap
 * @returns Enhanced component with additional props and functionality
 */
export function {{pascal_name}}<P extends object>(
  WrappedComponent: ComponentType<P>
): ComponentType<P> {
  {{#if use_ref}}
  const Enhanced = forwardRef<any, P>((props, ref) => {
    // Add your HOC logic here
    // Example: logging, authentication, loading states, etc.

    return <WrappedComponent {...props} ref={ref} />;
  });

  Enhanced.displayName = `{{pascal_name}}($\{WrappedComponent.displayName || WrappedComponent.name})`;
  return Enhanced;
  {{else}}
  const Enhanced: ComponentType<P> = (props) => {
    // Add your HOC logic here
    // Example: logging, authentication, loading states, etc.

    return <WrappedComponent {...props} />;
  };

  Enhanced.displayName = `{{pascal_name}}($\{WrappedComponent.displayName || WrappedComponent.name})`;
  return Enhanced;
  {{/if}}
}

export default {{pascal_name}};