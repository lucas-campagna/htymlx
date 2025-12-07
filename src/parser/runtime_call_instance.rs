#[allow(unused_variables, unused)]

use std::ops::Deref;

use rust_yaml::Value;
use crate::parser::utils::{apply_props, is_template};

use super::component::Component;
use super::runtime::{Runtime, RuntimeError};

pub struct RuntimeCallInstance<'a> {
    pub runtime: &'a Runtime,
    pub buffer: Component,
}

impl<'a> Deref for RuntimeCallInstance<'a> {
    type Target = Runtime;
    fn deref(&self) -> &Self::Target {
        self.runtime
    }
}

impl RuntimeCallInstance<'_> {
    
    pub fn call<'a>(&'a mut self, name: &str, props: &Value) -> Result<&'a Component, RuntimeError> {
        let is_temp = is_template(name);
        let process_comp = |comp| {
            apply_props(&mut self.buffer, props);
            let comp = comp;
            Some(comp)
        };
        if is_temp {
            self.get_component(name)
                .and_then(process_comp);
            return self.get_template(name)
                .and_then(process_comp)
                .and_then(|v| Some(v))
                .ok_or(RuntimeError::ComponentCallError);
        } 
        self.get_template(name)
            .and_then(process_comp);
        self.get_component(name)
            .and_then(process_comp)
            .and_then(|v| Some(v))
            .ok_or(RuntimeError::ComponentCallError)
    }    
}