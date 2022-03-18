use mpio_iso::lectura;
use mpio_iso::process;
use shapefile::dbase::{FieldValue};
use std::time::Instant;
use std::collections::HashMap;
use std::fs;
use fs::File;
use std::io::Write;

use lectura::read_shape;
use process::cves_inter;

fn main() {

    let ruta_out = "./datos_procesados/d_300.json";

    let rootfold = "./datos/denue/";

    let start = Instant::now();

    let mut denues = Vec::new();

    let paths = fs::read_dir(rootfold).unwrap()
        .filter(|ele| match ele {
            Ok(path) => match path.file_name().into_string() {
                Ok(nombre) => nombre.contains(".shp"),
                _ =>  false
            },
            _ => false
        });
    for path in paths {
        let mut den_temp = match path {
            Ok(file) => {
                match file.path().to_str() {
                    Some(cad) => match read_shape(cad) {
                        Ok(datos) => datos,
                        _ => continue
                    },
                    _ => continue
                }
            },
            _ => continue
        };
        denues.append(&mut den_temp);
    }
    
    let isocronas = read_shape("./datos/isocronas_sucmpls_09032022.shp").unwrap();

    let mut mapa: HashMap<String,Vec<String>> = HashMap::new();

    println!("Datos cargados! {} unidades",denues.len());
    let duration = start.elapsed();
    println!("Tiempo empleado para cargar datos: {:?}", duration);

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
        let intersects = cves_inter(isoc,&denues,"clee");
        println!("cve: {:?}, intersecciones: {}",cve,intersects.len());

        mapa.insert(cve.unwrap().to_string(),intersects);
    }

    let duration = start.elapsed();
    println!("Tiempo empleado para calcular intersecciones: {:?}", duration);

    let mut salida = File::create(ruta_out).unwrap();
    let j = serde_json::to_string_pretty(&mapa).unwrap();
    write!(salida, "{}", j).unwrap();
    
}

