use anyhow::Result;
use gdal::{vector::LayerAccess, Dataset};
use geo::{Geometry, GeometryCollection, Polygon};
use geojson::{de::deserialize_geometry, Feature, FeatureCollection};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;

mod join_lines;
mod pavement;
mod step_along_line;

fn main() -> Result<()> {
    let pavements = if true {
        read_gj_input("test_input/small.geojson")?
    } else {
        read_gpkg_input("test_input/large.gpkg")?
    };

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

    dump_gj("input_polygons.geojson", input_polygons)?;
    dump_gj("skeletons.geojson", skeletons)?;
    dump_gj("perps.geojson", perps)?;

    std::fs::write("thickened.geojson", serde_json::to_string(&FeatureCollection {
        features: thickened,
        bbox: None,
        foreign_members: Some(serde_json::json!({
            "crs": { "type": "name", "properties": { "name": "urn:ogc:def:crs:EPSG::27700" } }
        })
        .as_object()
        .unwrap()
        .clone()),
    })?)?;
    println!("Wrote thickened.geojson");

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
    let input: Vec<Input> = geojson::de::deserialize_feature_collection_str_to_vec(
        &std::fs::read_to_string(filename)?,
    )?;
    Ok(input
        .into_iter()
        .map(|x| pavement::Pavement::new(x.geometry))
        .collect())
}

#[derive(Deserialize)]
struct Input {
    #[serde(deserialize_with = "deserialize_geometry")]
    geometry: Polygon,
}

fn read_gpkg_input(filename: &str) -> Result<Vec<pavement::Pavement>> {
    let mut results = Vec::new();
    let dataset = Dataset::open(filename)?;
    // Assume only one layer
    let mut layer = dataset.layer(0)?;
    for feature in layer.features() {
        if feature
            .field_as_string_by_name("descriptive_group")?
            .unwrap()
            != "Roadside"
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
