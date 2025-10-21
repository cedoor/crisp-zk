//! JavaScript library with WASM bindings for crisp inputs generation
//!
//! This crate provides JavaScript bindings for the crisp inputs generator using WASM.

use crisp_zk_inputs::CrispZKInputsGenerator;
use js_sys;
use wasm_bindgen::prelude::*;

/// JavaScript-compatible crisp inputs generator
#[wasm_bindgen]
pub struct ZKInputsGenerator {
    generator: CrispZKInputsGenerator,
}

#[wasm_bindgen]
impl ZKInputsGenerator {
    /// Create a new JavaScript crisp inputs generator
    #[wasm_bindgen(constructor)]
    pub fn new() -> ZKInputsGenerator {
        ZKInputsGenerator {
            generator: CrispZKInputsGenerator::new(),
        }
    }

    /// Generate a crisp inputs from JavaScript
    #[wasm_bindgen(js_name = "generateInputs")]
    pub fn generate_inputs(&self, public_key: &str, vote: u8) -> Result<JsValue, JsValue> {
        match self.generator.generate_inputs(public_key, vote) {
            Ok(inputs_json) => {
                // Parse the JSON string and return as JsValue
                match js_sys::JSON::parse(&inputs_json) {
                    Ok(js_value) => Ok(js_value),
                    Err(_) => Err(JsValue::from_str("Failed to parse inputs JSON")),
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
    fn test_js_inputs_generation() {
        let generator = ZKInputsGenerator::new();
        let public_key = generator.generator.generate_public_key().unwrap();
        let result = generator.generate_inputs(&public_key, 1);

        assert!(result.is_ok());

        let inputs = result.unwrap();

        // Convert JsValue to string for testing
        let inputs_str = js_sys::JSON::stringify(&inputs)
            .unwrap()
            .as_string()
            .unwrap();
        assert!(inputs_str.contains("params"));
        assert!(inputs_str.contains("pk0is"));
    }
}
