use std::collections::HashMap;
use std::ops::Deref;
use rust_yaml::Value;
use crate::map_get;

use super::component::Component;
use super::utils::{is_template, apply_props};

pub struct Runtime {
    pub components: HashMap<String, Component>,
}

pub enum RuntimeError {
    InvalidFormat,
    ComponentCallError,
}

impl Runtime {
    pub fn new() -> Self {
        Runtime {
            components: HashMap::new()
        }
    }

    pub fn add_many(&mut self, comps: Value) -> Result<(), RuntimeError>{
        match comps {
            Value::Mapping(comps) => {
                for (name, value) in comps.iter() {
                    let name = match name {
                        Value::String(name) => name,
                        _ => panic!("Invalid component name type"),
                    };
                    self.add(name, value.to_owned());
                };
                Ok(())
            },
            _ => Err(RuntimeError::InvalidFormat)
        }
    }

    pub fn add(&mut self, name: &str, comp: Value) {
        self.components.insert(name.to_owned(), Component::build(comp));
    }

    fn get_component(&self, name: &str) -> Option<&Component> {
        self.components.get(name)
    }

    fn get_template(&self, name: &str) -> Option<&Component> {
        let name = String::from("$") + name;
        self.get_component(name.as_str())
    }

    pub fn call(&self, name: &str, props: Value) -> Result<Value, RuntimeError> {
        let props = Component::build(props);
        let value = self.call_impl(name, props)?;
        Ok(value.deref().clone())
    }

    pub fn call_impl(&self, name: &str, props: Component) -> Result<Component, RuntimeError> {
        println!("name={}", name);
        let process_comp = |mut target: Component, mut source: Component | -> Option<Component> {
            if apply_props(&mut target, &mut source).is_err() {
                return None;
            }
            if let Some(from) = target
                .as_mapping()
                .and_then(|t| map_get!(t, "from")) &&
                let Value::String(from) = from &&
                let Some(comp) = self.call_impl(&from, target.clone()).ok() &&
                apply_props(&mut target, &mut comp.clone()).is_err() {
                    return None;
            }
            Some(target)
        };
        let is_temp = is_template(name);
        let (comp1, comp2) = if is_temp {
            (self.get_component(name), self.get_template(name))
        } else {
            (self.get_template(name), self.get_component(name))
        };
        if comp1.or(comp2).is_none() {
            return Ok(props);
        }
        let comp1 = comp1
            .and_then(|comp| 
                process_comp(comp.clone(), props)
        );
        let result = comp2
            .and_then(|comp|
                process_comp(comp.clone(), comp1.unwrap_or_default())
            )
            .ok_or_else(||
                RuntimeError::ComponentCallError
            );
        if let Ok(comp) = result.as_ref() {
            println!("{}: {}", name, comp.to_string());
        }
        result
    }
}