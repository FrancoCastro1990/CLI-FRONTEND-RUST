# CLI Frontend Generator 🚀

**Professional code generation for scalable frontend architectu## 🚀 Usage Examples

### 🧙‍♂️ Interactive Wizard Mode (NEW!)
```bash
# Simply run without arguments to start the interactive wizard
cli-frontend

# The wizard will guide you through:
# 1. Select what to generate (Template or Feature)
# 2. Choose specific template or architecture
# 3. Enter name with validation
# 4. Configure options (folder creation, output directory)
# 5. Review and confirm
```

> 💡 **Pro tip**: Press `ESC` at any time to cancel the wizard gracefully

### Quick Start - Individual Components**

A powerful Rust-based CLI tool that generates production-ready React components, hooks, services, and complete feature structures following established software architecture patterns. Designed for development teams who value consistency, maintainability, and architectural best practices.

[![Version](https://img.shields.io/badge/version-1.3.0-blue.svg)](https://github.com/FrancoCastro1990/cli-frontend-rust/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://rustup.rs)

> 🎉 **Version 1.3.0** - NEW: Interactive Wizard Mode! Enhanced with 12 software architectures, guided UI experience, and intelligent cross-platform installers

## 🎯 Why CLI Frontend Generator?

### For Development Teams
- **Eliminate architectural inconsistencies** across your codebase
- **Accelerate developer onboarding** with standardized patterns
- **Scale your frontend architecture** from startup to enterprise
- **Reduce code review overhead** with consistent structure generation

### For Technical Leaders
- **Enforce architectural standards** automatically across teams
- **Choose from 12 proven architectures** based on project requirements  
- **Customize and extend** templates for your specific tech stack
- **Measure developer productivity** gains through consistent tooling

### For Consultancies & Agencies
- **Rapid project setup** with client-specific architectural patterns
- **Consistent quality delivery** across multiple projects
- **Extensible template system** for reusable project scaffolding

## ⚡ Key Features

### �️ **12 Software Architecture Patterns**
Generate complete features following proven architectural patterns:
- **Clean Architecture** - Enterprise-grade layered architecture
- **Redux Pattern** - Predictable state management with selectors and middleware
- **Flux Architecture** - Unidirectional data flow with centralized dispatcher
- **Micro-frontends** - Independent deployable frontend modules
- **Atomic Design** - Systematic component hierarchy
- **Event-Driven** - Loosely coupled reactive systems
- **MVC/MVP/MVVM** - Traditional presentation patterns
- [**+ 5 more architectures**](./docs/ARCHITECTURES_GUIDE.md) with detailed documentation

### 🎯 **Production-Ready Code Generation**
- **TypeScript-first** - Full type safety and IntelliSense support
- **SCSS Modules** - Scoped styling with CSS module patterns
- **Automated Testing** - Jest/Testing Library test files included
- **React Best Practices** - Hooks, functional components, modern patterns

### 🔧 **Developer Experience Excellence**
- **🧙‍♂️ Interactive Wizard Mode** - Guided experience when no arguments provided
- **🔍 Template Discovery** - `--describe` command shows detailed template information
- **Zero configuration** - Intelligent installers handle setup automatically
- **Template extensibility** - Create custom templates without CLI recompilation
- **Multiple naming conventions** - PascalCase, camelCase, snake_case, kebab-case
- **Dynamic discovery** - Auto-detection of templates and architectures
- **Cross-platform** - Native support for Windows, macOS, and Linux

### � **Team Scalability**
- **Consistent code structure** across all team members
- **Architectural documentation** - Each pattern includes benefits and trade-offs
- **Customizable conventions** - Adapt to your team's specific requirements
- **Version controlled templates** - Share templates across teams and projects

## 🚀 Quick Installation

### One-Command Installation

**Linux/macOS:**
```bash
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install.sh | bash
```

**Windows (PowerShell as Administrator):**
```powershell
iwr -useb https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install-windows.ps1 | iex
```

> ✅ **Smart Installers**: Automatically download precompiled binaries or compile from source. Create configuration files and add CLI to system PATH.

### Manual Installation

**From Source:**
```bash
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust
cargo build --release
# Binary available at target/release/cli-frontend
```

**Verify Installation:**
```bash
cli-frontend --help
```

## � Usage Examples

### Quick Start - Individual Components
```bash
# Generate a React component with default settings
cli-frontend Button --type component

# Generate component with SCSS styles
cli-frontend Button --type component --var style=scss

# Generate component with styled-components and tests
cli-frontend Modal --type component --var style=styled-components --var with_tests=true

# Generate component without tests
cli-frontend Header --type component --var with_tests=false

# Generate a custom hook
cli-frontend useAuth --type hook

# Generate an API service
cli-frontend UserAPI --type api

# Generate a Higher-Order Component
cli-frontend withAuth --type hoc

# Generate an Error Boundary
cli-frontend AppErrorBoundary --type error-boundary

# Generate without folder creation
cli-frontend Modal --type component --no-folder
```

### Architecture-Driven Development
```bash
# Generate feature with default architecture (Screaming Architecture)
cli-frontend UserAuth --type feature

# Generate feature with Clean Architecture
cli-frontend PaymentSystem --type feature --architecture clean-architecture

# Generate feature with Redux pattern
cli-frontend ShoppingCart --type feature --architecture redux

# Generate feature with Atomic Design
cli-frontend DesignSystem --type feature --architecture atomic-design
```

### Team Standardization
```bash
# Use specific output directory
cli-frontend Header --type component --output-dir ./src/components

# Use team-specific configuration
cli-frontend Dashboard --type feature --config ./team-config.conf

# List all available architectures and templates
cli-frontend --help
```

### Template Discovery & Exploration
```bash
# List all available templates and architectures
cli-frontend --list

# Explore a template before using it
cli-frontend --describe component

# See what variables are available for hooks
cli-frontend --describe hook

# Check API service template options
cli-frontend --describe api-service
```

**Example `--describe` Output:**
```
📋 Template: component
==================================================

Description:
  Functional component with TypeScript

Template Variables (use --var):

  --var style=<value>
    Options: scss, styled-components, css, none
    Default: scss
    Description: Styling approach for the component

  --var with_tests=<value>
    Type: boolean
    Default: true
    Description: Include unit tests for the component

Files Generated:
  ✓ ComponentName.tsx (always)
  ○ ComponentName.module.scss (--var style=scss)
  ○ ComponentName.spec.tsx (--var with_tests=true)
  ○ ComponentName.styled.ts (--var style=styled-components)

Usage Examples:

  # Basic (with defaults)
  cli-frontend ComponentName --type component

  # With styled-components
  cli-frontend ComponentName --type component --var style=styled-components

  # Without tests
  cli-frontend ComponentName --type component --var with_tests=false
```

## 🏗️ Software Architecture Patterns

**Generate complete features following proven architectural patterns:**

| Architecture | Best For | Generated Structure |
|--------------|----------|-------------------|
| **Clean Architecture** | Enterprise applications requiring high testability | Domain → Application → Infrastructure → Presentation |
| **Redux** | Complex state management with predictable updates | Store → Actions → Reducers → Selectors → Middleware |
| **Flux** | Unidirectional data flow applications | Actions → Dispatcher → Stores → Views |
| **Micro-frontends** | Large teams with independent deployment needs | Shell → Components → Services → API |
| **Atomic Design** | Design system development | Atoms → Molecules → Organisms → Templates → Pages |
| **Event-Driven** | Real-time applications with loose coupling | Events → Publishers → Listeners → Components |
| **MVC/MVP/MVVM** | Traditional applications with clear separation | Models → Views → Controllers/Presenters |

> 📖 **Comprehensive Guide**: See [ARCHITECTURES_GUIDE.md](./docs/ARCHITECTURES_GUIDE.md) for detailed documentation, benefits, limitations, and usage scenarios for each architecture.

### Example: Clean Architecture Feature
```bash
cli-frontend PaymentSystem --type feature --architecture clean-architecture
```

**Generated Structure:**
```
PaymentSystem/
├── domain/
│   ├── entities/PaymentSystemEntity.ts
│   └── repositories/PaymentSystemRepository.ts
├── application/
│   └── usecases/PaymentSystemUseCase.ts
├── infrastructure/
│   └── services/PaymentSystemService.ts
├── presentation/
│   ├── components/PaymentSystem.tsx
│   └── hooks/usePaymentSystem.ts
└── types.ts
```

## 📋 Command Reference

### Interactive Wizard Mode
```bash
# Start interactive wizard (no arguments)
cli-frontend
```

### Direct Command Mode
```bash
cli-frontend [name] [OPTIONS]

Arguments:
  <name>                      Name of the template or feature to generate

Options:
  -t, --type <TYPE>           Template type to generate (auto-detection available)
  -a, --architecture <ARCH>   Architecture pattern for features (mvc, clean-architecture, etc.)
  --var <KEY=VALUE>           Template variables (can be used multiple times)
  --no-folder                 Generate files without creating a parent folder
  -o, --output-dir <DIR>      Output directory for generated files
  -c, --config <CONFIG>       Path to custom configuration file
  --list                      List all available templates and architectures
  --describe <TEMPLATE>       Show detailed information about a template
  --help                      Display help, available templates and architectures
```

> 📖 **Detailed Wizard Guide**: See [WIZARD_GUIDE.md](./docs/WIZARD_GUIDE.md) for comprehensive documentation on using the interactive wizard.

### 🔍 Exploring Templates

Before generating code, you can explore template details including available variables, file generation rules, and usage examples:

```bash
# Show detailed information about a template
cli-frontend --describe component

# Explore hook template
cli-frontend --describe hook

# View all templates
cli-frontend --list
```

**What `--describe` shows:**
- 📄 Template description and metadata
- ⚙️ Available variables with types and default values
- 📁 Files that will be generated (conditional and always)
- 💡 Usage examples with different configurations

### Available Templates

| Template | Description | Generated Files |
|----------|-------------|----------------|
| `component` | React component with TypeScript | `.tsx`, `.module.scss`, `.spec.tsx` |
| `hook` | Custom React hook | `.ts`, `.test.ts` |
| `service` | Business logic service | `.ts` |
| `context` | React context with provider | `Context.tsx`, `Provider.tsx` |
| `page` | Page component with routing | `.tsx`, `.module.scss`, `.spec.tsx` |
| `store` | Redux store slice | `.store.ts`, `.types.ts`, `.thunks.ts`, `.test.ts` |
| `api` | API service interface | `.api.ts` |
| `api-service` | **Advanced API service with config** | `.service.ts`, `.test.ts`, `.conf` |
| `hoc` | **Higher-Order Component with generics** | `.hoc.tsx`, `.hoc.spec.tsx` |
| `error-boundary` | **Error Boundary for error handling** | `.boundary.tsx`, `.boundary.spec.tsx` |

> 🔧 **Extensible**: Add custom templates by creating folders in the `templates/` directory. The CLI automatically discovers new templates without recompilation.

### 🎯 **Template Configuration & Dynamic Variables**

Templates support advanced configuration through `.conf` files and runtime variables via the `--var` flag.

#### Using the `--var` Flag

Pass runtime variables to customize template generation:

```bash
# Generate component with SCSS styling
cli-frontend Button --type component --var style=scss

# Generate component with styled-components and tests
cli-frontend Modal --type component --var style=styled-components --var with_tests=true

# Generate component without tests and stories
cli-frontend Header --type component --var with_tests=false --var with_stories=false

# Multiple variables for custom templates
cli-frontend Dashboard --type my-template --var theme=dark --var layout=grid --var with_api=true
```

**Key Features:**
- **Conditional File Generation**: Only generate files when conditions match
- **Dynamic Template Content**: Templates adapt based on variable values
- **Boolean Helpers**: Automatic generation of `var_is_value` helpers for conditionals
- **CLI Override**: CLI variables override `.conf` defaults

#### Template `.conf` File Format

Configure template behavior and define variables in `.conf` files:

```ini
# templates/component/.conf

[metadata]
name=React Component
description=Functional component with TypeScript

[options]
# Styling approach
style=scss
style_description=Styling approach for the component
style_options=scss,styled-components,css,none

# Testing
with_tests=true
with_tests_description=Include unit tests
with_tests_type=boolean

# Storybook
with_stories=false
with_stories_description=Generate Storybook stories
with_stories_type=boolean

[files]
$FILE_NAME.tsx=always
$FILE_NAME.spec.tsx=var_with_tests
$FILE_NAME.module.scss=var_style_scss
$FILE_NAME.styled.ts=var_style_styled_components
$FILE_NAME.stories.tsx=var_with_stories
```

**Sections:**
- `[metadata]`: Template information
- `[options]`: Variable definitions with metadata (`_options`, `_type`, `_description`)
- `[files]`: Conditional file generation rules (`always`, `default`, `var_X`, `var_X_value`)

> 📖 **Complete Guide**: See [TEMPLATE_GUIDE.md](./docs/TEMPLATE_GUIDE.md) for comprehensive `.conf` format documentation and template development guide.

## ⚙️ Configuration

The CLI searches for configuration files in this order:
1. `.cli-frontend.conf` in current directory
2. `~/.cli-frontend.conf` in home directory  
3. File specified with `--config` option

### 🐧 **Linux Installation Note**
For proper operation on Linux systems, the configuration file **must use absolute paths**. The installation scripts automatically create the correct configuration:

**User installation** (via `install-quick.sh`):
```ini
templates_dir=/home/username/.cli-template/templates
architectures_dir=/home/username/.cli-template/architectures
```

### Configuration Example (cli-frontend.conf)
```ini
# CLI Frontend Generator Configuration

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Path configuration (use absolute paths on Linux)
templates_dir=/home/franco/.cli-template/templates
output_dir=.
architectures_dir=/home/franco/.cli-template/architectures

# Architecture settings
default_architecture=screaming-architecture
```

### 🔧 **Troubleshooting Configuration Issues**

If the installer didn't create the configuration file, or you're experiencing template detection issues, manually create the configuration:

```bash
# Create configuration file manually
cat > ~/.cli-frontend.conf << 'EOF'
# CLI Frontend Generator Configuration

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Path configuration (replace with your actual username)
templates_dir=/home/$(whoami)/.cli-template/templates
output_dir=.
architectures_dir=/home/$(whoami)/.cli-template/architectures

# Architecture settings
default_architecture=screaming-architecture
EOF
```

**Verify your installation:**
```bash
# Check if directories exist
ls -la ~/.cli-template/
ls -la ~/.cli-template/templates/
ls -la ~/.cli-template/architectures/

# Check configuration file
cat ~/.cli-frontend.conf

# Test CLI detection
cli-frontend --list
```

## 🔧 Technical Architecture

### Project Structure
```
cli-frontend-rust/
├── src/
│   ├── main.rs              # Application entry point
│   ├── cli.rs               # CLI argument parsing with dynamic detection
│   ├── config.rs            # Configuration and architecture management
│   ├── template_engine.rs   # Handlebars template engine with features
│   └── error.rs             # Comprehensive error handling
├── templates/               # Auto-discovered template library
│   ├── component/           # React component templates
│   ├── hook/                # Custom hook templates
│   ├── service/             # Business logic templates
│   ├── api-service/         # Advanced API service with .conf
│   └── [custom-templates]/  # User-defined templates
├── architectures/           # Software architecture configurations
│   ├── clean-architecture.json
│   ├── redux.json
│   ├── flux.json
│   └── [12 total architectures]
└── docs/                    # Comprehensive documentation
```

## 🐛 Troubleshooting

### Templates Not Detected
**Problem**: `Error: Unknown type 'my-template'. Available types: ...`

**Solution**: Ensure your configuration file uses absolute paths:
```bash
# Check current configuration
cat ~/.cli-frontend.conf

# Should show absolute paths like:
templates_dir=/home/username/.cli-template/templates
```

**Fix**: Edit `~/.cli-frontend.conf` with absolute paths or reinstall:
```bash
# Quick fix
sed -i "s|templates_dir=.*|templates_dir=$HOME/.cli-template/templates|g" ~/.cli-frontend.conf
sed -i "s|architectures_dir=.*|architectures_dir=$HOME/.cli-template/architectures|g" ~/.cli-frontend.conf

# Or reinstall
curl -sSL https://github.com/FrancoCastro1990/CLI-FRONTEND-RUST/releases/latest/download/install.sh | bash
```

### CLI Not Found After Installation
**Problem**: `command not found: cli-frontend`

**Solutions**:
```bash
# Reload shell
source ~/.zshrc # or ~/.bashrc  


### Template Configuration Issues
**Problem**: Custom variables not working in templates

**Solution**: Ensure template has `.conf` file with proper format:
```ini
# templates/my-template/.conf
var_author=Your Name
var_version=1.0.0
enable_timestamps=true
```

### Template System
- **Handlebars Engine** - Powerful templating with conditional logic
- **Dynamic Discovery** - Auto-detection of new templates without recompilation
- **Variable Interpolation** - Support for multiple naming conventions
- **Conditional Rendering** - Template logic based on configuration

### Naming Convention Support
```typescript
// Available template variables:
{{pascal_name}}     // PascalCase (MyComponent)
{{camel_name}}      // camelCase (myComponent)  
{{snake_name}}      // snake_case (my_component)
{{kebab_name}}      // kebab-case (my-component)
{{upper_name}}      // UPPER_CASE (MY_COMPONENT)
{{hook_name}}       // Intelligent hook naming (useMyComponent)
{{context_name}}    // Intelligent context naming (MyComponentContext)
```

## 🛠️ Technology Stack

### Core Technologies
- **Rust** - Systems programming language for performance and safety
- **Clap** - Command-line argument parsing with auto-completion support
- **Handlebars** - Logic-enabled templating engine
- **Tokio** - Asynchronous runtime for concurrent operations
- **Serde** - Serialization framework for configuration management
- **Anyhow** - Context-aware error handling

### Development Features
- **Cross-platform** - Native binaries for Windows, macOS, and Linux
- **Zero dependencies** - Single binary with no runtime requirements
- **Fast execution** - Rust performance with minimal startup time
- **Memory safe** - No runtime errors or memory leaks

## 📈 Performance Metrics

- **Template Generation**: < 50ms per component
- **Feature Scaffolding**: < 200ms for complete architecture
- **Memory Usage**: < 10MB peak memory consumption
- **Binary Size**: ~8MB optimized release binary
- **Cold Start**: < 100ms from command execution to completion



## 🎨 Creating Custom Templates

One of the most powerful features is **automatic template discovery**. Create custom templates without modifying the source code:

### Custom Template Creation Steps:

1. **Create a folder** in `templates/` with your template name:
   ```bash
   mkdir templates/my-template
   ```

2. **Add template files** using Handlebars variables:
   ```typescript
   // templates/my-template/$FILE_NAME.ts
   export class {{pascal_name}} {
     // Your custom code here
   }
   ```

3. **Use immediately** - No recompilation needed:
   ```bash
   cli-frontend MyElement --type my-template
   ```

### Available Template Variables:
- `$FILE_NAME` and `{{name}}` - Original name
- `{{pascal_name}}` - PascalCase (MyComponent)
- `{{camel_name}}` - camelCase (myComponent)
- `{{kebab_name}}` - kebab-case (my-component)
- `{{snake_name}}` - snake_case (my_component)
- `{{upper_name}}` - UPPER_CASE (MY_COMPONENT)

#### **NEW: Environment-Aware Variables**
- `{{environment}}` - Current environment (development/production)
- `{{timestamp}}` - Current ISO timestamp
- `{{date}}` - Current date (YYYY-MM-DD)
- `{{uuid}}` - Generated UUID v4
- `{{version}}` - CLI version
- `{{generated}}` - Always `true`

#### **NEW: Template Variables & Conditional Generation**

Templates support powerful variable systems with both static `.conf` configuration and dynamic `--var` CLI flags:

**1. Define variables in `.conf`:**
```ini
[options]
style=scss
style_options=scss,styled-components,css,none
with_tests=true
with_tests_type=boolean

[files]
$FILE_NAME.tsx=always
$FILE_NAME.spec.tsx=var_with_tests
$FILE_NAME.module.scss=var_style_scss
```

**2. Use variables in templates with auto-generated boolean helpers:**
```typescript
import React from "react";
{{#if style_is_scss}}
import styles from "./{{kebab_name}}.module.scss";
{{/if}}
{{#if style_is_styled_components}}
import { Styled{{pascal_name}} } from "./{{kebab_name}}.styled";
{{/if}}

export const {{pascal_name}}: React.FC = () => {
  {{#if style_is_scss}}
  return <div className={styles.{{camel_name}}}>Component</div>;
  {{else}}
  return <div>Component</div>;
  {{/if}}
};
```

**3. Override at runtime:**
```bash
cli-frontend Button --type component --var style=styled-components --var with_tests=false
```

**Key Features:**
- **Automatic Boolean Helpers**: `style=scss` generates `style_is_scss`, `style_is_css`, etc.
- **Conditional Files**: Files generate only when conditions match
- **No Code Changes**: Add new templates with new variables without modifying Rust code
- **Type Safety**: `_type=boolean` for boolean variables, `_options` for enums

> 📖 For detailed template creation guide with `.conf` format specification, see [TEMPLATE_GUIDE.md](./docs/TEMPLATE_GUIDE.md)

## 🤝 Contributing

Contributions are welcome! Help us build the future of frontend code generation:

### How to Contribute
1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/AmazingFeature`)
3. **Commit** your changes (`git commit -m 'Add: Amazing new feature'`)
4. **Push** to the branch (`git push origin feature/AmazingFeature`)
5. **Open** a Pull Request with detailed description

### Contribution Areas
- **New Architecture Patterns** - Add support for additional software architectures
- **Template Improvements** - Enhance existing templates with better patterns
- **Documentation** - Improve guides and examples
- **Performance Optimizations** - Make the CLI faster and more efficient
- **Platform Support** - Extend cross-platform compatibility
- **Testing** - Add comprehensive test coverage

### Development Setup
```bash
git clone https://github.com/FrancoCastro1990/cli-frontend-rust.git
cd cli-frontend-rust
cargo build
cargo test
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👨‍💻 Author

**Franco Castro** - [GitHub](https://github.com/FrancoCastro1990)

## 🙏 Acknowledgments

- The Rust community for excellent development tools and ecosystem
- Frontend developers who inspire better architectural patterns
- Contributors and users who provide valuable feedback and improvements
- Open source projects that make this tool possible

## 📚 Additional Resources

- **[🧙‍♂️ Wizard Guide](./docs/WIZARD_GUIDE.md)** - Complete guide to using the interactive wizard mode
- **[Architecture Guide](./docs/ARCHITECTURES_GUIDE.md)** - Comprehensive documentation for all 12 software architectures
- **[Template Guide](./docs/TEMPLATE_GUIDE.md)** - Learn how to create custom templates
- **[Installation Guide](./docs/INSTALLATION.md)** - Detailed installation instructions for all platforms
- **[GitHub Actions Setup](./docs/GITHUB_ACTIONS_SETUP.md)** - CI/CD integration examples

---

⭐ **If this project helps your development workflow, please give it a star!**

**Ready to standardize your team's frontend architecture?** [Get started now](#-quick-installation) and see the difference in minutes.