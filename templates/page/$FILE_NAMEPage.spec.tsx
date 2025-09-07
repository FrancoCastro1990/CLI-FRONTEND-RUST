import React from 'react';
import { render, screen } from '@testing-library/react';
import { $FILE_NAMEPage } from './$FILE_NAMEPage';

describe('$FILE_NAMEPage', () => {
  it('renders without crashing', () => {
    render(<$FILE_NAMEPage />);
    expect(screen.getByRole('heading', { level: 1 })).toBeInTheDocument();
  });

  it('displays the default title', () => {
    render(<$FILE_NAMEPage />);
    expect(screen.getByText('$FILE_NAME')).toBeInTheDocument();
  });

  it('displays custom title when provided', () => {
    const customTitle = 'Custom Page Title';
    render(<$FILE_NAMEPage title={customTitle} />);
    expect(screen.getByText(customTitle)).toBeInTheDocument();
  });

  it('applies custom className', () => {
    const customClass = 'custom-page-class';
    render(<$FILE_NAMEPage className={customClass} />);
    const pageElement = screen.getByRole('heading', { level: 1 }).closest('div');
    expect(pageElement).toHaveClass(customClass);
  });

  it('renders welcome message', () => {
    render(<$FILE_NAMEPage />);
    expect(screen.getByText(/Welcome to/)).toBeInTheDocument();
    expect(screen.getByText(/This is the.*page/)).toBeInTheDocument();
  });
});