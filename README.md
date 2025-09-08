# CLI Frontend Generator 🚀

**Professional code generation for scalable frontend architectures**

A powerful Rust-based CLI tool that generates production-ready React components, hooks, services, and complete feature structures following established software architecture patterns. Designed for development teams who value consistency, maintainability, and architectural best practices.

[![Version](https://img.shields.io/badge/version-1.2.3-blue.svg)](https://github.com/FrancoCastro1990/cli-frontend-rust/releases)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://rustup.rs)

> 🎉 **Version 1.2.3** - Enhanced with 12 software architectures, automated configuration, and intelligent cross-platform installers

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
- [**+ 5 more architectures**](./ARCHITECTURES_GUIDE.md) with detailed documentation

### 🎯 **Production-Ready Code Generation**
- **TypeScript-first** - Full type safety and IntelliSense support
- **SCSS Modules** - Scoped styling with CSS module patterns
- **Automated Testing** - Jest/Testing Library test files included
- **React Best Practices** - Hooks, functional components, modern patterns

### 🔧 **Developer Experience Excellence**
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
curl -sSL https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install-quick.sh | bash
```

**Windows (PowerShell as Administrator):**
```powershell
iwr -useb https://raw.githubusercontent.com/FrancoCastro1990/cli-frontend-rust/main/install-quick.ps1 | iex
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
# Generate a React component
cli-frontend Button --type component

# Generate a custom hook
cli-frontend useAuth --type hook

# Generate an API service
cli-frontend UserAPI --type api

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

> 📖 **Comprehensive Guide**: See [ARCHITECTURES_GUIDE.md](./ARCHITECTURES_GUIDE.md) for detailed documentation, benefits, limitations, and usage scenarios for each architecture.

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

```bash
cli-frontend [name] [OPTIONS]

Arguments:
  <name>                      Name of the template or feature to generate

Options:
  -t, --type <TYPE>           Template type to generate (auto-detection available)
  -a, --architecture <ARCH>   Architecture pattern for features (mvc, clean-architecture, etc.)
  --no-folder                 Generate files without creating a parent folder
  -o, --output-dir <DIR>      Output directory for generated files
  -c, --config <CONFIG>       Path to custom configuration file
  --help                      Display help, available templates and architectures
```

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

> 🔧 **Extensible**: Add custom templates by creating folders in the `templates/` directory. The CLI automatically discovers new templates without recompilation.

## ⚙️ Configuration

The CLI searches for configuration files in this order:
1. `.cli-frontend.conf` in current directory
2. `~/.cli-frontend.conf` in home directory  
3. File specified with `--config` option

### Configuration Example
```ini
# CLI Frontend Generator Configuration

# General settings
default_type=component
create_folder=true
enable_hooks=true

# Path configuration
templates_dir=./templates
output_dir=./src
architectures_dir=./architectures

# Architecture settings
default_architecture=screaming-architecture
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
│   └── [custom-templates]/  # User-defined templates
├── architectures/           # Software architecture configurations
│   ├── clean-architecture.json
│   ├── redux.json
│   ├── flux.json
│   └── [12 total architectures]
└── docs/                    # Comprehensive documentation
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

## 🔮 Roadmap

### ✅ Completed (v1.2.3)
- [x] 12 comprehensive software architecture patterns
- [x] Dynamic template discovery and auto-detection
- [x] Cross-platform intelligent installers
- [x] Automated configuration management
- [x] Professional documentation and guides

### 🚧 In Development (v1.3.0)
- [ ] **VS Code Extension** - Integrated development experience
- [ ] **.conf** - .conf in template to add variables and config in custom templates.
- [ ] **Template Validation** - JSON schema validation for custom templates
- [ ] **Interactive Mode** - Guided architecture selection wizard
- [ ] **Template Marketplace** - Community-driven template sharing

### 🌟 Planned Features (v2.0.0)
- [ ] **Multi-framework Support** - Vue.js and Angular templates


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

> 📖 For detailed template creation guide, see [TEMPLATE_GUIDE.md](./TEMPLATE_GUIDE.md)

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

- **[Architecture Guide](./ARCHITECTURES_GUIDE.md)** - Comprehensive documentation for all 12 software architectures
- **[Template Guide](./TEMPLATE_GUIDE.md)** - Learn how to create custom templates
- **[Installation Guide](./INSTALLATION.md)** - Detailed installation instructions for all platforms
- **[GitHub Actions Setup](./GITHUB_ACTIONS_SETUP.md)** - CI/CD integration examples

---

⭐ **If this project helps your development workflow, please give it a star!**

**Ready to standardize your team's frontend architecture?** [Get started now](#-quick-installation) and see the difference in minutes.