import React from "react";
{{#if style_is_scss}}
import styles from "./$FILE_NAME.module.scss";
{{/if}}
{{#if style_is_styled_components}}
import { Styled$FILE_NAME } from "./$FILE_NAME.styled";
{{/if}}

/**
 * Props interface for $FILE_NAME component
 * @author {{author}}
 * @generated {{date}}
 */
export interface $FILE_NAMEProps {
  // Add your props here
  className?: string;
  children?: React.ReactNode;
}

/**
 * $FILE_NAME Component
 *
 * Generated with style: {{style}}
 * Tests included: {{with_tests}}
 * Storybook included: {{with_stories}}
 */
export const $FILE_NAME: React.FC<$FILE_NAMEProps> = ({
  className = "",
  children,
  ...props
}) => {
{{#if style_is_styled_components}}
  return (
    <Styled$FILE_NAME className={className} {...props}>
      {children || <div>$FILE_NAME Component</div>}
    </Styled$FILE_NAME>
  );
{{else}}
{{#if style_is_scss}}
  return (
    <div className={`${styles.$FILE_NAME} ${className}`.trim()} {...props}>
      {children || <div>$FILE_NAME Component</div>}
    </div>
  );
{{else}}
  return (
    <div className={className} {...props}>
      {children || <div>$FILE_NAME Component</div>}
    </div>
  );
{{/if}}
{{/if}}
};

export default $FILE_NAME;
