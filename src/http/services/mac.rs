use crate::functions::data::mac::MacVendor;

pub fn get_mac(mac: String, mac_vendors: Vec<MacVendor>) -> Vec<MacVendor> {
    let prefixes: Vec<&str> = match mac.len() {
        0..=6 => vec![&mac[0..6]],
        7..=8 => vec![&mac[0..6], &mac[0..7]],
        9.. => vec![&mac[0..6], &mac[0..7], &mac[0..9]],
    };

    mac_vendors
        .into_iter()
        .filter(|vendor| {
            let vendor_prefix = &vendor.prefix.replace(':', "");

            prefixes
                .iter()
                .any(|&prefix| vendor_prefix.starts_with(prefix))
        })
        .collect::<Vec<MacVendor>>()
}
