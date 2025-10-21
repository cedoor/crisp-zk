//! JavaScript library with WASM bindings for witness generation
//!
//! This crate provides JavaScript bindings for the witness generator using WASM.

use crisp_zk_witness::WitnessGenerator;
use js_sys;
use wasm_bindgen::prelude::*;

/// JavaScript-compatible witness generator
#[wasm_bindgen]
pub struct JsWitnessGenerator {
    generator: WitnessGenerator,
}

#[wasm_bindgen]
impl JsWitnessGenerator {
    /// Create a new JavaScript witness generator
    #[wasm_bindgen(constructor)]
    pub fn new() -> JsWitnessGenerator {
        JsWitnessGenerator {
            generator: WitnessGenerator::new(),
        }
    }

    /// Generate a witness from JavaScript
    #[wasm_bindgen(js_name = "generateWitness")]
    pub fn generate_witness(&self, public_key: &str, vote: u8) -> Result<JsValue, JsValue> {
        match self.generator.generate_witness(public_key, vote) {
            Ok(witness_json) => {
                // Parse the JSON string and return as JsValue
                match js_sys::JSON::parse(&witness_json) {
                    Ok(js_value) => Ok(js_value),
                    Err(_) => Err(JsValue::from_str("Failed to parse witness JSON")),
                }
            }
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }

    /// Generate a public key from JavaScript
    #[wasm_bindgen(js_name = "generatePublicKey")]
    pub fn generate_public_key(&self) -> Result<String, JsValue> {
        match self.generator.generate_public_key() {
            Ok(public_key) => Ok(public_key),
            Err(e) => Err(JsValue::from_str(&e)),
        }
    }

    /// Get the version of the library
    #[wasm_bindgen]
    pub fn version() -> String {
        env!("CARGO_PKG_VERSION").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_js_witness_generation() {
        let generator = JsWitnessGenerator::new();
        let public_key = generator.generator.generate_public_key().unwrap();
        let result = generator.generate_witness(&public_key, 1);

        assert!(result.is_ok());

        let witness = result.unwrap();

        // Convert JsValue to string for testing
        let witness_str = js_sys::JSON::stringify(&witness)
            .unwrap()
            .as_string()
            .unwrap();
        assert!(witness_str.contains("params"));
        assert!(witness_str.contains("pk0is"));
    }
}
