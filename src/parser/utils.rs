use std::sync::LazyLock;

use indexmap::IndexMap;
use rust_yaml::{Value, Yaml};
use regex::Regex;

static YAML: LazyLock<Yaml> = LazyLock::new(|| Yaml::new());
static VAR_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\$\w+").unwrap());

pub fn is_template(name: &str) -> bool{
    name.starts_with("$")
}

// pub enum ApplyTypes<'a> {
//     Vec(&'a Vec<Value>),
//     IndexMap(&'a IndexMap<Value, Value>),
//     Value(&'a Value),
//     VecMut(&'a mut Vec<Value>),
//     IndexMapMut(&'a mut IndexMap<Value, Value>),
//     ValueMut(&'a mut Value),
// }

fn get_num_of_props(prop: &Value) -> usize {
    match prop {
        Value::String(target) => 
            VAR_RE
                .find_iter(target)
                .count(),
        Value::Sequence(target) => 
            target
                .iter()
                .fold(0, |acc, cur| acc + get_num_of_props(cur)),
        Value::Mapping(target) => 
            target
                .values()
                .fold(0, |acc, cur| acc + get_num_of_props(cur)),
        _ => 0,
    }
}

pub enum ApplyPropsError {
    InvalidKey,
}

pub fn apply_prop(target: &mut Value, key: &str, value: &mut Value) -> Result<bool, ApplyPropsError> {
    let has_key = key.len() > 0;
    let num_of_props = get_num_of_props(target);
    if has_key && num_of_props > 1 {
        return Err(ApplyPropsError::InvalidKey);
    }
    fn apply_prop_recursive(target: &mut Value, key: &str, value: &mut Value) -> usize {
        match target {
            Value::String(target_str) => {
                if target_str.trim() == key.trim() {
                    *target = value.clone();
                    return 1;
                }
                let re = Regex::new(format!("${}", key).as_str()).unwrap();
                let count = re.find_iter(target_str).count();
                match value.clone() {
                    Value::Null => 
                        re.replace_all(target_str, ""),
                    Value::Bool(b) => 
                        re.replace_all(target_str, if b {"true"} else {"false"}),
                    Value::Int(n) => 
                        re.replace_all(target_str, n.to_string()),
                    Value::Float(n) => 
                        re.replace_all(target_str, n.to_string()),
                    Value::String(s) => 
                        re.replace_all(target_str, s.to_string()),
                    // TODO: Review later
                    values => re.replace_all(target_str, YAML.dump_str(&values).unwrap_or_default()),
                };
                value.clone_from(&&Value::Null);
                count
            },
            Value::Sequence(target) => 
                target
                    .iter_mut()
                    .fold(0, |acc, target| acc + apply_prop_recursive(target, key, value)),
            Value::Mapping(target) => 
                target
                    .values_mut()
                    .fold(0, |acc, target| acc + apply_prop_recursive(target, key, value)),
            _ => 0,
        }
    }
    Ok(apply_prop_recursive(target, key, value) > 0)
}


pub fn apply_props(target: &mut Value, source: &mut Value) -> Result<bool, ApplyPropsError> {
    match source {
        Value::Sequence(source_seq) => {
            if source_seq.len() == 0 {
                return Ok(true);
            }
            let mut target_cloned = vec![target.clone(); source_seq.len()];
            let mut iter = target_cloned
                .iter_mut()
                .map(|target| apply_props(target, source));
            let result = iter
                .find(|r| r.is_err())
                .unwrap_or(Ok(true));
            *target = Value::Sequence(target_cloned.to_owned());
            result
        },
        Value::Mapping(map) => {
            if map.len() == 0 {
                return Ok(true);
            }
            let parsed = map
                .drain(..)
                .fold(IndexMap::new(), |mut acc, (key, value)| {
                    let mut value = value.clone();
                    if let Some(key) = key.as_str() && apply_prop(target, key, &mut value).unwrap_or(false) && !value.is_null() {
                        acc.insert(Value::String(key.to_string()), value);
                    }
                    acc
                });
            *source = Value::Mapping(parsed.clone());
            Ok(true)
        },
        Value::Null => Ok(true),
        source => apply_prop(target, "", source)
    }
}


#[cfg(test)]
mod test {
    use crate::{json, json_get};
    use super::*;

    #[test]
    fn test_apply_prop_01() {
        let mut target = json!({
            key: "$var",
        });
        let mut source = json!({
            var: 10,
        });
        apply_prop(&mut target, "var", &mut source);
        assert_eq!(json_get!(target, "key").as_int().unwrap(), 10);
    }
}