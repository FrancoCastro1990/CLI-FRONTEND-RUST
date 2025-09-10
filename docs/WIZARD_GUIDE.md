# 🧙‍♂️ Interactive Wizard Guide

**Complete guide to using the CLI Frontend Generator's Interactive Wizard Mode**

The Interactive Wizard Mode transforms the CLI experience from memorizing commands to a guided, user-friendly interface that walks you through code generation step by step.

> ✨ **New in v1.3.0**: Zero learning curve - just run `cli-frontend` and follow the prompts!

## 🎯 Why Use the Wizard?

### Perfect For:
- **New Team Members** - No need to memorize commands or flags
- **Complex Decisions** - Guided selection of architectures and options
- **Exploration** - Discover available templates and architectures
- **Error Prevention** - Built-in validation prevents common mistakes
- **Speed** - Faster than typing complex commands with multiple flags

### Benefits:
- ✅ **Zero Learning Curve** - Intuitive interface guides you through every step
- ✅ **Input Validation** - Real-time feedback prevents errors before generation
- ✅ **Context-Aware Help** - Smart suggestions based on your selections
- ✅ **Graceful Cancellation** - Press ESC anytime to exit cleanly
- ✅ **Complete Visibility** - See exactly what will be generated before confirming

## 🚀 Getting Started

### Launching the Wizard

The wizard launches automatically when you run the CLI without arguments:

```bash
# Launch the interactive wizard
cli-frontend
```

**What you'll see:**
```
🧙‍♂️ CLI Frontend Generator Wizard
=====================================
Let's create something amazing! I'll guide you through the process.
Press ESC at any time to cancel.

? What do you want to generate? ›
❯ 📄 Individual Template (component, hook, service, etc.)
  🏗️  Complete Feature (with architecture pattern)
```

## 🎨 Wizard Flow Overview

### Step-by-Step Process:

```
1. Welcome & Instructions
   ↓
2. Choose Generation Type
   ├── Template Path (Individual Components)
   │   ├── Select Template Type
   │   ├── Enter Name (with validation)
   │   ├── Configure Options
   │   └── Review & Generate
   │
   └── Feature Path (Complete Architectures)
       ├── Select Architecture Pattern
       ├── Enter Feature Name (with validation)
       ├── Configure Options
       └── Review & Generate
```

## 📄 Template Generation Path

### 1. Select Template Type
When you choose "Individual Template", you'll see all available template types:

```
? Select template type: ›
❯ component     - React component with styles and tests
  hook          - Custom React hook with tests
  service       - Business logic service
  context       - React context with provider
  page          - Page component with routing setup
  store         - Redux store slice with actions
  api           - API service interface
  api-service   - Advanced API service with configuration
```

> 💡 **Dynamic Discovery**: The list automatically includes any custom templates you've added to your templates directory.

### 2. Enter Name with Smart Validation

The wizard validates your input in real-time:

```
? Enter the component name: › MyButton

✓ Name validation:
  ✅ Not empty
  ✅ At least 2 characters
  ✅ Valid format (letters, numbers, underscores only)
```

**Smart Help Messages:**
- **Components**: "Use PascalCase (e.g., MyButton, UserCard)"
- **Hooks**: "Use camelCase starting with 'use' (e.g., useAuth, useLocalStorage)"
- **Services**: "Use PascalCase or camelCase (e.g., UserService, apiClient)"

### 3. Configure Additional Options

```
Additional Options:
? Create in new folder? › Yes / No
? Use custom output directory? › Yes / No
```

If you choose custom directory:
```
? Enter output directory path: › ./src/components
```

### 4. Review & Generate

Before generating, you'll see a complete summary:

```
📋 Generation Summary:
=====================================
Type: Component
Name: MyButton
Output: ./MyButton/
Files to generate:
  ✓ MyButton.tsx
  ✓ MyButton.module.scss  
  ✓ MyButton.spec.tsx

? Proceed with generation? › Yes
```

## 🏗️ Feature Generation Path

### 1. Select Architecture Pattern

When you choose "Complete Feature", you'll see all available architectures:

```
? Select architecture pattern: ›
❯ atomic-design        - Systematic component hierarchy (Atoms → Molecules → Organisms)
  clean-architecture   - Enterprise-grade layered architecture
  component-based      - Simple component-focused structure
  event-driven        - Reactive systems with loose coupling
  flux                - Unidirectional data flow with dispatcher
  micro-frontends     - Independent deployable modules
  mvc                 - Model-View-Controller pattern
  mvp                 - Model-View-Presenter pattern
  mvvm                - Model-View-ViewModel pattern
  redux               - Predictable state management
  screaming-architecture - Feature-first organization (DEFAULT)
```

> 🏛️ **Architecture Help**: Each option shows a brief description. For detailed documentation, see [ARCHITECTURES_GUIDE.md](./ARCHITECTURES_GUIDE.md).

### 2. Enter Feature Name

Feature names follow the same validation as templates but with additional context:

```
? Enter the feature name: › UserAuthentication

💡 Feature naming tips:
  • Use PascalCase for feature names
  • Be descriptive but concise
  • Examples: UserAuth, PaymentSystem, ProductCatalog
```

### 3. Configure Options

Same as template generation - folder creation and output directory options.

### 4. Review Complete Feature Structure

Features show a detailed file tree preview:

```
📋 Feature Generation Summary:
=====================================
Architecture: Clean Architecture
Feature: UserAuthentication
Output: ./UserAuthentication/

Files to generate:
  📁 domain/
    ├── entities/UserAuthenticationEntity.ts
    └── repositories/UserAuthenticationRepository.ts
  📁 application/
    └── usecases/UserAuthenticationUseCase.ts
  📁 infrastructure/
    └── services/UserAuthenticationService.ts  
  📁 presentation/
    ├── components/UserAuthentication.tsx
    ├── hooks/useUserAuthentication.ts
    └── UserAuthentication.module.scss
  📄 types.ts

? Proceed with generation? › Yes
```

## ⌨️ Wizard Controls & Navigation

### Keyboard Shortcuts:
- **↑/↓ Arrow Keys** - Navigate options
- **Enter** - Select/Confirm
- **ESC** - Cancel gracefully at any time
- **Tab** - Auto-complete (where available)

### Smart Cancellation:
Press ESC at any point to cancel gracefully:

```
👋 Wizard canceled. See you next time!
```

**No error codes, no messy exits** - just a clean return to your terminal.

## ✅ Input Validation & Error Prevention

### Name Validation Rules:
- ✅ **Not empty** - Must provide a name
- ✅ **Minimum length** - At least 2 characters
- ✅ **Valid characters** - Letters, numbers, underscores only
- ✅ **Contextual format** - Smart suggestions based on template type

### Real-time Feedback:
```
? Enter the hook name: › use

❌ Name must be at least 2 characters long

? Enter the hook name: › use@#$

❌ Name should contain only letters, numbers, and underscores

? Enter the hook name: › useAuth

✅ Valid name!
```

### Directory Validation:
- Validates paths exist or can be created
- Prevents invalid characters in paths
- Shows resolved absolute paths for confirmation

## 🎨 Wizard Customization

### Context-Aware Help

The wizard provides different help text based on what you're generating:

**For Components:**
```
💡 Component naming suggestions:
  • Use PascalCase (MyButton, UserCard)
  • Be descriptive but concise
  • Avoid generic names like 'Component'
```

**For Hooks:**
```
💡 Hook naming suggestions:
  • Start with 'use' (useAuth, useLocalStorage)  
  • Use camelCase after 'use'
  • Describe what the hook does
```

**For Services:**
```
💡 Service naming suggestions:
  • Use PascalCase (UserService, ApiClient)
  • Include 'Service' suffix when appropriate
  • Be clear about the service purpose
```

### Smart Defaults

The wizard remembers your configuration and uses intelligent defaults:
- Uses your configured output directory
- Respects your `create_folder` preference from config
- Suggests appropriate naming conventions per template type

## 🛠️ Advanced Usage

### Using Wizard with Custom Templates

If you've added custom templates to your `templates/` directory, they automatically appear in the wizard:

```
? Select template type: ›
❯ component
  hook
  service  
  my-custom-template    # ← Your custom template
  another-template      # ← Another custom template
```

The wizard automatically detects and includes any custom templates without requiring CLI recompilation.

### Configuration Integration

The wizard respects all your configuration settings from `.cli-frontend.conf`:

```ini
# Your config is automatically used
default_type=component
create_folder=true
output_dir=./src/components
templates_dir=/path/to/templates
architectures_dir=/path/to/architectures
```

## 🚀 Pro Tips

### Speed Up Your Workflow:
1. **Use the wizard for exploration** - Browse available templates and architectures
2. **Let validation guide you** - The real-time feedback teaches best practices
3. **Bookmark common paths** - After using the wizard, you'll know the direct commands for frequent tasks
4. **Cancel anytime** - Use ESC to quickly back out and try different approaches

### Best Practices:
1. **Start with the wizard** - Especially when learning new architectures
2. **Use direct commands for repetitive tasks** - Once you know the patterns
3. **Explore architectures** - Try different patterns to understand their structures
4. **Validate before generating** - Always review the summary before confirming

## 🔄 Transitioning from Wizard to Direct Commands

The wizard teaches you the direct command equivalents. After using the wizard, you'll see patterns like:

**Wizard Selection:**
- Type: component
- Name: MyButton  
- Create folder: Yes
- Output: ./src/components

**Equivalent Direct Command:**
```bash
cli-frontend MyButton --type component --output-dir ./src/components
```

**Feature with Architecture:**
```bash
cli-frontend UserAuth --type feature --architecture clean-architecture
```

## 🐛 Troubleshooting

### Wizard Won't Start
**Problem**: CLI shows help instead of starting wizard

**Solution**: Ensure you're not passing any arguments:
```bash
# ✅ Correct - launches wizard
cli-frontend

# ❌ Wrong - shows help
cli-frontend --help
cli-frontend MyComponent
```

### Templates Not Showing
**Problem**: Custom templates don't appear in wizard

**Solution**: Check your configuration file has correct paths:
```bash
# Verify config
cat ~/.cli-frontend.conf

# Should show absolute paths
templates_dir=/absolute/path/to/templates
```

### Validation Errors
**Problem**: Name validation keeps failing

**Solutions**:
- Ensure name has at least 2 characters
- Use only letters, numbers, and underscores
- Follow the suggested naming convention for the template type

### ESC Not Working
**Problem**: ESC doesn't cancel the wizard properly

**This should not happen with v1.3.0+, but if it does:**
- Try Ctrl+C as alternative
- Report the issue on GitHub
- Use `cli-frontend --help` to see available options

## 📊 Wizard vs Direct Commands

| Aspect | Wizard Mode | Direct Commands |
|--------|-------------|-----------------|
| **Learning Curve** | None - guided UI | Requires memorizing flags |
| **Speed (First Time)** | Faster - no syntax to learn | Slower - need to check help |
| **Speed (Repetitive)** | Moderate - interactive prompts | Fastest - direct execution |
| **Error Prevention** | Excellent - real-time validation | Good - runtime error messages |
| **Discovery** | Excellent - shows all options | Limited - need to check docs |
| **Automation** | Not suitable | Perfect for scripts |

## 🎯 When to Use Each Mode

### Use Wizard Mode When:
- 🆕 **Learning the CLI** - New to the tool or exploring features
- 🔍 **Exploring architectures** - Want to see what's available
- 🎨 **Complex features** - Generating complete features with many options
- 👥 **Onboarding others** - Teaching team members
- 🤔 **Unsure about options** - Want guided experience

### Use Direct Commands When:
- ⚡ **Repetitive tasks** - Generating many similar components
- 🔄 **Automation** - Scripts, CI/CD, or batch operations
- 💨 **Quick generation** - Know exactly what you want
- 📝 **Documentation** - Sharing commands with team

## 🤝 Contributing to the Wizard

### Ideas for Enhancement:
- **Template previews** - Show sample generated code
- **Favorites** - Remember frequently used combinations
- **Search functionality** - Filter templates and architectures
- **Batch generation** - Generate multiple items in one session

### Feedback Welcome:
- Found the wizard confusing? Let us know!
- Missing a feature? Open an issue!
- Have improvement ideas? We'd love to hear them!

---

🎉 **The Interactive Wizard makes CLI Frontend Generator accessible to everyone - from beginners to experts!**

**Ready to generate amazing code?** Just run `cli-frontend` and let the wizard guide you! ✨

**Need help?** Check out our other guides:
- [Architecture Guide](./ARCHITECTURES_GUIDE.md) - Understand the 12 architecture patterns
- [Template Guide](./TEMPLATE_GUIDE.md) - Create custom templates
- [Installation Guide](./INSTALLATION.md) - Get set up on any platform