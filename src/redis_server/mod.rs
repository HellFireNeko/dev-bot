mod redis_value;
mod command;
mod db;

use log::info;

use std::{fs, sync::Arc};
use tokio::sync::Mutex;

use self::redis_value::RedisValue;

pub async fn execute() {
    info!("Hello from the redis thread!");
    


    #[cfg(feature = "testrunner")]
    {
        test::test().await;
    }
}

#[cfg(feature = "testrunner")]
mod test {
    // A simple function designed to help me test the RESP values and other features
    async fn test() {
        let mut values = Vec::new();
        
        values.push(RedisValue::Boolean(true));
        values.push(RedisValue::BulkString("Hello, world".to_string()));
        values.push(RedisValue::Integer(42069));

        let value = RedisValue::Array(values);

        let resp = value.to_resp();

        info!("Resp for redis value: {resp}");

        let value_from_resp = RedisValue::from_resp(resp);

        dbg!(value_from_resp);
    }
}