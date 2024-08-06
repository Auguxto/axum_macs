use axum::Router;

pub mod mac;

pub fn register() -> Router {
    Router::new().merge(mac::register())
}
