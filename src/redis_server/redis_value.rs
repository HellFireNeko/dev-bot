use std::collections::HashMap;

use crate::string_manip::{consume_crlf, consume_n_chars, consume_until_crlf};

#[derive(Debug, Clone)]
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
    pub fn to_resp(&self) -> String {
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
                vec.into_iter().for_each(|item| string.push_str(&item.to_resp()));
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
                    string.push_str(&key.to_resp());
                    string.push_str(&value.to_resp());
                });
                string
            },
            RedisValue::Set(set) => {
                let mut string = String::new();
                string.push_str(&format!("~{}\r\n", set.len()));
                for item in set {
                    let item_str = item.to_resp();
                    if string.contains(&item_str) {
                        string = RedisValue::SimpleError("ERR Set is not unique".into()).to_resp();
                        break;
                    }
                    string.push_str(&item_str)
                }
                string
            },
            RedisValue::Push(push) => {
                let mut string = String::new();
                string.push_str(&format!(">{}\r\n", push.len()));
                push.into_iter().for_each(|item| string.push_str(&item.to_resp()));
                string
            },
        }
    }

    pub fn from_resp(resp: String) -> RedisValue {
        let mut iter = resp.chars().peekable();

        RedisValue::from_iter(&mut iter)
    }

    fn from_iter(iter: &mut std::iter::Peekable<std::str::Chars>) -> RedisValue {
        match iter.peek() {
            Some('+') => {
                // Simple String
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(value) => RedisValue::SimpleString(value),
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('-') => {
                // Simple Error
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(value) => RedisValue::SimpleError(value),
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some(':') => {
                // Integer
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(value) => {
                        match value.parse() {
                            Ok(value) => RedisValue::Integer(value),
                            Err(_) => RedisValue::SimpleError("Could not parse as Integer".to_string())
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('$') => {
                // Bulk String
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(length) => {
                        if length == "-1" {
                            RedisValue::NullBulkString
                        } else {
                            match length.parse() {
                                Ok(length) => {
                                    match consume_n_chars(iter, length) {
                                        Ok(value) => {
                                            match consume_crlf(iter) {
                                                Ok(()) => RedisValue::BulkString(value),
                                                Err(err) => RedisValue::SimpleError(err)
                                            }
                                        }
                                        Err(err) => RedisValue::SimpleError(err)
                                    }
                                },
                                Err(_) => RedisValue::SimpleError("Could not parse as Integer".to_string())
                            }
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('*') => {
                // Array
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(length) => {
                        if length == "-1" {
                            RedisValue::NullArray
                        } else {
                            match length.parse() {
                                Ok(length) => {
                                    let mut array = Vec::new();
                                    for _ in 0..length {
                                        // recursively get from iter
                                        array.push(RedisValue::from_iter(iter));
                                    }
                                    RedisValue::Array(array)
                                },
                                Err(_) => RedisValue::SimpleError("Could not parse as Integer".to_string())
                            }
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('_') => {
                // Null
                iter.next();
                match consume_crlf(iter) {
                    Ok(()) => {
                        RedisValue::Null
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('#') => {
                match consume_until_crlf(iter) {
                    Ok(value) => {
                        match value.as_str() {
                            "#t" => {
                                RedisValue::Boolean(true)
                            }
                            "#f" => {
                                RedisValue::Boolean(false)
                            }
                            _ => RedisValue::SimpleError("No valid boolean found".to_string())
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('(') => {
                // Big Integer
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(value) => {
                        match value.parse() {
                            Ok(value) => RedisValue::BigInteger(value),
                            Err(_) => RedisValue::SimpleError("Could not parse as Integer".to_string())
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('!') => {
                // Bulk Error
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(length) => {
                        if length == "-1" {
                            RedisValue::NullBulkString
                        } else {
                            match length.parse() {
                                Ok(length) => {
                                    match consume_n_chars(iter, length) {
                                        Ok(value) => {
                                            match consume_crlf(iter) {
                                                Ok(()) => RedisValue::BulkString(value),
                                                Err(err) => RedisValue::SimpleError(err)
                                            }
                                        }
                                        Err(err) => RedisValue::SimpleError(err)
                                    }
                                },
                                Err(_) => RedisValue::SimpleError("Could not parse as Integer".to_string())
                            }
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            Some('=') => {
                // Verbatim String
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(length) => {
                        if length == "-1" {
                            RedisValue::NullBulkString
                        } else {
                            match length.parse() {
                                Ok(length) => {
                                    match consume_n_chars(iter, length) {
                                        Ok(value) => {
                                            match consume_crlf(iter) {
                                                Ok(()) => {
                                                    let mut parts = value.split(':').into_iter();
                                                    if let (Some(encoding), Some(contents)) = (parts.next(), parts.next()) {
                                                        RedisValue::VerbatimString(encoding.to_string(), contents.to_string())
                                                    } else {
                                                        RedisValue::SimpleError("Invalid verbatim string".to_string())
                                                    }
                                                },
                                                Err(err) => RedisValue::SimpleError(err)
                                            }
                                        }
                                        Err(err) => RedisValue::SimpleError(err)
                                    }
                                },
                                Err(_) => RedisValue::SimpleError("Could not parse as Integer".to_string())
                            }
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err)
                }
            }
            
            Some('~') => {
                // Set
                iter.next(); 
                match consume_until_crlf(iter) {
                    Ok(length) => {
                        if length == "-1" {
                            RedisValue::NullArray
                        } else {
                            match length.parse::<usize>() {
                                Ok(length) => {
                                    let mut set = Vec::new();
                                    for _ in 0..length {
                                        set.push(RedisValue::from_iter(iter));
                                    }
                                    let mut value = Vec::new();

                                    for item in &set {
                                        let item_str = item.to_resp();
                                        if value.contains(&item_str) {
                                            return RedisValue::SimpleError("Set is not unique".to_string())
                                        }
                                        value.push(item_str)
                                    }

                                    RedisValue::Set(set)
                                }
                                Err(_) => RedisValue::SimpleError("Invalid length for set".to_string()),
                            }
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err),
                }
            }
            Some('>') => {
                // Push
                iter.next();
                match consume_until_crlf(iter) {
                    Ok(length) => {
                        if length == "-1" {
                            RedisValue::NullArray
                        } else {
                            match length.parse::<usize>() {
                                Ok(length) => {
                                    let mut array = Vec::new();
                                    for _ in 0..length {
                                        // recursively get from iter
                                        array.push(RedisValue::from_iter(iter));
                                    }
                                    RedisValue::Push(array)
                                }
                                Err(_) => RedisValue::SimpleError("Invalid length for push".to_string()),
                            }
                        }
                    }
                    Err(err) => RedisValue::SimpleError(err),
                }
            }
            _ => RedisValue::SimpleError("Invalid or empty character for value initializer".to_string())
        }
    }
}