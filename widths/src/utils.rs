use anyhow::{bail, Result};
use geo::{Geometry, GeometryCollection, Polygon};
use geojson::GeoJson;

use crate::{Mercator, Pavement};

pub fn read_gj_input(input: String) -> Result<(Vec<Pavement>, Mercator)> {
    let gj: GeoJson = input.parse()?;
    let mut wgs84_polygons = Vec::new();
    for x in geojson::quick_collection(&gj)? {
        match x {
            Geometry::Polygon(p) => {
                wgs84_polygons.push(p);
            }
            Geometry::MultiPolygon(mp) => {
                for p in mp {
                    wgs84_polygons.push(p);
                }
            }
            _ => bail!("Unexpected geometry type {:?}", x),
        }
    }

    Ok(to_mercator(wgs84_polygons))
}

pub fn to_mercator(polygons: Vec<Polygon>) -> (Vec<Pavement>, Mercator) {
    // TODO Expensive clone
    let collection = GeometryCollection::from(polygons.clone());
    let mercator = Mercator::from(collection).unwrap();

    let mut results = Vec::new();
    for mut p in polygons {
        mercator.to_mercator_in_place(&mut p);
        results.push(Pavement::new(p));
    }

    (results, mercator)
}
