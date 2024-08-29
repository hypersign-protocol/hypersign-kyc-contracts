use serde_json::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

// Load and merge JSON-LD contexts from local files
fn load_local_context(context_paths: &[&str]) -> HashMap<String, String> {
    let mut context_map = HashMap::new();

    for path in context_paths {
        // Read the context file
        let mut file = File::open(path).expect("Failed to open context file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read context file");

        // Parse the context file as JSON
        let context_value: Value = serde_json::from_str(&contents).expect("Failed to parse context JSON");

        // Handle both object and array forms of @context
        if let Value::Array(context_array) = context_value {
            for item in context_array {
                if let Value::Object(context_object) = item {
                    for (key, value) in context_object {
                        if let Value::String(uri) = value {
                            context_map.insert(key.clone(), uri.clone());
                        }
                    }
                }
            }
        } else if let Value::Object(context_object) = context_value {
            for (key, value) in context_object {
                if let Value::String(uri) = value {
                    context_map.insert(key.clone(), uri.clone());
                }
            }
        }
    }

    context_map
}

// Example usage
fn main() {
    let context_paths = [
            "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context1.json", // Replace with actual local context file paths
            "/home/ubuntu/backend-drf/hypersign/hypersign-kyc-contracts/contracts/ssi-manager/context2.json",
        ];
    let context_map = load_local_context(&context_paths);

    // Output the context map
    for (term, uri) in &context_map {
        println!("Term: {}, URI: {}", term, uri);
    }
}