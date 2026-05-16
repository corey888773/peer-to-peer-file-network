use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug)]
pub enum Value {
    Int(i64),
    Str(String),
    Map(HashMap<String, Value>),
    List(Vec<Value>),
}

pub fn encode(value: &Value) -> String {
    match value {
        Value::Int(i) => encode_int(i),
        Value::Str(s) => encode_string(s),
        Value::Map(m) => encode_map(m),
        Value::List(l) => encode_list(l),
    }
}

fn encode_int(value: &i64) -> String {
    return format!("i{}e", *value);
}

fn encode_string(value: &String) -> String {
    let value_length: usize = value.len();
    return format!("{value_length}:{value}");
}

fn encode_map(value: &HashMap<String, Value>) -> String {
    let mut keys: Vec<&String> = value.keys().collect();
    keys.sort();

    let mut out: String = String::new();
    out.push('d');
    for key in keys {
        write!(out, "{}{}", encode_string(&key), encode(&value[key])).unwrap();
    }

    out.push('e');
    out
}

fn encode_list(value: &Vec<Value>) -> String {
    let mut out: String = String::new();
    out.push('l');
    value
        .iter()
        .for_each(|v| write!(out, "{}", encode(&v)).unwrap());
    out.push('e');
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn official_bencoding_examples() {
        // examples from https://bittorrent.org/beps/bep_0003.html

        let cases = [
            (Value::Str("spam".into()), "4:spam"),
            (Value::Int(3), "i3e"),
            (Value::Int(-3), "i-3e"),
            (Value::Int(0), "i0e"),
            (
                Value::List(vec![Value::Str("spam".into()), Value::Str("eggs".into())]),
                "l4:spam4:eggse",
            ),
            (
                Value::Map(HashMap::from([
                    ("cow".to_string(), Value::Str("moo".into())),
                    ("spam".to_string(), Value::Str("eggs".into())),
                ])),
                "d3:cow3:moo4:spam4:eggse",
            ),
            (
                Value::Map(HashMap::from([(
                    "spam".to_string(),
                    Value::List(vec![Value::Str("a".into()), Value::Str("b".into())]),
                )])),
                "d4:spaml1:a1:bee",
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                encode(&input),
                expected,
                "bencoding failed for {:#?}",
                input
            );
        }
    }

    #[test]
    fn map_keys_must_be_sorted_in_alphabetical_order() {
        // examples from https://bittorrent.org/beps/bep_0003.html

        let cases = [
            (
                Value::Map(HashMap::from([
                    ("cow".to_string(), Value::Str("moo".into())),
                    ("spam".to_string(), Value::Str("eggs".into())),
                ])),
                "d3:cow3:moo4:spam4:eggse",
            ),
            (
                Value::Map(HashMap::from([
                    ("spam".to_string(), Value::Str("eggs".into())),
                    ("cow".to_string(), Value::Str("moo".into())), // order changed
                ])),
                "d3:cow3:moo4:spam4:eggse",
            ),
            (
                Value::Map(HashMap::from([
                    ("11".to_string(), Value::Str("moo".into())),
                    ("12".to_string(), Value::Str("eggs".into())),
                ])),
                "d2:113:moo2:124:eggse",
            ),
            (
                Value::Map(HashMap::from([
                    ("12".to_string(), Value::Str("eggs".into())),
                    ("11".to_string(), Value::Str("moo".into())), // order changed
                ])),
                "d2:113:moo2:124:eggse",
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(
                encode(&input),
                expected,
                "bencoding failed for {:#?}",
                input
            );
        }
    }
}
