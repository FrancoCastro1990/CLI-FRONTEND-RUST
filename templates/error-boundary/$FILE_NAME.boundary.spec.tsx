/**
 * {{pascal_name}} Error Boundary Tests
 *
 * @generated {{generated}}
 * @timestamp {{timestamp}}
 */

import React from 'react';
import { render, screen, fireEvent } from '@testing-library/react';
import '@testing-library/jest-dom';
import { {{pascal_name}} } from './{{kebab_name}}.boundary';

// Component that throws an error for testing
const ThrowError: React.FC<{ shouldThrow?: boolean }> = ({ shouldThrow = false }) => {
  if (shouldThrow) {
    throw new Error('Test error');
  }
  return <div>No error</div>;
};

// Mock console.error to avoid noise in tests
const originalError = console.error;
beforeAll(() => {
  console.error = jest.fn();
});

afterAll(() => {
  console.error = originalError;
});

describe('{{pascal_name}} Error Boundary', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('renders children when there is no error', () => {
    render(
      <{{pascal_name}}>
        <div data-testid="child">Child content</div>
      </{{pascal_name}}>
    );

    expect(screen.getByTestId('child')).toBeInTheDocument();
    expect(screen.getByText('Child content')).toBeInTheDocument();
  });

  it('renders default error UI when child throws error', () => {
    render(
      <{{pascal_name}}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    expect(screen.getByText('{{fallback_message}}')).toBeInTheDocument();
    expect(screen.getByText('Try Again')).toBeInTheDocument();
  });

  it('renders custom fallback when provided', () => {
    const customFallback = <div data-testid="custom-fallback">Custom Error UI</div>;

    render(
      <{{pascal_name}} fallback={customFallback}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    expect(screen.getByTestId('custom-fallback')).toBeInTheDocument();
    expect(screen.getByText('Custom Error UI')).toBeInTheDocument();
    expect(screen.queryByText('{{fallback_message}}')).not.toBeInTheDocument();
  });

  it('calls onError callback when error occurs', () => {
    const onError = jest.fn();

    render(
      <{{pascal_name}} onError={onError}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    expect(onError).toHaveBeenCalledTimes(1);
    expect(onError).toHaveBeenCalledWith(
      expect.any(Error),
      expect.objectContaining({
        componentStack: expect.any(String)
      })
    );
  });

  {{#if enable_logging}}
  it('logs error to console when logging is enabled', () => {
    render(
      <{{pascal_name}}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    expect(console.error).toHaveBeenCalledWith(
      '[{{pascal_name}}] Error caught:',
      expect.any(Error)
    );
    expect(console.error).toHaveBeenCalledWith(
      '[{{pascal_name}}] Error info:',
      expect.any(Object)
    );
  });
  {{/if}}

  it('resets error state when retry button is clicked', () => {
    const TestComponent: React.FC<{ shouldThrow: boolean }> = ({ shouldThrow }) => (
      <ThrowError shouldThrow={shouldThrow} />
    );

    const { rerender } = render(
      <{{pascal_name}}>
        <TestComponent shouldThrow={true} />
      </{{pascal_name}}>
    );

    // Error UI should be visible
    expect(screen.getByText('{{fallback_message}}')).toBeInTheDocument();

    // Click retry button
    fireEvent.click(screen.getByText('Try Again'));

    // Re-render with working component
    rerender(
      <{{pascal_name}}>
        <TestComponent shouldThrow={false} />
      </{{pascal_name}}>
    );

    // Should show normal content again
    expect(screen.getByText('No error')).toBeInTheDocument();
    expect(screen.queryByText('{{fallback_message}}')).not.toBeInTheDocument();
  });

  it('has accessible retry button', () => {
    render(
      <{{pascal_name}}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    const retryButton = screen.getByRole('button', { name: 'Try Again' });
    expect(retryButton).toBeInTheDocument();
    expect(retryButton).toBeEnabled();
  });

  {{#if show_error_details}}
  it('shows error details when configured', () => {
    render(
      <{{pascal_name}}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    expect(screen.getByText('Error Details')).toBeInTheDocument();
    expect(screen.getByText('Test error')).toBeInTheDocument();
  });
  {{/if}}

  it('handles multiple errors correctly', () => {
    const { rerender } = render(
      <{{pascal_name}}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    // First error
    expect(screen.getByText('{{fallback_message}}')).toBeInTheDocument();

    // Reset
    fireEvent.click(screen.getByText('Try Again'));

    // Second error with different component
    rerender(
      <{{pascal_name}}>
        <ThrowError shouldThrow={true} />
      </{{pascal_name}}>
    );

    expect(screen.getByText('{{fallback_message}}')).toBeInTheDocument();
  });
});