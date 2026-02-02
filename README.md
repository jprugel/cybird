# Cybird 🕊️

A flexible, type-safe plugin system for Rust applications and games. Cybird provides a generic API that allows developers to create modular, extensible applications with dynamic plugin loading capabilities.

## 🎯 Overview

Cybird is designed with two primary goals:
- **For Game Developers**: Easy-to-implement plugin API that can be integrated into any game or application
- **For Plugin Developers**: Simple, type-safe interface for creating plugins that extend game functionality

The system is built around a generic plugin architecture that can be adapted to any domain, with a complete example implementation for clicker-style games.

## 🏗️ Architecture

The project consists of several interconnected crates:

### Core Crates
- **`cybird`** - The main plugin API with core traits and functionality
- **`cybird-macro`** - Procedural macros for code generation and ergonomic derives

### Example Implementation  
- **`clicker-plugin`** - A concrete implementation showing how to build a plugin system for clicker games
- **`crypto-crab`** - A complete Bevy-based clicker game demonstrating plugin integration
- **`crazier-crab`** - An example dynamic plugin that adds upgrades to the game

## 🚀 Quick Start

### Adding Cybird to Your Project

```toml
[dependencies]
cybird = { path = "path/to/cybird" }
```

### Defining Your Plugin Context

First, define what types of items your plugins can register:

```rust
use cybird::prelude::*;

// Define what plugins can register
pub enum Registrable {
    Upgrade(Upgrade),
    Item(Item),
    // Add more types as needed
}

// Your plugin context - this holds all registered items
#[derive(Default, Context)]
pub struct GameContext(Vec<Registrable>);
```

### Creating Registrable Types

Make your types registrable by implementing the derive:

```rust
#[derive(Registrable)]
pub struct Upgrade {
    pub name: String,
    pub level: u32,
    pub cost: u32,
    // ... other fields
}
```

### Implementing Plugins

Create plugins by implementing the `Plugin` trait:

```rust
use cybird::prelude::*;

#[cybird::plugin]
#[derive(Default)]
pub struct MyPlugin;

impl Plugin<GameContext> for MyPlugin {
    fn author(&self) -> &str {
        "Your Name"
    }

    fn id(&self) -> &str {
        "my-plugin"
    }

    fn load(&self, ctx: &mut GameContext) -> cybird::Result<()> {
        // Register your items
        ctx.register(Upgrade {
            name: "Super Upgrade".to_string(),
            level: 1,
            cost: 100,
        });
        
        Ok(())
    }
}
```

Build the plugin as a dynamic library:

```toml
[lib]
crate-type = ["rlib", "cdylib"]

[features]
default = ["dynamic"]
dynamic = []
```

## 📚 Complete Example

Here's how the clicker game example works:

### 1. Define the Plugin System (clicker-plugin)

```rust
#[derive(Default, Context)]
pub struct PluginContext(Vec<Registrable>);

pub enum Registrable {
    Upgrade(Upgrade),
}

#[derive(Registrable)]
pub struct Upgrade {
    pub name: String,
    pub level: u32,
    pub stage: u32,
    pub cost: fn(u32) -> u32,
    pub effects: Vec<Effect>,
    // ...
}
```

### 2. Integrate into Your Game (crypto-crab)

```rust
pub struct GameState {
    context: PluginContext,
    stage: u32,
}

// Load and use plugins
fn load_plugins(mut game_state: ResMut<GameState>) {
    // Static plugin loading
    let plugin = MyPlugin::default();
    plugin.load(&mut game_state.context).unwrap();
    
    // Get registered items
    let upgrades = game_state.context.get_registrables::<Upgrade>();
    // Use upgrades in your game logic...
}
```

### 3. Create Dynamic Plugins (crazier-crab)

```rust
#[cybird::plugin]
#[derive(Default)]
pub struct CrazierCrab;

impl Plugin<PluginContext> for CrazierCrab {
    fn load(&self, ctx: &mut PluginContext) -> Result<()> {
        ctx.register(Upgrade {
            name: "Crumble Decintigrator".to_string(),
            // ... configuration
        });
        Ok(())
    }
}
```

## 🔧 Building and Running

### Build the example game:
```bash
just run
```

### Build a dynamic plugin:
```bash
just build
```

### Using with Just:
The project includes a `justfile` with convenient commands:
- `just run` - Run the example game
- `just build` - Build dynamic plugins

## 🎮 Example Game Features

The included `crypto-crab` example demonstrates:
- ✅ Dynamic plugin loading
- ✅ Upgrade system with effects
- ✅ Stage-based progression
- ✅ Real-time plugin integration
- ✅ Bevy ECS integration
- ✅ Debug inspector UI

## 🛠️ API Reference

### Core Traits

#### `Plugin<T: Context>`
The main trait for implementing plugins:
- `fn author(&self) -> &str` - Plugin author information
- `fn id(&self) -> &str` - Unique plugin identifier  
- `fn load(&self, ctx: &mut T) -> Result<()>` - Load plugin content

#### `Context`
Manages registrable items:
- `fn register<T>(&mut self, item: T)` - Register an item
- `fn get_registrables<T>(&self) -> Vec<&T>` - Get items of a specific type
- `fn get_registrables_mut<T>(&mut self) -> Vec<&mut T>` - Get mutable references

### Derive Macros

#### `#[derive(Context)]`
Auto-implements the `Context` trait. Works with:
- Tuple structs: `MyContext(Vec<Registrable>)`
- Named fields: Automatically detects `Vec<T>` fields
- Custom configuration: `#[context(registrable = MyEnum, field = my_field)]`

#### `#[derive(Registrable)]`
Implements conversion traits for registration. Supports:
- Default enum variants: `Registrable::TypeName(value)`
- Custom variants: `#[registrable(CustomVariant)]`
- External enums: `#[registrable(MyEnum::Variant)]`

## 🏆 Design Philosophy

1. **Type Safety**: Leverage Rust's type system to prevent common plugin errors
2. **Flexibility**: Generic design that adapts to any application domain  
3. **Ergonomics**: Derive macros eliminate boilerplate code
4. **Performance**: Zero-cost abstractions and compile-time generation
5. **Extensibility**: Easy to add new registrable types and plugin capabilities

## 🤝 Contributing

This project is designed to be a foundation for plugin systems in Rust applications. Contributions are welcome for:
- Additional example implementations
- Performance improvements
- Documentation enhancements
- New plugin system patterns

## 📋 To Do

- [x] Implement simple API for developers to implement a single feature that is moddable
- [ ] Expand to multi-feature API
- [ ] Allow game developers to create a simple mod collection manager with a single command using just
- [ ] Expand just commands to focus on extension

## 📄 License

This project is open source. See individual crate documentation for specific licensing information.

---

**Getting Started**: Check out the `crypto-crab` example to see Cybird in action, or dive into the `clicker-plugin` crate to understand how to build your own plugin system!
```

This README provides a comprehensive overview of your Cybird plugin system, explaining:

1. **What it is**: A flexible plugin system for Rust applications
2. **Architecture**: How the different crates work together
3. **Usage examples**: Step-by-step guide for implementation
4. **Complete working example**: Based on your clicker game
5. **API reference**: Documentation of key traits and macros
6. **Design philosophy**: The principles behind the system

The README is structured to help both game developers who want to integrate
