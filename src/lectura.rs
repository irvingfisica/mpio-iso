use geo::Geometry;
use geo::GeoNum;
use geo::Point;
use geo::prelude::Centroid;
use std::error::Error;
use shapefile::dbase::Record;
use std::path;
use std::fs;
use geojson::GeoJson;
use serde_json::{map::Map,value::Value};

#[derive(Debug)]
pub struct GeoElemento<T: GeoNum,D> {
    pub geometria: Geometry<T>,
    pub datos: D,
    pub centroide: Point<T>
}

pub fn read_shape(ruta: &str) -> Result<Vec<GeoElemento<f64,Record>>, Box<dyn Error>>  
{
    let mut reader= shapefile::Reader::from_path(ruta)?;

    let mut salida: Vec<GeoElemento<f64,Record>> = Vec::new();

    for shape_record in reader.iter_shapes_and_records() {
        let (shape, record) = shape_record?;
        let geometry = match geo_types::Geometry::<f64>::try_from(shape) {
            Ok(geom) => geom,
            _ => continue
        };
        let centro = match geometry.centroid() {
            Some(centro) => centro,
            None => continue
        };
        let elem = GeoElemento {
            geometria: geometry,
            datos: record,
            centroide: centro,
        };

        salida.push(elem);
    }
    
    Ok(salida)
}

pub fn read_gjson(ruta: &str) -> Result<Vec<GeoElemento<f64,Map<String,Value>>>, Box<dyn Error>> {
    let json_file_path = path::Path::new(ruta);
    let file = fs::read_to_string(json_file_path).expect("No se pudo leer el archivo");

    let geojson = file.parse::<GeoJson>()?;
    let features = process_geojson(geojson)?;

    let mut salida = Vec::new();

    for feature in features.into_iter() {


        let props = match feature.properties.as_ref() {
            Some(propmap) => propmap.clone(),
            _ => continue,
        };

        let geometry: Geometry<f64> = match feature.try_into() {
            Ok(geom) => geom,
            _ => continue
        };
        let centro = match geometry.centroid() {
            Some(centro) => centro,
            None => continue
        };
        let elem = GeoElemento {
            geometria: geometry,
            datos: props,
            centroide: centro,
        };

        salida.push(elem);
    }

    Ok(salida)
}

fn process_geojson(gj: GeoJson) -> Result<Vec<geojson::Feature>,Box<dyn Error>> {
    match gj {
        GeoJson::FeatureCollection(ctn) => {
            Ok(ctn.features)
        },
        GeoJson::Feature(feat) => {
            Ok(vec![feat])
        },
        GeoJson::Geometry(_) => {
            Err(From::from("El geojson contiene geometría, no features."))
        },
    }
}