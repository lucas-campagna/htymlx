use rust_yaml::Value;

pub fn render(obj: &Value) -> String {
    match obj {
        Value::Mapping(map) => {
            let from = map.get(&Value::String("from".to_string()));
            let body = map.get(&Value::String("body".to_string()));
            let has_from = from.is_some();
            let has_body = body.is_some();
            let props = {
                let mut result = Vec::new();
                for (k, v) in map {
                    if k.is_string() &&
                        (has_from && k.as_str().unwrap().to_string() == "from" || has_body && k.as_str().unwrap().to_string() == "body") {
                        continue;
                    }
                    fn check_is_valid_type(k: &Value) -> bool {
                        k.is_string() || k.is_bool() || k.is_number()
                    }
                    let is_prop = check_is_valid_type(k) && check_is_valid_type(v);
                    if is_prop {
                        result.push(format!("{}=\"{}\"", render(k), render(v)).to_owned());
                    }
                }
                result
            };
            let props = if props.len() == 0 {"".to_owned()} else {" ".to_owned() + &props.join(" ")};
            match (from, body) {
                (None, None) => "".to_string(),
                (None, Some(body)) => render(body),
                (Some(from), None) => format!("<{from}{props}></{from}>", from=render(from), props=props),
                (Some(from), Some(body)) => format!("<{from}{props}>{body}</{from}>", from=render(from), body=render(body), props=props),
            }
        },
        Value::Sequence(v) => v.iter().map(|obj| render(obj)).collect::<Vec<String>>().join(""),
        Value::Null => "".to_string(),
        v => if v.is_string() {v.as_str().unwrap_or_default().to_string()} else {format!("{}", v)},
    }
}

#[cfg(test)]
mod test {
    use crate::json;
    use super::*;

    #[test]
    fn render_with_from_and_body() {
        let obj = json!({
            from: "div",
            body: "hello world",
        });
        assert_eq![render(&obj), "<div>hello world</div>".to_string()];
    }
}
