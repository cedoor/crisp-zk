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
let public_key = generator.generate_public_key()?;
let witness = generator.generate_witness(&public_key, 1)?;
```

### JavaScript

The JavaScript library provides WASM bindings for witness generation. Here's how to use it:

#### Installation

```bash
# Build the WASM library
cd crates/js-lib
wasm-pack build

# Or use the example setup
cd example
pnpm install
```

#### Basic Usage

```javascript
import { JsWitnessGenerator } from "./pkg/js_lib.js";

// Create a new witness generator instance
const generator = new JsWitnessGenerator();

// Generate a public key
const publicKey = await generator.generatePublicKey();
console.log("Public key:", publicKey);

// Generate a witness with the public key and vote (0 or 1)
const witness = await generator.generateWitness(publicKey, 1);
console.log("Witness:", witness);
```

#### API Reference

##### `JsWitnessGenerator`

The main class for generating witnesses in JavaScript.

**Constructor:**

```javascript
const generator = new JsWitnessGenerator();
```

**Methods:**

- `generatePublicKey()`: `Promise<string>`

  - Generates a new public key
  - Returns a string representation of the public key

- `generateWitness(publicKey: string, vote: number)`: `Promise<any>`

  - Generates a witness for the given public key and vote
  - `publicKey`: The public key string (from `generatePublicKey()`)
  - `vote`: The vote value (0 or 1)
  - Returns a JavaScript object containing the witness data

- `version()`: `string` (static method)
  - Returns the version of the library
  - Usage: `JsWitnessGenerator.version()`

#### Complete Example with Noir Integration

```javascript
import { JsWitnessGenerator } from "./pkg/js_lib.js";
import { UltraHonkBackend } from '@aztec/bb.js';
import circuitJson from './crisp-circuit.json' with { type: 'json' };
import { Noir } from '@noir-lang/noir_js';

// Initialize the witness generator
const generator = new JsWitnessGenerator();

// Generate public key and witness
const publicKey = await generator.generatePublicKey();
const proverInputs = await generator.generateWitness(publicKey, 1);

// Set up Noir and backend
const noir = new Noir(circuitJson);
const backend = new UltraHonkBackend(circuitJson.bytecode);

// Execute the circuit and generate proof
const { witness } = await noir.execute(proverInputs);

// Measure proof generation time
const startTime = performance.now();
await backend.generateProof(witness);
const endTime = performance.now();

console.log(`Proof generated in ${(endTime - startTime).toFixed(2)}ms`);

// Clean up
await backend.destroy();
```

#### Error Handling

The JavaScript API returns promises that can reject with error messages:

```javascript
try {
  const generator = new JsWitnessGenerator();
  const publicKey = await generator.generatePublicKey();
  const witness = await generator.generateWitness(publicKey, 1);
} catch (error) {
  console.error("Error generating witness:", error);
}
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
