use super::redis_value::RedisValue;

#[derive(Debug)]
pub enum Command {
    PING,
    SET { key: RedisValue, value: RedisValue },
    GET { key: RedisValue },
    DEL { key: RedisValue },
}

impl Command {
    pub fn from_value(value: RedisValue) -> Option<Command> {
        match value {
            RedisValue::Array(items) => {
                if items.len() > 0 {
                    match items[0].clone() {
                        RedisValue::BulkString(command) => match command.to_lowercase().as_str() {
                            "ping" => Some(Command::PING),
                            "set" => {
                                let params: Vec<_> =
                                    items[1..].iter().map(|item| item.clone()).collect();
                                let key = params.get(0)?.clone();
                                let value = params.get(1)?.clone();
                                Some(Command::SET { key, value })
                            }
                            "get" => {
                                let params: Vec<_> =
                                    items[1..].iter().map(|item| item.clone()).collect();
                                let key = params.get(0)?.clone();
                                Some(Command::GET { key })
                            }
                            "del" => {
                                let params: Vec<_> =
                                    items[1..].iter().map(|item| item.clone()).collect();
                                let key = params.get(0)?.clone();
                                Some(Command::DEL { key })
                            }
                            _ => None,
                        },
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}
