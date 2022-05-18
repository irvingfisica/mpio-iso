
use mpio_iso::isocron;

use std::collections::BTreeMap;
use std::fs;
use fs::File;
use std::io::Write;
use isocron::Isocrona;

fn main() {

    let path_den = "./datos_procesados/d_g500.json";
    let path_cen = "./datos_procesados/grid500x500_censo.json";

    let ruta_out = "./datos_procesados/f_g500.json";

    let file_den = File::open(path_den).unwrap();
    let file_cen = File::open(path_cen).unwrap();

    let isocden: BTreeMap<String,Vec<String>> = serde_json::from_reader(file_den)
            .expect("Error al leer el archivo de unidades económicas");
    let isoccen: BTreeMap<String,BTreeMap<String,Option<f64>>> = serde_json::from_reader(file_cen)
            .expect("Error al leer el archivo de datos censales");

    let mut iscoronas = BTreeMap::new();

    for (key,values) in isocden.iter() {
        let mut isoc = Isocrona::new(key);

        for unidad in values.iter() {
            let actividad = unidad[5..11].to_string();
            *isoc.economico.entry(actividad).or_insert(0) += 1;
        };

        iscoronas.insert(key.to_string(),isoc);
    };

    for (key, values) in isoccen.iter() {
        match iscoronas.get_mut(key) {
            Some(isoc) => {
                isoc.demografico = values.clone();
            },
            _ => continue
        }
    };

    let mut salida = File::create(ruta_out).unwrap();
    let j = serde_json::to_string_pretty(&iscoronas).unwrap();
    write!(salida, "{}", j).unwrap();
    
}

