use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use log::info;
use tokio::signal;

mod bot;
#[allow(dead_code)]
mod redis_server;
mod string_manip;
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
        .filter_module("tracing", log::LevelFilter::Error)
        .init();

    // Executor client daemon
    let redis_thread = tokio::spawn(redis_server::execute());
    tokio::spawn(bot::execute());
    tokio::spawn(web_server::execute());

    'sigloop: loop {
        // Check if the Shutdown flag got set either from bot or from web
        if is_shutdown_flag_set() {
            info!("Shutdown requested by external source, shutting down gracefully...");
            break 'sigloop; // Exit sigloop
        }

        tokio::select! {
            _ = sigterm.recv() => {
                // Handle SIGTERM (e.g., perform cleanup)
                info!("Received SIGTERM, shutting down gracefully...");
                set_shutdown_flag();
                break 'sigloop; // Exit sigloop
            }
            _ = sigint.recv() => {
                // Handle SIGINT (e.g., perform cleanup)
                info!("Received SIGINT, shutting down gracefully...");
                set_shutdown_flag();
                break 'sigloop; // Exit sigloop
            }
            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                continue 'sigloop; // Continue sigloop
            }
        }
    }

    let _ = redis_thread.await;
}
