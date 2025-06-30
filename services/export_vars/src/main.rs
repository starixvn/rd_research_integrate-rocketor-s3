use serde_json;
use std::env;

fn main() {
    let env_name = env::var("TRANSFORM_CONFIG").expect("TRANSFORM_CONFIG is not set");
    let vars_data = env::var("VARS_DATA").expect("VARS_DATA is not set");
    let vars_data: serde_json::Value = serde_json::from_str(&vars_data).unwrap();
    let config = vars_data["services"][0][format!("config_{}", env_name)].clone();
    let config: serde_json::Value =
        serde_json::from_value(config).expect("Failed to deserialize config");

    // Export config as environment variables
    export_config_as_env_vars(&config);
}

fn export_config_as_env_vars(config: &serde_json::Value) {
    match config {
        serde_json::Value::Object(map) => {
            for (key, value) in map {
                let env_var_name = key.to_uppercase();
                let env_var_value = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => String::new(),
                    serde_json::Value::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
                    serde_json::Value::Object(obj) => {
                        serde_json::to_string(obj).unwrap_or_default()
                    }
                };

                // Print export command for shell sourcing
                println!("export {}={}", env_var_name, env_var_value);
            }
        }
        _ => {
            eprintln!("Config is not an object, cannot export as environment variables");
        }
    }
}
