#[cfg(test)]
mod test {
  use htymlx::Parser;
  use rust_yaml::Value;
  
  #[test]
  fn test_null_component() {
    let r = Parser::from(r"
box: null
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "");
  }
  
  #[test]
  fn test_number_component() {
    let r = Parser::from(r"
box: 123
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "123");
  }
  
  #[test]
  fn test_string_component() {
    let r = Parser::from(r"
box: hello world
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "hello world");
  }
  
  #[test]
  fn test_body_only() {
    let r = Parser::from(r"
box:
    body: hello world
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "hello world");
  }

  #[test]
  fn test_from_only() {
    let r = Parser::from(r"
box:
    from: div
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "<div></div>");
  }
  
  #[test]
  fn test_from_body_component() {
    let r = Parser::from(r"
box:
    from: div
    body: hello world
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "<div>hello world</div>");
  }
  
  #[test]
  fn test_from_body_component_with_props() {
    let r = Parser::from(r"
box:
  from: div
  class: bg-red-100
  body: hello world
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "<div class=\"bg-red-100\">hello world</div>");
  }
  
  #[test]
  fn test_sequence_component_simple() {
    let r = Parser::from(r"
box:
  - abc
  - 123
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "abc123");
  }
  
  #[test]
  fn test_sequence_component_with_component_inside() {
    let r = Parser::from(r"
box:
  - from: div
    body: hello world
  - 123
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "<div>hello world</div>123");
  }

  #[test]
  fn test_sequence_component_with_component_inside_and_null() {
    let r = Parser::from(r"
box:
  - from: div
    body: hello world
  - 123
  - null
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "<div>hello world</div>123");
  }

  #[test]
  fn test_component_calling() {
    let r = Parser::from(r"
box1:
  from: div
box:
  from: box1
  body: hello world
").unwrap();
    let comp = r.call("box", Value::Null).ok();
    assert!(comp.is_some());
    assert_eq!(comp.unwrap().to_html(), "<div>hello world</div>");
  }
}