
use mpio_iso::isocron;
use mpls::{datos::Sucursal};
use serde::{Deserialize,Serialize};

use std::collections::BTreeMap;
use std::fs;
use fs::File;
use std::io::Write;
use isocron::Isocrona;

fn main() {

    let ruta_out = "./datos_procesados/sucursales_isoc.json";
    
    let path_sucs = "./datos/sucursales_margen.json";
    let prepath_i = "./datos_procesados/";

    let file_suc = File::open(path_sucs).unwrap();

    let sucursales: BTreeMap<String,Sucursal> = serde_json::from_reader(file_suc)
            .expect("Error al leer el archivo de sucursales");

    let mut mapa: BTreeMap<String,Contexto> = BTreeMap::new();

    for (key, value) in sucursales.iter() {
        let id = key.to_string();

        let contexto = Contexto {
            sucursal: value.clone(),
            isocronas: BTreeMap::new()
        };

        mapa.insert(id,contexto);
    }

    let distancias = ["300","600","900","1200","1500","1800"];

    for dist in distancias {
        let ruta = format!("{}f_{}.json",prepath_i,dist);
        let file = File::open(ruta).unwrap();

        let isocronas: BTreeMap<String, Isocrona> = serde_json::from_reader(file)
            .expect("Error al leer las isocronas");

        for (key, value) in isocronas.iter() {
            let id = key[..4].to_string();

            let isoc = value.clone();

            match mapa.get_mut(&id) {
                Some(contexto) => contexto.isocronas.insert(dist.to_string(),isoc),
                _ => continue
            };
        }
    }

    let mut salida = File::create(ruta_out).unwrap();
    let j = serde_json::to_string_pretty(&mapa).unwrap();
    write!(salida, "{}", j).unwrap();
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Contexto {
    sucursal: Sucursal,
    isocronas: BTreeMap<String, Isocrona>
}

