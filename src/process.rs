use geo::{intersects::Intersects, prelude::BoundingRect, Rect, GeoNum};
use crate::lectura::GeoElemento;
use shapefile::dbase::{Record,FieldValue};
use rayon::prelude::*;


fn peek_inside<T,D>(bbox: Rect<T>, geom: &GeoElemento<T,D>) -> bool 
where T: GeoNum
{
    let centro = geom.centroide;

    let x = centro.x();
    let y = centro.y();

    let xmin = bbox.min().x;
    let ymin = bbox.min().y;
    let xmax = bbox.max().x;
    let ymax = bbox.max().y;

    x >= xmin && x <= xmax && y >= ymin && y <= ymax
}

pub fn cves_inter(isoge: &GeoElemento<f64,Record>, subsge: &Vec<GeoElemento<f64,Record>>) -> Vec<String> 
{
    
    let isoc = &isoge.geometria;
    let bbox = isoc.bounding_rect().unwrap();

    let intersects: Vec<String> = subsge.par_iter()
            .filter(|mza| peek_inside(bbox, mza))
            .filter(|mnza| isoc.intersects(&mnza.geometria))
            .map(|mnz| {
                match mnz.datos.get("CVEGEO") {
                    Some(fcve) => match fcve {
                        FieldValue::Character(ocve) => match ocve {
                            Some(cve) => Some(cve.to_string()),
                            None => None
                        },
                        _ => None
                    },
                    _ => None
                }
            })
            .filter_map(|cve| cve)
            .collect();

    intersects
}