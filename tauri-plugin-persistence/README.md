# tauri-plugin-persistence
A wrapper plugin for several persistence backends, focused on managing complex project folders with less boilerplate.

## Installation

```bash
# Install cargo dependency
cargo add tauri-plugin-persistence

# Install JS dependency
npm install tauri-plugin-persistence-api
```

## Setup

The plugin must be initialized in Rust. A basic example follows:

```rust
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_persistence::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

## Usage

The plugin's functions can be accessed in Rust from `app.persistence()`, or in the frontend (see [the example](/tauri-plugin-persistence/examples/persistence-examples)).