use std::collections::BTreeMap;
use serde::{Deserialize,Serialize};
use mpls::datos::SucursalIn;
// use std::error::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Isocrona {
    pub id: String,
    pub economico: BTreeMap<String,u32>,
    pub demografico: BTreeMap<String,f64>,
    pub mpls_cercanos: Option<i32>,
}

impl Isocrona {
    pub fn new(id: &str) -> Self {
        Isocrona {
            id: id.to_string(),
            economico: BTreeMap::new(),
            demografico: BTreeMap::new(),
            mpls_cercanos: None,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Alien {
    #[serde(alias = "Suc MPLS Externas")]
    pub mpls_cercanos: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contexto {
    pub sucursal: SucursalIn,
    pub isocronas: BTreeMap<String, Isocrona>,
}