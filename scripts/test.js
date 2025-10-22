import { ZKInputsGenerator } from "../crates/js-lib/pkg/crisp_zk.js";
import { UltraHonkBackend } from '@aztec/bb.js';
import circuitJson from './crisp.json' with { type: 'json' };
import { Noir } from '@noir-lang/noir_js';

const generator = new ZKInputsGenerator();
const publicKey = await generator.generatePublicKey();
const oldCiphertext = await generator.encryptVote(publicKey, 0);
const inputs = await generator.generateInputs(oldCiphertext, publicKey, 1);

const noir = new Noir(circuitJson);
const backend = new UltraHonkBackend(circuitJson.bytecode);

const { witness } = await noir.execute(inputs);

// Measure the time it needs to generate a proof.
const startTime = performance.now();

const proof = await backend.generateProof(witness);

const endTime = performance.now();
const generationTime = endTime - startTime;

console.log(`Proof generated in ${generationTime.toFixed(2)}ms`);

const verificationResult = await backend.verifyProof(proof);

console.log(`Proof verified: ${verificationResult}`);

await backend.destroy();