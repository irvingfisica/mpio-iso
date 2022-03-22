use std::collections::BTreeMap;
use serde::{Deserialize,Serialize};
// use std::error::Error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Isocrona {
    pub id: String,
    pub economico: BTreeMap<String,u32>,
    pub demografico: BTreeMap<String,f64>
}

impl Isocrona {
    pub fn new(id: &str) -> Self {
        Isocrona {
            id: id.to_string(),
            economico: BTreeMap::new(),
            demografico: BTreeMap::new(),
        }
    }
}