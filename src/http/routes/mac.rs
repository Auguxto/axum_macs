use axum::{routing::get, Router};

use crate::http::handlers::mac;

pub fn register() -> Router {
    let router = Router::new().route("/:mac", get(mac::get_mac));

    Router::nest(Router::new(), "/mac", router)
}
