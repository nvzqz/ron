extern crate ron;

use std::collections::BTreeMap;

use ron::value::{Number, Value};

#[test]
fn bool() {
    assert_eq!(Value::from_str("true"), Ok(Value::Bool(true)));
    assert_eq!(Value::from_str("false"), Ok(Value::Bool(false)));
}

#[test]
fn char() {
    assert_eq!(Value::from_str("'a'"), Ok(Value::Char('a')));
}

#[test]
fn map() {
    let mut map = BTreeMap::new();
    map.insert(Value::Char('a'), Value::Number(Number::new(1f64)));
    map.insert(Value::Char('b'), Value::Number(Number::new(2f64)));
    assert_eq!(Value::from_str("{ 'a': 1, 'b': 2 }"), Ok(Value::Map(map)));
}

#[test]
fn number() {
    assert_eq!(Value::from_str("42"), Ok(Value::Number(Number::new(42f64))));
    assert_eq!(Value::from_str("3.1415"), Ok(Value::Number(Number::new(3.1415f64))));
}

#[test]
fn option() {
    let opt = Some(Box::new(Value::Char('c')));
    assert_eq!(Value::from_str("Some('c')"), Ok(Value::Option(opt)));
}

#[test]
fn string() {
    let normal = "\"String\"";
    assert_eq!(Value::from_str(normal), Ok(Value::String("String".into())));

    let raw = "r\"Raw String\"";
    assert_eq!(Value::from_str(raw), Ok(Value::String("Raw String".into())));

    let raw_hashes = "r#\"Raw String\"#";
    assert_eq!(Value::from_str(raw_hashes), Ok(Value::String("Raw String".into())));

    let raw_escaped = "r##\"Contains \"#\"##";
    assert_eq!(Value::from_str(raw_escaped), Ok(Value::String("Contains \"#".into())));

    let raw_multi_line = "r\"Multi\nLine\"";
    assert_eq!(Value::from_str(raw_multi_line), Ok(Value::String("Multi\nLine".into())));
}

#[test]
fn seq() {
    let seq = vec![Value::Number(Number::new(1f64)), Value::Number(Number::new(2f64))];
    assert_eq!(Value::from_str("[1, 2]"), Ok(Value::Seq(seq)));
}

#[test]
fn unit() {
    assert_eq!(Value::from_str("()"), Ok(Value::Unit));
}
