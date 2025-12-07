use std::ops::{Deref, DerefMut};
// use super::utils::ApplyTypes;
use rust_yaml::Value;

use crate::renderes::html::render;


#[derive(Clone)]
pub struct Component(Value);

impl Default for Component {
    fn default() -> Self {
        Component(Value::Null)
    }
}

impl Deref for Component {
    type Target = Value;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Component {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// pub enum FieldAccessError {
//     KeyNotFound,
//     InvalidType,
// }

impl Component {
    pub fn build(value: Value) -> Self {
        Self(value)
    }

    // pub fn get_field<'a>(&'a self, key: &str) -> Result<&'a Value, FieldAccessError> {
    //     match self.deref() {
    //         Value::Mapping(m) =>
    //             m.get(&Value::String(String::from(key))).ok_or(FieldAccessError::KeyNotFound),
    //         _ => Err(FieldAccessError::InvalidType)
    //     }
    // }

    // pub fn set_field(&mut self, key: &str, value: &Value) -> Result<(), FieldAccessError> {
    //     match self.deref_mut() {
    //         Value::Mapping(m) => {
    //             let key = Value::String(key.to_owned());
    //             m.insert(key, value.to_owned());
    //             Ok(())
    //         }
    //         _ => Err(FieldAccessError::InvalidType)
    //     }
    // }

    // pub fn apply(&mut self, comp: &Component) -> Result<&mut Self, FieldAccessError> {
    //     self.apply_value(comp)
    // }

    // pub fn apply_value(&mut self, props: &Value) -> Result<&mut Self, FieldAccessError> {
    //     let target = match &mut **self {
    //         Value::Sequence(target) => ApplyTypes::VecMut(target),
    //         Value::Mapping(target) => ApplyTypes::IndexMapMut(target),
    //         target => ApplyTypes::ValueMut(target),
    //     };
    //     let source = match props {
    //         Value::Sequence(source) => ApplyTypes::Vec(source),
    //         Value::Mapping(source) => ApplyTypes::IndexMap(source),
    //         source => ApplyTypes::Value(source),
    //     };
    //     apply(target, source)?;
    //     Ok(self)
    // }

    // fn merge_value(&mut self, source: &Value) -> Result<(), FieldAccessError> {
    //     if *self.type_name() != *source.type_name() {
    //         return Err(FieldAccessError::InvalidType);
    //     }
    //     match (self.deref_mut(), source) {
    //         (Value::Mapping(target), Value::Mapping(source)) => 
    //             source.iter()
    //             .for_each(|(key, value)| {
    //                 target.insert(key.to_owned(), value.to_owned());
    //             }),
    //         (Value::Sequence(target), Value::Sequence(source)) =>
    //             target.append(&mut source.to_owned()),
    //         (target, source) =>
    //             *target = source.to_owned(),
    //     };
    //     Ok(())
    // }

    // fn merge(&mut self, source: &Self) -> Result<(), FieldAccessError> {
    //     self.merge_value(source)
    // }
    pub fn to_html(&self) -> String {
        render(self)
    }
}
