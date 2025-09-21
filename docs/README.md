# OpenSCENARIO-rs Documentation

Welcome to the comprehensive documentation for OpenSCENARIO-rs, a type-safe, high-performance Rust library for parsing, validating, and manipulating OpenSCENARIO files.

## 📚 Documentation Overview

This documentation provides complete coverage of the library's capabilities, from basic usage to advanced extension development.

### 🚀 Getting Started

- **[User Guide](user_guide.md)** - Complete tutorial from installation to advanced usage
- **[API Reference](api_reference.md)** - Comprehensive API documentation with examples
- **[Examples](../examples/)** - Real-world code examples and demos

### 🏗️ Core Architecture

- **[Type System Guide](type_system_guide.md)** - Understanding the Value<T> system and schema mapping
- **[Validation Guide](validation_guide.md)** - Error handling and validation system
- **[Performance Guide](performance_guide.md)** - Optimization strategies and best practices

### 🔧 Advanced Topics

- **[Extension Guide](extension_guide.md)** - Creating custom types, actions, and integrations
- **[Development Guide](development_guide.md)** - Contributing patterns and architectural insights

## 🎯 Quick Navigation

### For New Users
1. Start with the **[User Guide](user_guide.md)** for installation and basic concepts
2. Browse **[Examples](../examples/)** for practical usage patterns
3. Reference **[API Documentation](api_reference.md)** for specific functions

### For Library Developers
1. Read **[Type System Guide](type_system_guide.md)** to understand the architecture
2. Study **[Development Guide](development_guide.md)** for coding patterns
3. Follow **[Extension Guide](extension_guide.md)** for creating custom functionality

### For Performance-Critical Applications
1. Review **[Performance Guide](performance_guide.md)** for optimization strategies
2. Implement monitoring using patterns from the performance guide
3. Use **[Validation Guide](validation_guide.md)** for efficient error handling

## 📖 Documentation Structure

```
docs/
├── README.md                 # This overview (you are here)
├── user_guide.md            # Complete usage tutorial
├── api_reference.md         # Detailed API documentation
├── type_system_guide.md     # Type system and schema mapping
├── validation_guide.md      # Validation and error handling
├── extension_guide.md       # Customization and extensions
├── performance_guide.md     # Performance optimization
└── development_guide.md     # Development patterns and troubleshooting
```

## 🔍 Key Features Covered

### Core Functionality
- **Complete OpenSCENARIO Support** - All 347+ types from the specification
- **Type-Safe Parsing** - Rust's type system prevents runtime errors
- **Parameter Resolution** - Full `${parameter}` reference support
- **Catalog Management** - Automatic loading and resolution
- **XML Round-Trip** - Perfect serialization/deserialization

### Advanced Capabilities
- **Custom Extensions** - Add domain-specific types and validation
- **Performance Optimization** - Zero-copy parsing and streaming support
- **Validation System** - Comprehensive error checking and reporting
- **Integration APIs** - Connect with simulation engines and pipelines

## 📋 Common Use Cases

| Use Case | Primary Docs | Key Features |
|----------|-------------|--------------|
| **Parse scenario files** | [User Guide](user_guide.md) | Basic parsing, entity access |
| **Validate scenarios** | [Validation Guide](validation_guide.md) | Built-in validation, custom rules |
| **Build scenarios programmatically** | [API Reference](api_reference.md) | Builder patterns, fluent APIs |
| **Handle catalogs** | [User Guide](user_guide.md) | Catalog resolution, caching |
| **Optimize performance** | [Performance Guide](performance_guide.md) | Memory management, streaming |
| **Add custom functionality** | [Extension Guide](extension_guide.md) | Custom types, actions, validators |
| **Integrate with systems** | [Extension Guide](extension_guide.md) | Simulation bridges, data pipelines |

## 🎲 Example Code Index

### Basic Usage Examples
```rust
// Parse a scenario file
use openscenario_rs::parse_file;
let scenario = parse_file("scenario.xosc")?;

// Access entities
if let Some(entities) = &scenario.entities {
    for entity in &entities.scenario_objects {
        println!("Entity: {}", entity.name.as_literal().unwrap());
    }
}
```

### Advanced Usage Examples
```rust
// Custom validation
use openscenario_rs::types::{Validate, ValidationContext};
impl Validate for MyCustomType {
    fn validate(&self, ctx: &ValidationContext) -> Result<()> {
        // Custom validation logic
        Ok(())
    }
}

// Performance optimization
use openscenario_rs::parser::streaming::StreamingParser;
let mut parser = StreamingParser::new(file);
while let Some(entity) = parser.next_entity()? {
    process_entity(&entity);
}
```

## 🔗 Cross-References

### Related Documentation
- **OpenSCENARIO Specification** - Official ASAM standard documentation
- **Rust serde Documentation** - Serialization framework used internally
- **quick-xml Documentation** - XML parsing library

### External Resources
- **[OpenSCENARIO Official Site](https://www.asam.net/standards/detail/openscenario/)** - Specification and resources
- **[Rust Book](https://doc.rust-lang.org/book/)** - Learning Rust programming
- **[Cargo Guide](https://doc.rust-lang.org/cargo/)** - Rust package management

## 🚦 Getting Help

### Documentation Issues
- Missing information? Check the [API Reference](api_reference.md)
- Need examples? Browse the [examples directory](../examples/)
- Performance concerns? See the [Performance Guide](performance_guide.md)

### Code Issues
- Compilation errors? Check the [Development Guide](development_guide.md)
- Type system confusion? Read the [Type System Guide](type_system_guide.md)
- Validation problems? Study the [Validation Guide](validation_guide.md)

### Community Support
- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - Questions and community help
- **Documentation PRs** - Improvements and corrections welcome

## 📝 Contributing to Documentation

We welcome documentation improvements! Here's how to contribute:

1. **Identify gaps** - What's missing or unclear?
2. **Propose changes** - Open an issue or discussion
3. **Submit improvements** - Create a pull request
4. **Review process** - Community review and feedback

### Documentation Standards
- **Clear examples** - Every concept should have code examples
- **Complete coverage** - All public APIs should be documented
- **Practical focus** - Emphasize real-world usage patterns
- **Consistent style** - Follow established formatting and tone

## 🎯 Next Steps

Choose your path based on your goals:

**🚀 Get Started Quickly**
→ [User Guide](user_guide.md) → [Examples](../examples/) → [API Reference](api_reference.md)

**🔧 Understand the Architecture**
→ [Type System Guide](type_system_guide.md) → [Development Guide](development_guide.md) → [Extension Guide](extension_guide.md)

**⚡ Optimize Performance**
→ [Performance Guide](performance_guide.md) → [Validation Guide](validation_guide.md) → Advanced examples

**🎨 Build Extensions**
→ [Extension Guide](extension_guide.md) → [Type System Guide](type_system_guide.md) → Custom development

---

*This documentation is maintained alongside the OpenSCENARIO-rs library. For the latest updates, visit the [GitHub repository](https://github.com/ashfaqfarooqui/openscenario-rs).*