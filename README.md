# Crisp ZK

A library for generating ZK inputs to create CRISP proofs.

## Structure

This repository contains two main crates:

- **`generator`**: Core ZK inputs generation logic written in Rust.
- **`js-lib`**: JavaScript library with WASM bindings for the ZK inputs generator.

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
cd scripts
pnpm build
```

## Testing

### Run Rust tests

```bash
cargo test
```

### Run tests for specific crate

```bash
cargo test -p crisp-zk-inputs
cargo test -p crisp-zk
```

### Run tests for the final JS bundle

```bash
cd scripts
pnpm test
```

## Usage

### Rust

```rust
use crisp_zk_inputs::CrispZKInputsGenerator;

let generator = CrispZKInputsGenerator::new();
let public_key = generator.generate_public_key()?;
let old_ciphertext = generator.encrypt_vote(&public_key, 0)?;
let inputs = generator.generate_inputs(&old_ciphertext, &public_key, 1)?;
```

### JavaScript

The JavaScript library provides WASM bindings for ZK inputs generation. Here's how to use it:

#### Installation

Install the library.

```bash
pnpm install crisp-zk
```

#### Basic Usage

```javascript
import { ZKInputsGenerator } from "crisp-zk";

// Create a new ZK inputs generator instance
const generator = new ZKInputsGenerator();

// Generate a public key
const publicKey = await generator.generatePublicKey();
console.log("Public key:", publicKey);

// Encrypt a vote to create an old ciphertext
const oldCiphertext = await generator.encryptVote(publicKey, 0);
console.log("Old ciphertext:", oldCiphertext);

// Generate ZK inputs with the old ciphertext, public key, and new vote (0 or 1)
const inputs = await generator.generateInputs(oldCiphertext, publicKey, 1);
console.log("ZK inputs:", inputs);
```
