use std::collections::HashMap;

pub enum RedisValue {
    SimpleString(String),
    SimpleError(String),
    Integer(i64),
    BulkString(String),
    NullBulkString,
    Array(Vec<RedisValue>),
    NullArray,
    Null,
    Boolean(bool),
    BigInteger(i128),
    BulkError(String),
    VerbatimString(String, String),
    Map(HashMap<RedisValue, RedisValue>),
    Set(Vec<RedisValue>),
    Push(Vec<RedisValue>),
}

impl RedisValue {
    pub fn to_string(&self) -> String {
        match self {
            RedisValue::SimpleString(text) => {
                format!("+{text}\r\n")
            },
            RedisValue::SimpleError(text) => {
                format!("-{text}\r\n")
            },
            RedisValue::Integer(num) => {
                format!(":{num}\r\n")
            },
            RedisValue::BulkString(text) => {
                format!("${}\r\n{}\r\n", text.len(), text)
            },
            RedisValue::NullBulkString => {
                "$-1\r\n".into()
            },
            RedisValue::Array(vec) => {
                let mut string = String::new();
                string.push_str(&format!("*{}\r\n", vec.len()));
                vec.into_iter().for_each(|item| string.push_str(&item.to_string()));
                string
            },
            RedisValue::NullArray => {
                "*-1\r\n".into()
            },
            RedisValue::Null => {
                "_\r\n".into()
            },
            RedisValue::Boolean(value) => {
                if *value {
                    "#t\r\n".into()
                } else {
                    "#f\r\n".into()
                }
            },
            RedisValue::BigInteger(num) => {
                format!("({num}\r\n")
            },
            RedisValue::BulkError(text) => {
                format!("!{}\r\n{}\r\n", text.len(), text)
            },
            RedisValue::VerbatimString(encoding, text) => {
                format!("={}\r\n{}:{}\r\n", text.len(), encoding, text)
            },
            RedisValue::Map(map) => {
                let mut string = String::new();
                string.push_str(&format!("&{}\r\n", map.len()));
                map.into_iter().for_each(|(key, value)| {
                    string.push_str(&key.to_string());
                    string.push_str(&value.to_string());
                });
                string
            },
            RedisValue::Set(set) => {
                let mut string = String::new();
                string.push_str(&format!("~{}\r\n", set.len()));
                for item in set {
                    let item_str = item.to_string();
                    if string.contains(&item_str) {
                        string = RedisValue::SimpleError("ERR Set is not unique".into()).to_string();
                        break;
                    }
                    string.push_str(&item_str)
                }
                string
            },
            RedisValue::Push(push) => {
                let mut string = String::new();
                string.push_str(&format!(">{}\r\n", push.len()));
                push.into_iter().for_each(|item| string.push_str(&item.to_string()));
                string
            },
        }
    }
}