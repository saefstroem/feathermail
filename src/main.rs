use std::{env, io};
mod db;

#[actix_web]
async fn main() -> io::Result<()> {
    env_logger::init();

    let SSL_FULLCHAIN=env::var("SSL_FULLCHAIN").unwrap_or_default();
    let SSL_PRIVKEY=env::var("SSL_PRIVKEY").unwrap_or_default();
    let BIND_ADDRESS=env::var("BIND_ADDRESS").unwrap_or("localhost");
    let WEBHOOK_URL=env::var("WEBHOOK_URL").unwrap_or_default();


}