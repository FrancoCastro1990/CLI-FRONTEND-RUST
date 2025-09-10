//! # Naming Module
//!
//! Simple and efficient name transformations for React components using
//! prefix/suffix utilities and standard case conversions.

/// Smart processed names for React patterns
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcessedNames {
    pub hook_name: String,
    pub context_name: String,
    pub provider_name: String,
    pub page_name: String,
}

/// Name transformation service for React patterns
#[derive(Debug, Default)]
pub struct SmartNaming;

impl SmartNaming {
    /// Create a new SmartNaming instance
    pub fn new() -> Self {
        Self
    }

    /// Process names with smart patterns for React components
    pub fn process_smart_names(&self, name: &str) -> ProcessedNames {
        let name_lower = name.to_lowercase();

        // For hook names, check if it already has the semantic "use" prefix
        let hook_name = if name_lower.starts_with("use") && name.len() > 3 {
            // Check if the 4th character is uppercase (indicating semantic boundary)
            if name.chars().nth(3).is_some_and(char::is_uppercase) {
                name.to_string() // Keep as is, like "useAuth"
            } else {
                // It's something like "user", add proper prefix
                let pascal_name = self.to_pascal_case(name);
                format!("use{}", pascal_name)
            }
        } else {
            let pascal_name = self.to_pascal_case(name);
            format!("use{}", pascal_name)
        };

        let pascal_name = self.to_pascal_case(name);

        ProcessedNames {
            hook_name,
            context_name: self.ensure_suffix(&pascal_name, "Context"),
            provider_name: self.ensure_suffix(&pascal_name, "Provider"),
            page_name: self.ensure_suffix(&pascal_name, "Page"),
        }
    }

    /// Ensure a string has a specific prefix
    /// Ensure a string has a specific suffix
    /// If it already has the suffix (case insensitive), keep the original case
    /// If not, add the suffix
    pub fn ensure_suffix(&self, text: &str, suffix: &str) -> String {
        let text_lower = text.to_lowercase();
        let suffix_lower = suffix.to_lowercase();

        if text_lower.ends_with(&suffix_lower) {
            // Already has suffix, return as is
            text.to_string()
        } else {
            // Add suffix
            format!("{}{}", text, suffix)
        }
    }

    /// Convert string to PascalCase using simple word splitting
    pub fn to_pascal_case(&self, s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }

        // Check if it's already in a camelCase/PascalCase format
        // (contains uppercase letters but no separators)
        if s.chars().any(|c| c.is_uppercase()) && !s.chars().any(|c| !c.is_alphanumeric()) {
            // It's camelCase/PascalCase, just capitalize the first letter
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        } else {
            // Split on separators and process each word
            let words: Vec<&str> =
                s.split(|c: char| !c.is_alphanumeric()).filter(|word| !word.is_empty()).collect();

            if words.is_empty() {
                String::new()
            } else {
                words
                    .iter()
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => {
                                first.to_uppercase().collect::<String>()
                                    + &chars.as_str().to_lowercase()
                            },
                        }
                    })
                    .collect()
            }
        }
    }

    /// Convert string to camelCase
    pub fn to_camel_case(&self, s: &str) -> String {
        let pascal = self.to_pascal_case(s);
        if pascal.is_empty() {
            return pascal;
        }

        let mut chars = pascal.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
        }
    }

    /// Convert string to snake_case
    pub fn to_snake_case(&self, s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }

        // Handle camelCase/PascalCase by inserting underscores before uppercase letters
        let mut result = String::new();

        for ch in s.chars() {
            if ch.is_uppercase() && !result.is_empty() {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap_or(ch));
        }

        // Also handle other separators by replacing them with underscores
        result
            .split(|c: char| !c.is_alphanumeric())
            .filter(|word| !word.is_empty())
            .collect::<Vec<_>>()
            .join("_")
    }

    /// Convert string to kebab-case
    pub fn to_kebab_case(&self, s: &str) -> String {
        self.to_snake_case(s).replace('_', "-")
    }

    /// Apply smart replacements to content
    ///
    /// This replaces template placeholders with smart names:
    /// - `use$FILE_NAME` → hook name
    /// - `$FILE_NAMEContext` → context name  
    /// - `$FILE_NAMEProvider` → provider name
    /// - `$FILE_NAMEPage` → page name
    /// - `$FILE_NAME` → original name
    pub fn apply_smart_replacements(
        &self,
        content: &str,
        name: &str,
        smart_names: &ProcessedNames,
    ) -> String {
        let mut result = content.to_string();

        // Replace specific patterns with smart names
        result = result.replace("use$FILE_NAME", &smart_names.hook_name);
        result = result.replace("$FILE_NAMEContext", &smart_names.context_name);
        result = result.replace("$FILE_NAMEProvider", &smart_names.provider_name);
        result = result.replace("$FILE_NAMEPage", &smart_names.page_name);

        // Replace remaining $FILE_NAME with original name
        result = result.replace("$FILE_NAME", name);

        result
    }

    /// Apply smart filename replacements
    ///
    /// This processes template filenames with smart names:
    /// - `use$FILE_NAME.ts` → `useUser.ts`
    /// - `$FILE_NAMEContext.tsx` → `UserContext.tsx`
    pub fn apply_smart_filename_replacements(
        &self,
        filename: &str,
        name: &str,
        smart_names: &ProcessedNames,
    ) -> String {
        let mut result = filename.to_string();

        // Replace specific patterns in filenames first
        result = result.replace("use$FILE_NAME", &smart_names.hook_name);
        result = result.replace("$FILE_NAMEContext", &smart_names.context_name);
        result = result.replace("$FILE_NAMEProvider", &smart_names.provider_name);
        result = result.replace("$FILE_NAMEPage", &smart_names.page_name);

        // Replace remaining $FILE_NAME with PascalCase name
        result = result.replace("$FILE_NAME", &self.to_pascal_case(name));

        result
    }

    /// Process filename pattern with smart replacements
    ///
    /// Used for architecture-based filename generation
    pub fn process_filename_pattern(&self, pattern: &str, name: &str) -> String {
        let smart_names = self.process_smart_names(name);

        let mut result = pattern.to_string();

        // Replace specific patterns
        result = result.replace("use{name}", &smart_names.hook_name);
        result = result.replace("{name}Context", &smart_names.context_name);
        result = result.replace("{name}Provider", &smart_names.provider_name);
        result = result.replace("{name}Page", &smart_names.page_name);

        // Replace remaining {name}
        result = result.replace("{name}", name);

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smart_names_processing() {
        let naming = SmartNaming::new();
        let result = naming.process_smart_names("user");

        assert_eq!(result.hook_name, "useUser");
        assert_eq!(result.context_name, "UserContext");
        assert_eq!(result.provider_name, "UserProvider");
        assert_eq!(result.page_name, "UserPage");
    }

    #[test]
    fn test_smart_names_with_existing_patterns() {
        let naming = SmartNaming::new();

        // Test hook name that already starts with "use"
        let result = naming.process_smart_names("useAuth");
        assert_eq!(result.hook_name, "useAuth");

        // Test name that already ends with "Context"
        let result = naming.process_smart_names("AuthContext");
        assert_eq!(result.context_name, "AuthContext");
    }

    #[test]
    fn test_case_conversions() {
        let naming = SmartNaming::new();

        assert_eq!(naming.to_pascal_case("user_profile"), "UserProfile");
        assert_eq!(naming.to_pascal_case("my-component"), "MyComponent");
        assert_eq!(naming.to_camel_case("UserProfile"), "userProfile");
        assert_eq!(naming.to_snake_case("UserProfile"), "user_profile");
        assert_eq!(naming.to_kebab_case("UserProfile"), "user-profile");
    }

    #[test]
    fn test_smart_replacements() {
        let naming = SmartNaming::new();
        let smart_names = naming.process_smart_names("user");

        let content = "export const use$FILE_NAME = () => {}; export const $FILE_NAMEContext = {};";
        let result = naming.apply_smart_replacements(content, "user", &smart_names);

        assert!(result.contains("useUser"));
        assert!(result.contains("UserContext"));
    }

    #[test]
    fn test_filename_pattern_processing() {
        let naming = SmartNaming::new();

        let pattern = "use{name}.ts";
        let result = naming.process_filename_pattern(pattern, "auth");
        assert_eq!(result, "useAuth.ts");

        let pattern = "{name}Context.tsx";
        let result = naming.process_filename_pattern(pattern, "user");
        assert_eq!(result, "UserContext.tsx");
    }

    #[test]
    fn test_ensure_suffix() {
        let naming = SmartNaming::new();

        // Should add suffix when missing
        assert_eq!(naming.ensure_suffix("User", "Context"), "UserContext");

        // Should keep existing suffix
        assert_eq!(naming.ensure_suffix("AuthContext", "Context"), "AuthContext");
        assert_eq!(naming.ensure_suffix("Authcontext", "Context"), "Authcontext");
        // Keep original case
    }
}
