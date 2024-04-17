use std::sync::atomic::{AtomicBool, Ordering};

use tokio::signal;

mod redis_server;
mod bot;
mod web_server;

lazy_static::lazy_static! {
    static ref SHUTDOWN_FLAG: AtomicBool = AtomicBool::new(false);
}

pub fn set_shutdown_flag() {
    SHUTDOWN_FLAG.store(true, Ordering::SeqCst);
}

pub fn is_shutdown_flag_set() -> bool {
    SHUTDOWN_FLAG.load(Ordering::SeqCst)
}

#[tokio::main]
async fn main() {
    // Register handlers for signals of Terminate and Interrupt
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();

    // Initialize a logger of info level
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .filter_module("serenity", log::LevelFilter::Error)
        .init();

    // Executor client daemon
    tokio::spawn(redis_server::execute());
    tokio::spawn(bot::execute());
    tokio::spawn(web_server::execute());

    tokio::select! {
        _ = sigterm.recv() => {
            // Handle SIGTERM (e.g., perform cleanup)
            println!("Received SIGTERM, shutting down gracefully...");
            set_shutdown_flag();
        }
        _ = sigint.recv() => {
            // Handle SIGINT (e.g., perform cleanup)
            println!("Received SIGINT, shutting down gracefully...");
            set_shutdown_flag();
        }
    }
}
