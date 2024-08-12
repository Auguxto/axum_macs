static MAC_DATABASE: &str = include_str!("../../data/mac.csv");

use std::collections::HashMap;

use csv::Reader;
use serde::{Deserialize, Serialize};

// Mac Prefix,Vendor Name,Private,Block Type,Last Update
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MacVendor {
    #[serde(rename(serialize = "prefix", deserialize = "Mac Prefix"))]
    pub prefix: String,
    #[serde(rename(serialize = "vendor", deserialize = "Vendor Name"))]
    pub vendor: String,
    #[serde(rename(serialize = "private", deserialize = "Private"))]
    pub private: bool,
    #[serde(rename(serialize = "block_type", deserialize = "Block Type"))]
    pub block_type: String,
    #[serde(rename(serialize = "last_update", deserialize = "Last Update"))]
    pub last_update: String,
}

pub fn read_mac_database() -> HashMap<String, MacVendor> {
    let mut vendors: HashMap<String, MacVendor> = HashMap::new();
    let mut reader = Reader::from_reader(MAC_DATABASE.as_bytes());

    for record in reader.deserialize::<MacVendor>() {
        match record {
            Ok(record) => {
                vendors.insert(record.prefix.clone().replace(':', ""), record);
            }
            Err(e) => eprintln!("Error deserializing record: {}", e),
        }
    }

    vendors
}
