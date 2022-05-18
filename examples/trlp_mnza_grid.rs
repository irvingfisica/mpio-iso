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

    let start = Instant::now();
    let ruta_out = "./datos_procesados/g500.json";

    let manzanas = read_shape("./datos/mnzs_geod.shp").unwrap();
    let isocronas = read_shape("./datos/mexico_grid500x500WGS84.shp").unwrap();

    let mut mapa: HashMap<String,Vec<String>> = HashMap::new();

    println!("Datos cargados!");
    let duration = start.elapsed();
    println!("Tiempo empleado para cargar datos: {:?}", duration);

    let nmax = isocronas.len();
    println!("isocs: {:?}",nmax);

    let start = Instant::now();

    for (cta,isoc) in isocronas.iter().enumerate()
    {
        let cve: Option<String> = match isoc.datos.get("id") {
            Some(FieldValue::Numeric(Some(cvei))) => Some((cvei.floor() as i32).to_string()),
            _ => None
        };
        let intersects = cves_inter(isoc,&manzanas,"CVEGEO");
        println!("cve: {:?}, intersecciones: {}, %:{}",cve,intersects.len(),(cta as f64)/(nmax as f64)*100.0);

        mapa.insert(cve.unwrap().to_string(),intersects);
    }

    let duration = start.elapsed();
    println!("Tiempo empleado para calcular intersecciones: {:?}", duration);

    let mut salida = File::create(ruta_out).unwrap();
    let j = serde_json::to_string_pretty(&mapa).unwrap();
    write!(salida, "{}", j).unwrap();
    
}

