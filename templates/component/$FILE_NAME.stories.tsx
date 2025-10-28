import React from 'react';
import type { Meta, StoryObj } from '@storybook/react';
import { $FILE_NAME } from './$FILE_NAME';

/**
 * $FILE_NAME Storybook Stories
 * @author {{author}}
 * Generated: {{date}}
 */

const meta: Meta<typeof $FILE_NAME> = {
  title: 'Components/$FILE_NAME',
  component: $FILE_NAME,
  parameters: {
    layout: 'centered',
    docs: {
      description: {
        component: 'A description of the $FILE_NAME component.',
      },
    },
  },
  tags: ['autodocs'],
  argTypes: {
    className: {
      control: 'text',
      description: 'Additional CSS class names',
    },
    children: {
      control: 'text',
      description: 'Content to render inside the component',
    },
  },
};

export default meta;
type Story = StoryObj<typeof $FILE_NAME>;

/**
 * Default story showing the component in its basic state
 */
export const Default: Story = {
  args: {
    children: 'Default $FILE_NAME Content',
  },
};

/**
 * Story with custom className
 */
export const WithCustomClass: Story = {
  args: {
    className: 'custom-class',
    children: 'Component with custom class',
  },
};

/**
 * Story with no children (empty state)
 */
export const Empty: Story = {
  args: {},
};

/**
 * Story with complex children
 */
export const WithComplexChildren: Story = {
  args: {
    children: (
      <div>
        <h3>Complex Content</h3>
        <p>This is a more complex example with nested elements.</p>
      </div>
    ),
  },
};

/**
 * Interactive playground story
 */
export const Playground: Story = {
  args: {
    children: 'Playground - Modify props in controls panel',
  },
};
