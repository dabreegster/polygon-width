use anyhow::{bail, Result};
use gdal::{vector::LayerAccess, Dataset};
use geo::{Geometry, GeometryCollection};
use geojson::{Feature, FeatureCollection, GeoJson};
use indicatif::{ProgressBar, ProgressStyle};

mod join_lines;
mod pavement;
mod step_along_line;

fn main() -> Result<()> {
    //let pavements = read_gj_input("test_input/small_pavements.geojson")?;
    let pavements = read_gj_input("test_input/small_road_polygons.geojson")?;
    //let pavements = read_gj_input("test_input/dissolved_roads.geojson")?;
    //let pavements = read_gpkg_input("test_input/large.gpkg", "Roadside")?;
    //let pavements = read_gpkg_input("test_input/large.gpkg", "Road Or Track")?;

    let mut input_polygons = Vec::new();
    let mut skeletons = Vec::new();
    let mut perps = Vec::new();
    let mut thickened = Vec::new();

    let progress = ProgressBar::new(pavements.len() as u64).with_style(ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());

    for mut pavement in pavements {
        progress.inc(1);
        pavement.calculate();

        input_polygons.push(pavement.polygon);
        skeletons.extend(pavement.skeletons);
        perps.extend(pavement.perp_lines);
        for (polygon, width) in pavement.thickened_lines {
            let mut f = Feature::from(geojson::Geometry::from(&polygon));
            f.set_property("width", width);
            thickened.push(f);
        }
    }

    dump_gj("output/input_polygons.geojson", input_polygons)?;
    dump_gj("output/skeletons.geojson", skeletons)?;
    dump_gj("output/perps.geojson", perps)?;

    std::fs::write("output/thickened.geojson", serde_json::to_string(&FeatureCollection {
        features: thickened,
        bbox: None,
        foreign_members: Some(serde_json::json!({
            "crs": { "type": "name", "properties": { "name": "urn:ogc:def:crs:EPSG::27700" } }
        })
        .as_object()
        .unwrap()
        .clone()),
    })?)?;
    println!("Wrote output/thickened.geojson");

    Ok(())
}

fn dump_gj<IG: Into<Geometry>>(filename: &str, geometry: Vec<IG>) -> Result<()> {
    let mut fc = FeatureCollection::from(&GeometryCollection::from_iter(geometry));
    fc.foreign_members = Some(
        serde_json::json!({
            "crs": { "type": "name", "properties": { "name": "urn:ogc:def:crs:EPSG::27700" } }
        })
        .as_object()
        .unwrap()
        .clone(),
    );
    std::fs::write(filename, serde_json::to_string(&fc)?)?;
    println!("Wrote {filename}");
    Ok(())
}

fn read_gj_input(filename: &str) -> Result<Vec<pavement::Pavement>> {
    let gj: GeoJson = std::fs::read_to_string(filename)?.parse()?;
    let mut results = Vec::new();
    for x in geojson::quick_collection(&gj)? {
        match x {
            Geometry::Polygon(p) => {
                results.push(pavement::Pavement::new(p));
            }
            Geometry::MultiPolygon(mp) => {
                for p in mp {
                    results.push(pavement::Pavement::new(p));
                }
            }
            _ => bail!("Unexpected geometry type {:?}", x),
        }
    }
    Ok(results)
}

fn read_gpkg_input(filename: &str, descriptive_group: &str) -> Result<Vec<pavement::Pavement>> {
    let mut results = Vec::new();
    let dataset = Dataset::open(filename)?;
    // Assume only one layer
    let mut layer = dataset.layer(0)?;
    for feature in layer.features() {
        if feature
            .field_as_string_by_name("descriptive_group")?
            .unwrap()
            != descriptive_group
        {
            continue;
        }
        if feature.field_as_string_by_name("make")?.unwrap() != "Manmade" {
            continue;
        }

        let Geometry::Polygon(polygon) = feature.geometry().unwrap().to_geo()? else {
            continue;
        };
        results.push(pavement::Pavement::new(polygon));
    }
    Ok(results)
}
