use chrono::Duration;

use super::redis_value::RedisValue;

#[derive(Debug)]
pub enum Command {
    PING,
    SET {
        key: RedisValue,
        value: RedisValue,
        timeout: Option<Duration>,
    },
    GET {
        key: RedisValue,
    },
    DEL {
        key: RedisValue,
    },
}

impl Command {
    pub fn from_value(value: RedisValue) -> Option<Command> {
        match value {
            RedisValue::Array(items) => {
                if items.len() > 0 {
                    match items[0].clone() {
                        RedisValue::BulkString(command) => {
                            match command.to_lowercase().as_str() {
                                "ping" => Some(Command::PING),
                                "set" => {
                                    let params: Vec<_> = items[1..].iter().collect();
                                    None
                                }
                                _ => None
                            }
                        }
                        _ => None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
