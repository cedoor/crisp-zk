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
cd js-scripts
pnpm build
```

## Testing

### Run Rust tests

```bash
cargo test
```

### Run tests for specific crate

```bash
cargo test -p crisp-zk-witness
cargo test -p js-lib
```

### Run tests for the final JS bundle

```bash
cd js-scripts
pnpm test
```

## Usage

### Rust

```rust
use crisp_zk_witness::WitnessGenerator;

let generator = WitnessGenerator::new();
let public_key = generator.generate_public_key()?;
let witness = generator.generate_witness(&public_key, 1)?;
```

### JavaScript

The JavaScript library provides WASM bindings for witness generation. Here's how to use it:

#### Installation

Install the library.

```bash
pnpm install crisp-zk
```

#### Basic Usage

```javascript
import { JsWitnessGenerator } from "crisp-zk";

// Create a new witness generator instance
const generator = new JsWitnessGenerator();

// Generate a public key
const publicKey = await generator.generatePublicKey();
console.log("Public key:", publicKey);

// Generate a witness with the public key and vote (0 or 1)
const witness = await generator.generateWitness(publicKey, 1);
console.log("Witness:", witness);
```
