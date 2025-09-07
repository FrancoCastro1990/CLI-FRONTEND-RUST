import { createContext, useContext } from "react";

/**
 * Interface for $FILE_NAME context state
 * TODO: Define your context state structure
 */
export interface $FILE_NAMEContextState {
  // Add your context properties here
  // Example:
  // user: User | null;
  // isLoading: boolean;
  // error: string | null;
}

/**
 * Interface for $FILE_NAME context actions
 * TODO: Define your context actions/methods
 */
export interface $FILE_NAMEContextActions {
  // Add your context methods here  
  // Example:
  // login: (credentials: LoginCredentials) => Promise<void>;
  // logout: () => void;
  // updateUser: (user: Partial<User>) => void;
  // clearError: () => void;
}

/**
 * Combined context interface
 */
export interface $FILE_NAMEContextValue extends $FILE_NAMEContextState, $FILE_NAMEContextActions {}

/**
 * Default context value
 * TODO: Set appropriate default values
 */
const defaultContextValue: $FILE_NAMEContextValue = {
  // Add default state values here
  // Example:
  // user: null,
  // isLoading: false,
  // error: null,
  
  // Add default method implementations here
  // Example:
  // login: async () => { throw new Error('$FILE_NAMEProvider not found'); },
  // logout: () => { throw new Error('$FILE_NAMEProvider not found'); },
  // updateUser: () => { throw new Error('$FILE_NAMEProvider not found'); },
  // clearError: () => { throw new Error('$FILE_NAMEProvider not found'); },
};

/**
 * $FILE_NAME Context
 */
export const $FILE_NAMEContext = createContext<$FILE_NAMEContextValue>(defaultContextValue);

/**
 * Custom hook to use $FILE_NAME context
 * @returns $FILE_NAMEContextValue
 */
export const use$FILE_NAME = (): $FILE_NAMEContextValue => {
  const context = useContext($FILE_NAMEContext);
  
  if (!context) {
    throw new Error('use$FILE_NAME must be used within a $FILE_NAMEProvider');
  }
  
  return context;
};

export default $FILE_NAMEContext;
