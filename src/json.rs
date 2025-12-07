#[macro_export]
macro_rules! key {
    ($k:expr) => {
        &Value::String($k.to_string())
    };
}

#[macro_export]
macro_rules! map_get {
    ($v:expr, $k:expr) => {
        $v.get(&Value::String($k.to_string()))
    };
}

#[macro_export]
macro_rules! json_get {
    ($v:expr, $k:expr) => {
        $v.as_mapping().unwrap().get(&Value::String($k.to_string())).unwrap()
    };
}


#[macro_export]
macro_rules! json {
    // 1. Recursive rule for arrays (Sequences): Calls the main macro rule for elements
    ([$($value:tt),* $(,)?]) => {
        {
            Value::Sequence(vec![
                $(
                    json!($value)
                ),*
            ])
        }
    };

    ({$($key:tt : $value:tt),* $(,)?}) => {
        {
            // Use local imports to avoid cluttering the module scope
            let mut map = indexmap::IndexMap::new();
            $(
                let key = Value::String(stringify!($key).to_string());
                let value = json!($value);
                map.insert(key, value);
            )*
            Value::Mapping(map)
        }
    };

    // 3. Base case rule for literals: Uses the Into<Value> trait implementation
    //    for basic types (strings, numbers, booleans) to create a Value::* variant.
    ($value:expr) => {
        Value::from($value)
    }; 
}

#[cfg(test)]
mod test {
    use rust_yaml::Value;
    #[test]
    fn test_simple_int() {
        let a = json!(42);
        assert_eq!(a.as_int().unwrap(), 42);
    }

    #[test]
    fn test_simple_json() {
        let obj = json!({a:2, b:3});
        let a = Value::String("a".to_string());
        let b = Value::String("b".to_string());
        assert_eq!(obj.as_mapping().unwrap().get(&a).unwrap().as_int().unwrap(), 2);
        assert_eq!(obj.as_mapping().unwrap().get(&b).unwrap().as_int().unwrap(), 3);
    }

    #[test]
    fn test_simple_seq() {
        let obj = json!([1,2]);
        assert_eq!(obj.as_sequence().unwrap()[0].as_int().unwrap(), 1);
    }
}