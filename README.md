# Crisp ZK Witness

A Rust monorepo for CRISP ZK witness generation with JavaScript/WASM bindings.

## Structure

This repository contains two main crates:

- **`generator`**: Core witness generation logic written in Rust.
- **`js-lib`**: JavaScript library with WASM bindings for the generator.

## Prerequisites

- Rust (latest stable version)
- `wasm-pack` for building WASM bindings

Install wasm-pack:

```bash
cargo install wasm-pack
```

## Building

### Build all crates

```bash
cargo build
```

### Build the JavaScript library with WASM bindings

```bash
cd crates/js-lib
wasm-pack build
```

## Testing

### Run all tests

```bash
cargo test
```

### Run tests for specific crate

```bash
cargo test -p crisp-zk-witness
cargo test -p js-lib
```

## Usage

### Rust

```rust
use crisp_zk_witness::WitnessGenerator;

let generator = WitnessGenerator::new();
let witness = generator.generate_witness("your input")?;
```

### JavaScript

```javascript
import init, { JsWitnessGenerator } from "./pkg/js_lib.js";

await init();

const generator = new JsWitnessGenerator();
const witness = generator.generate_witness("your input");
console.log(witness);
```

## Development

This is a workspace project. You can work on individual crates or the entire workspace:

```bash
# Work on the entire workspace
cargo check

# Work on specific crate
cargo check -p crisp-zk-witness
cargo check -p js-lib
```
