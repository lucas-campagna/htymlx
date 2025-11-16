use rust_yaml::Value;
use rust_yaml::Yaml;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Document {
    pub templates: HashMap<String, Value>,
    pub components: HashMap<String, Value>,
    pub entry_points: HashMap<String, Value>,
}

impl Document {
    pub fn parse(code: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let yaml = Yaml::new();
        let binding = yaml.load_str(code)?;
        let parsed = binding.as_mapping().unwrap();

        let mut templates = HashMap::new();
        let mut components = HashMap::new();
        let mut entry_points = HashMap::new();

        for (key, value) in parsed {
            let key = key.as_str().unwrap();
            if key.starts_with("$(") && key.ends_with(")") {
                let selector = &key[2..key.len() - 1];
                entry_points.insert(selector.to_string(), value.clone());
            } else if key.starts_with("$") {
                let key = &key[1..];
                templates.insert(key.to_string(), value.clone());
            } else {
                components.insert(key.to_string(), value.clone());
            }
        }

        Ok(Self {
            templates,
            components,
            entry_points,
        })
    }

    pub fn render(self, target: &str) -> Option<String> {
        let props = self.components.get(target)?;
        let from = parse_from(props);
        Some(from?.to_string())
    }
}

fn parse_from(value: &Value) -> Option<&str> {
    match value {
        Value::String(v) => Some(v),
        _ => None,
    }
}

fn parse_body(value: &Value) -> Option<&str> {
    match value {
        Value::Null => None,
        Value::String(v) => Some(v),
        Value::Mapping(v) => {
            let key = Value::String(String::from("body"));
            match v.get(&key) {
                _ => Some(""),
            }
        }
        _ => panic!("From need to be empty or string"),
    }
}
