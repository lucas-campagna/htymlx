use rust_yaml::Value;

pub struct Component(Value);

impl Component {
    pub fn new(value: Value) -> Self {
        Component(value)
    }

    pub fn to_json(&self) -> &Value {
        &self.0
    }

    pub fn to_html(&self) -> String {
        // Placeholder implementation
        format!("{:?}", self.0)
    }
}