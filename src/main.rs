use std::sync::Arc;

use axum::{Extension, Router};
use functions::data::mac::MacVendor;
use tokio::net::TcpListener;

mod functions;
mod http;

pub struct AppState {
    pub mac_vendors: Vec<MacVendor>,
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app_state = Arc::new(AppState {
        mac_vendors: functions::data::mac::read_mac_database(),
    });

    let app = Router::new()
        .merge(http::routes::register())
        .layer(Extension(app_state));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
