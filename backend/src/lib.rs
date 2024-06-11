use std::sync::Once;

use anyhow::Result;
use geo::GeometryCollection;
use geojson::{Feature, FeatureCollection, GeoJson};
use wkt::ToWkt;

use wasm_bindgen::prelude::*;

static START: Once = Once::new();

/// Takes GeoJSON with polygons in WGS84
#[wasm_bindgen(js_name = findWidths)]
pub fn find_widths(input: String, raw_cfg: JsValue) -> Result<String, JsValue> {
    // Panics shouldn't happen, but if they do, console.log them.
    console_error_panic_hook::set_once();
    START.call_once(|| {
        console_log::init_with_level(log::Level::Info).unwrap();
    });

    let cfg: widths::Config = serde_wasm_bindgen::from_value(raw_cfg)?;

    let mut input_polygons = Vec::new();
    let mut skeletons = Vec::new();
    let mut perps = Vec::new();
    let mut thickened = Vec::new();

    let (pavements, mercator) = widths::utils::read_gj_input(input, &cfg).map_err(err_to_js)?;

    let wkt_input = if pavements.len() == 1 {
        mercator.to_wgs84(&pavements[0].polygon).wkt_string()
    } else {
        String::new()
    };

    for mut pavement in pavements {
        pavement.calculate(&cfg);

        input_polygons.push(pavement.polygon);
        skeletons.extend(pavement.skeletons);
        perps.extend(pavement.perp_lines);
        for (polygon, width) in pavement.thickened_lines {
            let mut f = Feature::from(geojson::Geometry::from(&mercator.to_wgs84(&polygon)));
            f.set_property("width", width);
            thickened.push(f);
        }
    }

    let json = serde_json::json!({
        "input": FeatureCollection::from(&mercator.to_wgs84(&GeometryCollection::from_iter(input_polygons))),
        "skeletons": FeatureCollection::from(&mercator.to_wgs84(&GeometryCollection::from_iter(skeletons))),
        "perps": FeatureCollection::from(&mercator.to_wgs84(&GeometryCollection::from_iter(perps))),
        "thickened": GeoJson::from(thickened),
        "wkt_input": wkt_input,
    });
    Ok(json.to_string())
}

fn err_to_js<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}
