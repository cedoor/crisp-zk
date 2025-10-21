import { JsWitnessGenerator } from "../crates/js-lib/pkg/js_lib.js";
import { UltraHonkBackend } from '@aztec/bb.js';
import circuitJson from './crisp-circuit.json' with { type: 'json' };
import { Noir } from '@noir-lang/noir_js';

const generator = new JsWitnessGenerator();
const publicKey = await generator.generatePublicKey();
const proverInputs = await generator.generateWitness(publicKey, 1);

const noir = new Noir(circuitJson);
const backend = new UltraHonkBackend(circuitJson.bytecode);

const { witness } = await noir.execute(proverInputs);

// Measure the time it needs to generate a proof.
const startTime = performance.now();

await backend.generateProof(witness);

const endTime = performance.now();
const generationTime = endTime - startTime;

await backend.destroy();

console.log(`Proof generated in ${generationTime.toFixed(2)}ms`);