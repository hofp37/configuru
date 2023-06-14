use std::{fs::{read_to_string}, env, collections::HashMap};

use jsonc_parser::{ParseOptions, parse_to_serde_value};
use serde_json::{Value};

#[derive(Debug)]
enum ConvertedValue {
  Bool(bool),
  Number(i64),
  String(String),
}

fn create_config_storage(path: &str) -> HashMap<String, ConvertedValue> {
  let root_dir = env::current_dir().expect("Failed to get current directory");
  let json_file_path = root_dir.join(path);
  
  let contents = read_to_string(&json_file_path).unwrap();
  println!("File Content: {:?}", contents);

  let parse_options = ParseOptions {
    allow_comments: true,
    allow_loose_object_property_names: true,
    allow_trailing_commas: true,
  };

  let json_value_without_comments = parse_to_serde_value(&contents, &parse_options).unwrap();
  let serde_value = json_value_without_comments.unwrap();

  // let mut result_map: Map<String, ConvertValue> = Map::new();
  let mut result_map: HashMap<String, ConvertedValue> = HashMap::new();

  if let Value::Object(object) = serde_value {
    let keys: Vec<String> = object.keys().cloned().collect();
    
    for key in keys.iter() {
      if let Some(value) = object.get(key) {
        let converted_value = match value {
          Value::Bool(value) => ConvertedValue::Bool(true),
          Value::Number(value) => ConvertedValue::Number(42),
          Value::String(value) => ConvertedValue::String("string".to_string()),
          _ => panic!("Unsupported value type"),
        };

        result_map.insert(key.to_string(), converted_value);
      }
      
    }
  }

  return result_map;
}

fn main() {
  let config_storage = create_config_storage("env.jsonc");
  println!("Config Storage: {:?}", config_storage);

}
