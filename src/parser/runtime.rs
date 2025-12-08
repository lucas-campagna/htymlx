use super::apply::apply;
use super::utils::is_template;
use rust_yaml::{Value, Error};

pub struct Runtime<'a>(&'a Value);

impl Runtime<'_> {
    pub fn new(value: &Value) -> Runtime<'_> {
        Runtime(value)
    }

    fn call_template(&self, name: &str, props: &Value) -> Result<Value, Error> {
        let name = "$".to_owned() + name;
        self.call(name.as_str(), props)
    }

    pub fn call(&self, name: &str, props: &Value) -> Result<Value, Error> {
        let is_template = is_template(name);
        let name = Value::String(name.into());
        let component = self
            .0
            .as_mapping()
            .unwrap()
            .get(&name);
        let has_template = self
            .0
            .as_mapping()
            .unwrap()
            .get(&name)
            .is_some();
        if component.is_none() && !has_template {
            return Ok(Value::Null);
        }
        let component = component.unwrap_or(&Value::Null);
        if is_template {
            let mut component = apply(component, props);
            if let Some(from) = component.as_mapping().and_then(|m| m.get(&Value::String("from".into()))) {
                component = match self.call(from.as_str().unwrap(), &component)? {
                    Value::Null => component,
                    other => other,
                }
            }
            if has_template {
                return self.call_template(name.as_str().unwrap(), &component);
            }
            Ok(component)
        } else {
            let template = if has_template {
                self.call_template(name.as_str().unwrap(), props)?
            } else {
                Value::Null
            };
            let component = apply(&component, &template);
            let component = apply(&component, props);
            Ok(component.clone())
        }
    }
}