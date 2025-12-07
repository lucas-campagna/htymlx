mod component;
mod utils;
mod runtime;

use component::Component;
use runtime::Runtime;
use rust_yaml::{Value, Yaml};

use crate::parser::runtime::RuntimeError;

pub struct Parser(Runtime);

impl Parser {
    pub fn from(code: &str) -> Result<Parser, Box<dyn std::error::Error>> {
        let yaml = Yaml::new();
        let components = yaml.load_str(code)?;
        let mut runtime = Runtime::new();
        let _ = runtime.add_many(components);
        Ok(Parser(runtime))
    }
    pub fn call(&self, name: &str, props: Value) -> Result<Component, RuntimeError> {
        Ok(Component::build(self.0.call(name, props)?))
    }
}