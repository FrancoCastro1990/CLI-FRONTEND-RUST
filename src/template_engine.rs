use anyhow::{Context, Result};
use handlebars::{Handlebars, Helper, HelperResult, Output, RenderContext};
use serde_json::json;
use std::path::{Path, PathBuf};
use tokio::fs;
use walkdir::WalkDir;
use colored::*;

#[derive(Debug)]
struct SmartNames {
    hook_name: String,
    context_name: String,
    provider_name: String,
    page_name: String,
}

pub struct TemplateEngine {
    templates_dir: PathBuf,
    output_dir: PathBuf,
    // Removed handlebars field since we create it per-file for better flexibility
}

impl TemplateEngine {
    pub fn new(templates_dir: PathBuf, output_dir: PathBuf) -> Result<Self> {
        Ok(Self {
            templates_dir,
            output_dir,
        })
    }
    
    pub fn template_exists(&self, template_type: &str) -> bool {
        self.templates_dir.join(template_type).exists()
    }
    
    pub fn list_templates(&self) -> Result<Vec<String>> {
        let mut templates = Vec::new();
        
        if !self.templates_dir.exists() {
            return Ok(templates);
        }
        
        for entry in std::fs::read_dir(&self.templates_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    if !name.starts_with('.') {
                        templates.push(name.to_string());
                    }
                }
            }
        }
        
        templates.sort();
        Ok(templates)
    }
    
    pub async fn generate(&self, name: &str, template_type: &str, create_folder: bool) -> Result<()> {
        let template_dir = self.templates_dir.join(template_type);
        
        if !template_dir.exists() {
            return Err(anyhow::anyhow!("Template directory not found: {}", template_dir.display()));
        }
        
        // Determine output path
        let output_path = if create_folder {
            self.output_dir.join(name)
        } else {
            self.output_dir.clone()
        };
        
        // Create output directory
        fs::create_dir_all(&output_path).await
            .with_context(|| format!("Could not create output directory: {}", output_path.display()))?;
        
        // Process all template files
        self.process_template_directory(&template_dir, &output_path, name).await?;
        
        // Show generated files
        self.show_generated_files(&output_path).await?;
        
        Ok(())
    }
    
    async fn process_template_directory(&self, template_dir: &Path, output_path: &Path, name: &str) -> Result<()> {
        let mut tasks = Vec::new();
        
        // Walk through all files in template directory
        for entry in WalkDir::new(template_dir) {
            let entry = entry.context("Error walking template directory")?;
            
            if entry.file_type().is_file() {
                let relative_path = entry.path().strip_prefix(template_dir)
                    .context("Could not get relative path")?;
                
                let template_file = entry.path().to_path_buf();
                let output_file = output_path.join(relative_path);
                
                // Process file asynchronously
                let name_clone = name.to_string();
                let task = tokio::spawn(async move {
                    Self::process_template_file(&template_file, &output_file, &name_clone).await
                });
                
                tasks.push(task);
            }
        }
        
        // Wait for all files to be processed
        for task in tasks {
            task.await??;
        }
        
        Ok(())
    }
    
    async fn process_template_file(template_file: &Path, output_file: &Path, name: &str) -> Result<()> {
        // Read template content
        let template_content = fs::read_to_string(template_file).await
            .with_context(|| format!("Could not read template file: {}", template_file.display()))?;
        
        // Create handlebars instance for this file
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("pascal_case", Box::new(pascal_case_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
        handlebars.register_helper("camel_case", Box::new(camel_case_helper));
        handlebars.register_helper("upper_case", Box::new(upper_case_helper));
        
        // Smart name processing for different patterns
        let processed_names = Self::process_smart_names(name);
        
        // Prepare template data
        let data = json!({
            "name": name,
            "pascal_name": to_pascal_case(name),
            "snake_name": to_snake_case(name),
            "kebab_name": to_kebab_case(name),
            "camel_name": to_camel_case(name),
            "upper_name": name.to_uppercase(),
            // Smart processed names
            "hook_name": processed_names.hook_name,
            "context_name": processed_names.context_name,
            "provider_name": processed_names.provider_name,
            "page_name": processed_names.page_name,
        });
        
        // Apply smart replacements
        let processed_content = Self::apply_smart_replacements(&template_content, name, &processed_names);
        
        // Render with handlebars for more advanced templating
        let rendered_content = handlebars.render_template(&processed_content, &data)
            .with_context(|| "Template rendering failed")?;
        
        // Process output filename with smart replacements
        let output_filename = output_file.file_name()
            .and_then(|n| n.to_str())
            .map(|n| Self::apply_smart_filename_replacements(n, name, &processed_names))
            .context("Invalid output filename")?;
        
        let final_output_path = output_file.parent()
            .context("Invalid output path")?
            .join(output_filename);
        
        // Create parent directories if needed
        if let Some(parent) = final_output_path.parent() {
            fs::create_dir_all(parent).await
                .with_context(|| format!("Could not create parent directory: {}", parent.display()))?;
        }
        
        // Write processed content
        fs::write(&final_output_path, rendered_content).await
            .with_context(|| format!("Could not write output file: {}", final_output_path.display()))?;
        
        Ok(())
    }
    
    // Smart name processing for different patterns
    fn process_smart_names(name: &str) -> SmartNames {
        let name_lower = name.to_lowercase();
        
        // Hook name processing
        let hook_name = if name_lower.starts_with("use") {
            name.to_string()
        } else {
            format!("use{}", to_pascal_case(name))
        };
        
        // Context name processing
        let context_name = if name_lower.ends_with("context") {
            name.to_string()
        } else {
            format!("{}Context", to_pascal_case(name))
        };
        
        // Provider name processing  
        let provider_name = if name_lower.ends_with("provider") {
            name.to_string()
        } else {
            let base_name = if name_lower.ends_with("context") {
                // Remove "Context" suffix if present
                let without_context = &name[..name.len()-7];
                to_pascal_case(without_context)
            } else {
                to_pascal_case(name)
            };
            format!("{}Provider", base_name)
        };
        
        // Page name processing
        let page_name = if name_lower.ends_with("page") {
            name.to_string()
        } else {
            format!("{}Page", to_pascal_case(name))
        };
        
        SmartNames {
            hook_name,
            context_name,
            provider_name,
            page_name,
        }
    }
    
    // Apply smart content replacements
    fn apply_smart_replacements(content: &str, name: &str, smart_names: &SmartNames) -> String {
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
    
    // Apply smart filename replacements
    fn apply_smart_filename_replacements(filename: &str, name: &str, smart_names: &SmartNames) -> String {
        let mut result = filename.to_string();
        
        // Replace specific patterns in filenames
        result = result.replace("use$FILE_NAME", &smart_names.hook_name);
        result = result.replace("$FILE_NAMEContext", &smart_names.context_name);
        result = result.replace("$FILE_NAMEProvider", &smart_names.provider_name);
        result = result.replace("$FILE_NAMEPage", &smart_names.page_name);
        
        // Replace remaining $FILE_NAME
        result = result.replace("$FILE_NAME", name);
        
        result
    }
    
    async fn show_generated_files(&self, output_path: &Path) -> Result<()> {
        let mut files = Vec::new();
        
        for entry in WalkDir::new(output_path).max_depth(1) {
            let entry = entry.context("Error reading output directory")?;
            
            if entry.file_type().is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    files.push(filename.to_string());
                }
            }
        }
        
        if !files.is_empty() {
            println!("{}", "Files created:".bold());
            for file in files {
                println!("  - {}", file.green());
            }
        }
        
        Ok(())
    }
}

// Helper functions for naming conventions
fn pascal_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_pascal_case(value))?;
        }
    }
    Ok(())
}

fn snake_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_snake_case(value))?;
        }
    }
    Ok(())
}

fn kebab_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_kebab_case(value))?;
        }
    }
    Ok(())
}

fn camel_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&to_camel_case(value))?;
        }
    }
    Ok(())
}

fn upper_case_helper(
    h: &Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        if let Some(value) = param.value().as_str() {
            out.write(&value.to_uppercase())?;
        }
    }
    Ok(())
}

// Utility functions for case conversions
fn to_pascal_case(s: &str) -> String {
    s.split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars.as_str().to_lowercase().chars()).collect(),
            }
        })
        .collect()
}

fn to_camel_case(s: &str) -> String {
    let pascal = to_pascal_case(s);
    if pascal.is_empty() {
        return pascal;
    }
    
    let mut chars = pascal.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().chain(chars.as_str().chars()).collect(),
    }
}

fn to_snake_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if c.is_uppercase() && i > 0 {
                vec!['_', c.to_lowercase().next().unwrap_or(c)]
            } else {
                vec![c.to_lowercase().next().unwrap_or(c)]
            }
        })
        .collect::<String>()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn to_kebab_case(s: &str) -> String {
    to_snake_case(s).replace('_', "-")
}