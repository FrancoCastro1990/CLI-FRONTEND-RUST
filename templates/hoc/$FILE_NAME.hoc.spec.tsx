/**
 * {{pascal_name}} HOC Tests
 *
 * @generated {{generated}}
 * @timestamp {{timestamp}}
 */

import React from 'react';
import { render, screen } from '@testing-library/react';
import '@testing-library/jest-dom';
import { {{pascal_name}} } from './{{kebab_name}}.hoc';

// Test component to wrap
const TestComponent: React.FC<{ message: string }> = ({ message }) => (
  <div data-testid="test-component">{message}</div>
);

TestComponent.displayName = 'TestComponent';

describe('{{pascal_name}} HOC', () => {
  const WrappedComponent = {{pascal_name}}(TestComponent);

  it('renders the wrapped component', () => {
    render(<WrappedComponent message="Hello World" />);
    expect(screen.getByTestId('test-component')).toBeInTheDocument();
    expect(screen.getByText('Hello World')).toBeInTheDocument();
  });

  it('passes props to the wrapped component', () => {
    const testMessage = 'Test message';
    render(<WrappedComponent message={testMessage} />);
    expect(screen.getByText(testMessage)).toBeInTheDocument();
  });

  it('sets correct display name', () => {
    expect(WrappedComponent.displayName).toBe('{{pascal_name}}(TestComponent)');
  });

  {{#if use_ref}}
  it('forwards ref to the wrapped component', () => {
    const ref = React.createRef<HTMLDivElement>();
    const RefTestComponent = React.forwardRef<HTMLDivElement, { message: string }>(
      ({ message }, forwardedRef) => (
        <div ref={forwardedRef} data-testid="ref-test-component">
          {message}
        </div>
      )
    );

    const WrappedRefComponent = {{pascal_name}}(RefTestComponent);
    render(<WrappedRefComponent ref={ref} message="Ref test" />);

    expect(ref.current).toBeInstanceOf(HTMLDivElement);
    expect(ref.current).toHaveTextContent('Ref test');
  });
  {{/if}}

  it('handles components without display name', () => {
    const AnonymousComponent: React.FC<{ text: string }> = ({ text }) => (
      <span>{text}</span>
    );

    const WrappedAnonymous = {{pascal_name}}(AnonymousComponent);
    expect(WrappedAnonymous.displayName).toBe('{{pascal_name}}(AnonymousComponent)');
  });

  it('preserves component functionality', () => {
    const ClickableComponent: React.FC<{ onClick: () => void; label: string }> = ({
      onClick,
      label,
    }) => (
      <button onClick={onClick} data-testid="clickable">
        {label}
      </button>
    );

    const WrappedClickable = {{pascal_name}}(ClickableComponent);
    const mockClick = jest.fn();

    render(<WrappedClickable onClick={mockClick} label="Click me" />);

    const button = screen.getByTestId('clickable');
    button.click();

    expect(mockClick).toHaveBeenCalledTimes(1);
    expect(screen.getByText('Click me')).toBeInTheDocument();
  });
});