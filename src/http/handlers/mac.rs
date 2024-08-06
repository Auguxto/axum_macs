use std::sync::Arc;

use axum::Json;
use axum::{extract::Path, http::StatusCode, Extension};

use crate::functions::data::mac::MacVendor;
use crate::http::services::mac;
use crate::AppState;

pub async fn get_mac(
    Extension(state): Extension<Arc<AppState>>,
    Path(mac_raw): Path<String>,
) -> Result<Json<MacVendor>, StatusCode> {
    let mac_formated = mac_raw.replace([':', '-', '.'], "");

    if mac_formated.len() < 6 {
        return Err(StatusCode::NOT_FOUND);
    }

    let mac_vendors = state.mac_vendors.clone();

    let macs = mac::get_mac(mac_formated, mac_vendors);

    match macs.first() {
        Some(mac) => Ok(Json(mac.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}
