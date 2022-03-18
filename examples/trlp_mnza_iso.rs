use mpio_iso::lectura;
use mpio_iso::process;
use shapefile::dbase::FieldValue;
use std::time::Instant;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

use lectura::read_shape;
use process::cves_inter;

fn main() {

    let ruta_out = "./datos_procesados/300.json";

    let manzanas = read_shape("./datos/mnzs_geod.shp").unwrap();
    let isocronas = read_shape("./datos/isocronas_sucmpls_09032022.shp").unwrap();

    let mut mapa: HashMap<String,Vec<String>> = HashMap::new();

    println!("Datos cargados!");

    let start = Instant::now();

    for isoc in isocronas.iter()
    .filter(|iso|{
        match iso.datos.get("range") {
            Some(FieldValue::Numeric(num)) => match num {
                Some(inum) => *inum == 300.0,
                _ => false
            }
            _ => false
        }
    })
    {
        let cve = match isoc.datos.get("cveiso") {
            Some(FieldValue::Character(Some(cvei))) => Some(cvei.as_str()),
            _ => None
        };
        let intersects = cves_inter(isoc,&manzanas,"CVEGEO");
        println!("cve: {:?}, intersecciones: {}",cve,intersects.len());

        mapa.insert(cve.unwrap().to_string(),intersects);
    }

    let duration = start.elapsed();
    println!("Tiempo empleado para calcular intersecciones: {:?}", duration);

    let mut salida = File::create(ruta_out).unwrap();
    let j = serde_json::to_string_pretty(&mapa).unwrap();
    write!(salida, "{}", j).unwrap();
    
}

