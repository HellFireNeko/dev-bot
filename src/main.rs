use tokio::signal;

mod executor;

#[tokio::main]
async fn main() {
    let mut sigterm = signal::unix::signal(signal::unix::SignalKind::terminate()).unwrap();
    let mut sigint = signal::unix::signal(signal::unix::SignalKind::interrupt()).unwrap();
    // Initialize a logger of info level
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Info)
        .filter_module("serenity", log::LevelFilter::Error)
        .init();

    // Executor client daemon
    tokio::spawn(executor::redis::execute());
    tokio::spawn(executor::bot::execute());
    tokio::spawn(executor::web::execute());

    tokio::select! {
        _ = sigterm.recv() => {
            // Handle SIGTERM (e.g., perform cleanup)
            println!("Received SIGTERM, shutting down gracefully...");
            // Perform cleanup actions here...
        }
        _ = sigint.recv() => {
            // Handle SIGINT (e.g., perform cleanup)
            println!("Received SIGINT, shutting down gracefully...");
            // Perform cleanup actions here...
        }
        // Other tasks or futures can continue here...
    }
}
