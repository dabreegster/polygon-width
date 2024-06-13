use anyhow::Result;
use gdal::{vector::LayerAccess, Dataset};
use geo::{Geometry, GeometryCollection};
use geojson::{Feature, FeatureCollection, GeoJson};
use indicatif::{ProgressBar, ProgressStyle};

use widths::{
    utils::{read_gj_input, to_mercator},
    Config, Mercator, Pavement,
};

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with a .geojson or .gpkg file in WGS84");
        std::process::exit(1);
    }

    let cfg = Config::default();
    let (pavements, mercator) = if args[1].ends_with(".geojson") {
        read_gj_input(std::fs::read_to_string(&args[1])?, &cfg)?
    } else if args[1].ends_with(".gpkg") {
        // TODO Take a flag to decide which one, or do the filtering elsewhere?
        read_gpkg_input("../test_input/large.gpkg", "Roadside", &cfg)?
        //read_gpkg_input("../test_input/large.gpkg", "Road Or Track", &cfg)?
    } else {
        println!("Call with a .geojson or .gpkg file in WGS84");
        std::process::exit(1);
    };

    let mut input_polygons = Vec::new();
    let mut skeletons = Vec::new();
    let mut perps = Vec::new();
    let mut thickened = Vec::new();

    let progress = ProgressBar::new(pavements.len() as u64).with_style(ProgressStyle::with_template(
        "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {human_pos}/{human_len} ({per_sec}, {eta})").unwrap());

    for mut pavement in pavements {
        progress.inc(1);
        pavement.calculate(&cfg);

        input_polygons.push(pavement.polygon);
        skeletons.extend(pavement.skeletons);
        perps.extend(pavement.perp_lines);
        for (polygon, width1, width2) in pavement.thickened_lines {
            let mut f = Feature::from(geojson::Geometry::from(&mercator.to_wgs84(&polygon)));
            f.set_property("width1", width1);
            f.set_property("width2", width2);
            thickened.push(f);
        }
    }

    dump_gj("output/input_polygons.geojson", &mercator, input_polygons)?;
    dump_gj("output/skeletons.geojson", &mercator, skeletons)?;
    dump_gj("output/perps.geojson", &mercator, perps)?;

    std::fs::write(
        "output/thickened.geojson",
        serde_json::to_string(&GeoJson::from(thickened))?,
    )?;
    println!("Wrote output/thickened.geojson");

    Ok(())
}

fn dump_gj<IG: Into<Geometry>>(
    filename: &str,
    mercator: &Mercator,
    geometry: Vec<IG>,
) -> Result<()> {
    let fc = FeatureCollection::from(&mercator.to_wgs84(&GeometryCollection::from_iter(geometry)));
    std::fs::write(filename, serde_json::to_string(&fc)?)?;
    println!("Wrote {filename}");
    Ok(())
}

#[allow(unused)]
fn read_gpkg_input(
    filename: &str,
    descriptive_group: &str,
    cfg: &Config,
) -> Result<(Vec<Pavement>, Mercator)> {
    let mut polygons = Vec::new();
    let dataset = Dataset::open(filename)?;
    // Assume only one layer
    let mut layer = dataset.layer(0)?;
    for mut feature in layer.features() {
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
        polygons.push(polygon);
    }

    Ok(to_mercator(polygons, cfg))
}
