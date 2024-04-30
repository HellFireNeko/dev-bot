use std::{fs, sync::Arc};

use tokio::sync::Mutex;

use super::redis_value::RedisValue;

pub struct Database {
    database: Arc<Mutex<RedisValue>>
}

impl Database {
    pub fn init() -> Self {
        Database {
            database: Arc::new(Mutex::new(RedisValue::from_resp(get_db())))
        }
    }

    async fn get_db(&mut self) -> RedisValue {
        self.database.lock().await.clone()
    }

    async fn set_db(&mut self, db: RedisValue) {

    }

    pub async fn get(&mut self, key: RedisValue) -> RedisValue {
        let db = self.get_db().await;
        match db {
            RedisValue::Map(map) => {
                if map.contains_key(&key) {
                    map[&key].clone()
                } else {
                    RedisValue::SimpleError("Internal databse error!".to_string())
                }
            }
            _ => RedisValue::SimpleError("Internal databse error!".to_string())
        }
    }

    pub async fn set(&mut self, key: RedisValue, value: RedisValue) {

    }
}

fn get_db() -> String {
    if let Ok(meta) = fs::metadata("database.resp") {
        if meta.is_file() {
            if let Ok(contents) = fs::read_to_string("database.resp") {
                contents
            } else {
                String::from("&0\r\n")
            }
        } else {
            String::from("&0\r\n")
        }
    } else {
        String::from("&0\r\n")
    }
}