use std::collections::HashMap;

use crate::functions::data::mac::MacVendor;

pub fn get_mac(mac: String, mac_vendors: HashMap<String, MacVendor>) -> Vec<MacVendor> {
    let prefixes: Vec<&str> = match mac.len() {
        0..=6 => vec![&mac[0..6]],
        7..=8 => vec![&mac[0..6], &mac[0..7]],
        9.. => vec![&mac[0..6], &mac[0..7], &mac[0..9]],
    };

    let mut macs: Vec<MacVendor> = Vec::new();

    for prefix in prefixes {
        if let Some(mac) = mac_vendors.get(prefix) {
            macs.push(mac.clone());
        }
    }

    macs
}
