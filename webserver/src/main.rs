use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
 
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "webserver=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
 
    info!("hello, web server!");
}
