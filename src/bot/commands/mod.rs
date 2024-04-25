pub mod shutdown;


pub async fn register(ctx: &serenity::all::Context) {
    let _ = serenity::all::Command::create_global_command(&ctx.http, shutdown::register()).await;
}