mod redis_value;
mod command;

use log::info;

pub async fn execute() {
    info!("Hello from the redis thread!");
    
}