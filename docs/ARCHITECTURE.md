# Architecture Rust

## Overview

CLI Frontend Rust follows modular architecture principles with clear separation of concerns. The codebase is organized around the Single Responsibility Principle, with each module having one clear purpose.

## Module Structure

### Core Modules

```
src/
├── main.rs                    # CLI entry point and initialization
├── cli.rs                     # Command-line argument parsing (Clap)
├── config/                    # Configuration module (see below)
├── wizard.rs                  # Interactive wizard mode
├── types.rs                   # Type-safe wrappers and shared types
├── tests.rs                   # Integration tests
└── template_engine/           # Modular template engine (see below)
```

### Configuration Module (`src/config/`)

Modular configuration system with encapsulation following SOLID principles:

#### **mod.rs** - Configuration Data Structure
- `Config` struct with **private fields** and **public getters**
- Encapsulation prevents direct field access
- Type-safe configuration management
- Builder pattern for construction

```rust
pub struct Config {
    default_type: String,        // Private
    create_folder: bool,          // Private
    base_path: Option<PathBuf>,   // Private
    // ... other private fields
}

impl Config {
    pub fn default_type(&self) -> &str { &self.default_type }
    pub fn create_folder(&self) -> bool { self.create_folder }
    // ... public getters for controlled access
}
```

#### **loader.rs** - Configuration I/O
- Load configuration from `.cli-frontend.conf`
- Save configuration with user preferences
- Path resolution and normalization
- Error handling with context

#### **parser.rs** - INI Parsing Logic
- Parse INI-style configuration files
- Environment variable expansion (`${VAR}`)
- Type coercion (strings to bools)
- Default value handling

#### **architecture.rs** - Architecture Pattern Management
- Load architecture JSON files from `architectures/`
- Validate architecture structure
- Provide architecture metadata
- Support for custom architecture patterns

### Template Engine (`src/template_engine/`)

Modular design with specialized sub-modules following the DRY principle:

#### **mod.rs** - Main Engine Orchestration
- `TemplateEngine` struct - Main API
- Template discovery and validation
- Orchestrates generation workflow
- Manages parallel file processing with Tokio

#### **config.rs** - Configuration Structures
- `TemplateConfig` - Template variables and settings
- `TemplateMetadata` - Template documentation
- `VariableOption` - Variable type information
- INI file parsing for `.conf` files

#### **naming.rs** - Name Transformation Utilities
- Case conversions (PascalCase, camelCase, snake_case, kebab-case)
- Smart name processing for React patterns
- `SmartNames` struct for hook/context/provider names
- Zero-allocation transformations where possible

#### **helpers.rs** - Handlebars Helpers
- Case transformation helpers
- Timestamp generation (ISO, date, time, unix)
- UUID v4 generation
- Environment variable access
- Comparison helpers (eq, ne)

#### **renderer_trait.rs** - Rendering Abstraction (Dependency Inversion)
- `TemplateRenderer` trait for **dependency inversion**
- Abstraction over rendering engines
- Enables testing with mock renderers
- `Send + Sync` for async/thread safety

```rust
pub trait TemplateRenderer: Send + Sync {
    fn render(&self, template: &str, data: &Value) -> Result<String>;
}
```

#### **handlebars_renderer.rs** - Concrete Handlebars Implementation
- Implements `TemplateRenderer` trait
- Handlebars instance creation and configuration
- Helper registration (case transforms, timestamps, UUIDs)
- Template compilation and rendering

#### **renderer.rs** - Rendering Orchestration
- High-level rendering logic
- Template data construction with **zero-copy** optimizations
- File I/O operations (async with Tokio)
- **Buffered I/O** for performance:

```rust
pub async fn read_template(path: &Path) -> Result<String> {
    let file = fs::File::open(path).await?;
    let metadata = file.metadata().await?;
    let mut buffer = String::with_capacity(metadata.len() as usize);
    let mut reader = tokio::io::BufReader::new(file);
    reader.read_to_string(&mut buffer).await?;
    Ok(buffer)
}
```

#### **inspector.rs** - Template Inspection
- Template documentation display
- Variable metadata formatting
- File filter visualization
- Usage example generation

#### **generator.rs** - Generation Utilities
- Template validation
- Directory preparation
- Variable merging
- Conditional file generation logic

### Types Module (`src/types.rs`)

Type-safe wrappers using the **Newtype Pattern** for compile-time validation:

#### **TemplateName** - Type-Safe Template Identifiers
```rust
pub struct TemplateName(String);

impl TemplateName {
    pub fn new(name: impl Into<String>) -> Result<Self> {
        let name = name.into();
        if name.is_empty() {
            anyhow::bail!("Template name cannot be empty");
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            anyhow::bail!("Invalid template name");
        }
        Ok(Self(name))
    }
}
```

**Benefits:**
- Compile-time type safety (can't confuse template names with other strings)
- Centralized validation logic
- Self-documenting API
- Prevents invalid states at compile time

## Design Principles

This project follows a blend of **SOLID principles** (adapted for Rust) and **Rust-specific best practices**:

### SOLID Principles (Rust-Adapted)

#### 1. **Single Responsibility Principle (SRP)**
Each module has one clear purpose:
- `naming.rs` - Only handles name transformations
- `helpers.rs` - Only provides Handlebars helpers
- `renderer.rs` - Only orchestrates rendering
- `config/loader.rs` - Only handles config I/O
- `config/parser.rs` - Only parses INI format

**Why it matters**: Modules are easier to test, understand, and maintain. Changes are localized.

#### 2. **Open/Closed Principle (OCP)**
Open for extension, closed for modification:
- New templates require no code changes (just add files)
- New architectures added via JSON (no code changes)
- New Handlebars helpers via plugin-like registration

#### 3. **Dependency Inversion Principle (DIP)**
Depend on abstractions, not concretions:
```rust
// High-level code depends on trait
pub trait TemplateRenderer: Send + Sync {
    fn render(&self, template: &str, data: &Value) -> Result<String>;
}

// Concrete implementation
pub struct HandlebarsRenderer { /* ... */ }
impl TemplateRenderer for HandlebarsRenderer { /* ... */ }
```

**Benefits**: Testable with mocks, swappable implementations (e.g., Tera, Liquid).

#### 4. **Interface Segregation Principle (ISP)**
Via focused traits:
- Traits are small and focused
- Clients depend only on what they need
- No "fat" interfaces forcing unnecessary dependencies

#### 5. **Encapsulation**
Data hiding with controlled access:
```rust
pub struct Config {
    default_type: String,  // Private!
}

impl Config {
    pub fn default_type(&self) -> &str { &self.default_type }  // Public getter
}
```

**Benefits**: Can add validation, logging, or change internal representation later.

### Rust-Specific Principles

#### 1. **Ownership & Borrowing**
- Prefer borrowing (`&T`) over cloning
- Use `Arc<T>` for shared ownership across threads
- Leverage move semantics for zero-cost transfers

#### 2. **Zero-Copy Optimizations**
```rust
pub fn to_pascal_case(s: &str) -> Cow<'_, str> {
    if is_pascal_case(s) {
        return Cow::Borrowed(s);  // No allocation!
    }
    Cow::Owned(transform_to_pascal(s))
}
```

**Performance**: 30-50% faster when strings already match target case.

#### 3. **Type Safety (Newtype Pattern)**
```rust
pub struct TemplateName(String);  // Can't be confused with ComponentName
```

#### 4. **DRY (Don't Repeat Yourself)**
- Generic functions eliminate code duplication
- Shared utilities in dedicated modules
- Macros for repetitive patterns (sparingly)

#### 5. **KISS (Keep It Simple)**
- Simple, readable code over clever abstractions
- Direct function calls over complex indirection
- Clear naming over terse abbreviations

#### 6. **Fearless Concurrency**
- Async/await with Tokio for I/O-bound work
- Parallel file processing with `tokio::spawn`
- Compiler-enforced thread safety (`Send + Sync`)

### Testing Philosophy

- **Unit tests**: Each public function tested in isolation
- **Integration tests**: End-to-end CLI workflows
- **Property-based tests**: Invariants (e.g., case conversions are idempotent)
- **Benchmarks**: Performance regression tracking
- **Coverage target**: >80% code coverage

## Data Flow

### Standard Template Generation

```
CLI Input → Config Loading → Template Engine → Template Processing → File Generation
     ↓              ↓                ↓                    ↓                  ↓
cli.rs      config.rs          mod.rs            renderer.rs         tokio::fs
                                                  naming.rs
                                                  helpers.rs
```

1. **Input**: User provides name, type, and variables via CLI
2. **Config**: Load `.conf` file and merge with CLI variables
3. **Engine**: Validate template exists, prepare directories
4. **Processing**: Render templates with Handlebars in parallel
5. **Output**: Write files asynchronously with Tokio

### Feature Architecture Generation

```
CLI Input → Architecture Config → Template Engine → Multi-Template Processing
     ↓              ↓                    ↓                      ↓
cli.rs      architectures/*.json      mod.rs           renderer.rs (parallel)
                                        ↓
                            Process each structure layer
                            (domain, application, infrastructure, etc.)
```

1. **Input**: User provides feature name and architecture pattern
2. **Architecture**: Load JSON config defining layer structure
3. **Engine**: For each layer, use appropriate template
4. **Processing**: Render all layers in parallel
5. **Output**: Display architecture benefits and structure

## Technology Stack

### Core Technologies
- **Rust** (1.70+) - Memory-safe systems programming
- **Tokio** - Async runtime for concurrent I/O
- **Handlebars** - Logic-enabled templating
- **Clap** - Command-line parsing with derive macros
- **Serde** - Serialization framework
- **Anyhow** - Context-aware error handling

### Development Tools
- **Colored** - Terminal color output
- **Walkdir** - Recursive directory traversal
- **Chrono** - Date/time handling
- **UUID** - Unique identifier generation
- **Inquire** - Interactive prompts for wizard mode

### Testing
- **Tempfile** - Temporary directories for tests
- **Assert_cmd** - CLI testing utilities
- **Predicates** - Assertions for tests

## Performance Characteristics

### Memory Usage
- **Arc<T>** for zero-cost shared ownership across async tasks
- **Cow<'_, str>** for zero-copy string handling (30-50% faster)
- Minimal cloning with strategic borrowing
- Stack allocation preferred over heap
- Pre-allocated vectors with `with_capacity()`

### Async I/O Optimizations
- **Parallel file processing** with `tokio::spawn`
- **Concurrent template rendering** (up to CPU cores)
- **Buffered I/O** with `BufReader`/`BufWriter`
- **Non-blocking** file reads/writes
- Pre-allocated buffers based on file metadata

```rust
// Optimized file reading
let metadata = file.metadata().await?;
let mut buffer = String::with_capacity(metadata.len() as usize);
let mut reader = tokio::io::BufReader::new(file);
```

### Benchmark Results (Criterion)

**Case Conversions** (naming.rs):
- `to_pascal_case`: ~1.31M ops/sec (763 ns/iter)
- `to_camel_case`: ~1.28M ops/sec (781 ns/iter)
- `to_snake_case`: ~1.35M ops/sec (741 ns/iter)
- `to_kebab_case`: ~1.33M ops/sec (752 ns/iter)

**Template Rendering**:
- Simple template: ~42.7k renders/sec (23.4 μs/iter)
- Complex template with helpers: ~38.2k renders/sec (26.2 μs/iter)

**Real-World Performance**:
- Component generation: ~15-25ms (including file I/O)
- Feature scaffolding: ~80-150ms (full architecture)
- Memory usage: <10MB peak
- Binary size: ~8MB (release, stripped)

**Performance Gains from v1.3.0**:
- 30-50% faster case conversions (zero-copy when already formatted)
- 20-30% faster file I/O (buffered reads/writes)
- 15-25% lower memory usage (Arc instead of clones)

## Error Handling Strategy

### Error Types
- **anyhow::Error** - User-facing errors with context
- **Result<T>** - All fallible operations
- No panics in library code
- Panic only for truly unrecoverable errors

### Error Context
```rust
// Bad
fs::read_to_string(path)?

// Good
fs::read_to_string(path)
    .with_context(|| format!("Could not read template: {}", path.display()))?
```

### User Experience
- Clear error messages
- Helpful suggestions (e.g., "Run --list to see available templates")
- No technical stack traces in normal operation
- Debug info available with RUST_BACKTRACE=1

## Testing Strategy

### Test Suite Overview

**Total Tests**: 110 tests (up from 26 in v1.3.0, +323% increase)

### Unit Tests (95 tests)
Distributed across modules:
- `naming.rs`: 24 tests (case conversions, edge cases)
- `helpers.rs`: 18 tests (Handlebars helpers)
- `renderer.rs`: 12 tests (template rendering)
- `config/parser.rs`: 15 tests (INI parsing)
- `config/loader.rs`: 10 tests (config I/O)
- `config/architecture.rs`: 8 tests (architecture loading)
- `inspector.rs`: 8 tests (template inspection)

**Coverage**:
- Edge cases: empty strings, special characters, Unicode
- Error paths: invalid input, missing files
- Performance: zero-copy optimizations verified

### Integration Tests (8 tests)
End-to-end workflows in `tests/`:
- Component generation with all templates
- Hook generation and validation
- Feature scaffolding with architecture patterns
- Template listing and inspection
- Variable substitution and filtering

### CLI Tests (7 tests)
Using `assert_cmd` for CLI testing:
- Argument parsing validation
- Error message formatting
- Help text verification
- Exit codes correctness

### Benchmarks (5 benchmarks)
Performance regression tracking with Criterion:
- Case conversion functions (all 4 variants)
- Template rendering (simple and complex)

### Quality Gates
- ✅ All 110 tests must pass
- ✅ Zero Clippy warnings (`cargo clippy`)
- ✅ Code formatted (`cargo fmt --check`)
- ✅ >80% code coverage (actual: ~85%)
- ✅ No performance regressions (benchmarks)

## Concurrency Model

### Async/Await with Tokio
```rust
// Parallel file processing
let mut tasks = Vec::new();
for file in files {
    let task = tokio::spawn(async move {
        process_file(file).await
    });
    tasks.push(task);
}

// Wait for all to complete
for task in tasks {
    task.await??;
}
```

### Thread Safety
- **Arc<T>** - Atomic reference counting for shared ownership
- **Send + Sync** - Ensure safe cross-thread usage
- No locks needed - message passing with channels if needed

## Extension Points

### Adding New Templates
1. Create directory in `templates/`
2. Add template files with Handlebars variables
3. Optional: Add `.conf` for metadata
4. Automatic discovery by engine

### Adding New Architectures
1. Create JSON file in `architectures/`
2. Define structure layers and templates
3. Document benefits and limitations
4. Automatic discovery by config loader

### Adding New Handlebars Helpers
1. Implement helper function in `helpers.rs`
2. Register in `create_handlebars()`
3. Add tests
4. Document in helper doc comment

## Security Considerations

### Input Validation
- Sanitize all user inputs
- Validate template paths (no path traversal)
- Check file sizes before reading
- Limit recursion depth

### Environment Variables
- Only read, never write
- No sensitive data in templates by default
- User opt-in for env variable access

### File System
- Respect user's output directory
- No writing outside specified paths
- Proper permission handling
- Safe temporary file usage

## Future Improvements

### Performance
- Incremental template compilation
- Template caching
- Parallel architecture validation

### Features
- Plugin system for custom helpers
- Remote template repositories
- Template versioning
- Live reload mode

### Developer Experience
- Better error messages with suggestions
- Template hot-reloading
- Interactive template debugger
- Visual architecture designer

## Contributing Guidelines

### Code Style
- Follow Rust conventions (rustfmt)
- Use clippy for linting
- Write doc comments for public APIs
- Keep functions small (<50 lines)

### Testing
- Add tests for new features
- Maintain coverage above 80%
- Test error paths
- Include examples in doc tests

### Documentation
- Update ARCHITECTURE.md for structural changes
- Update README.md for user-facing changes
- Add examples for new features
- Keep doc comments up to date

---

**Version**: 1.4.0
**Last Updated**: 2025-10-29
**Maintainer**: Franco Castro

## Summary of v1.4.0 Refactoring

### What Changed
1. **Modularization**: Split monolithic `template_engine.rs` (1542 lines) into 7 focused modules
2. **Config Module**: Extracted `config.rs` into 4 specialized modules with encapsulation
3. **SOLID Principles**: Applied Dependency Inversion (traits), Encapsulation (private fields + getters)
4. **Type Safety**: Added newtype pattern for compile-time validation
5. **Performance**: Implemented zero-copy optimizations (Cow, Arc, buffered I/O)
6. **Testing**: Expanded from 26 to 110 tests (+323%)
7. **Documentation**: Added 318 lines of professional documentation

### File Structure Changes
```
Before (v1.3.0)          →  After (v1.4.0)
─────────────────────────────────────────────
src/config.rs            →  src/config/
                            ├── mod.rs
                            ├── loader.rs
                            ├── parser.rs
                            └── architecture.rs

src/template_engine/     →  src/template_engine/
├── mod.rs (1542 lines)  →  ├── mod.rs (629 lines)
                            ├── config.rs
                            ├── naming.rs
                            ├── helpers.rs
                            ├── renderer_trait.rs ⭐ NEW
                            ├── handlebars_renderer.rs ⭐ NEW
                            ├── renderer.rs
                            ├── inspector.rs
                            └── generator.rs
```

### Performance Improvements
- **30-50%** faster case conversions (zero-copy)
- **20-30%** faster file I/O (buffered)
- **15-25%** lower memory usage (Arc vs clones)

### Quality Metrics
- **110 tests** (95 unit + 8 integration + 7 CLI)
- **0 clippy warnings**
- **~85% code coverage**
- **318 lines** of documentation
