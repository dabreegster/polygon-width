# Polygon widths

This tool takes polygons representing pavements or roads and calculates the
width at regular intervals along it. This is useful for assessing active travel
schemes -- are pavements wide enough for foot traffic? Does a road have enough
space to fit a cycle lane? It likely has other uses.

Try it out: <https://dabreegster.github.io/polygon-width>

## How it works

TODO: Outdated, there are more refinements now

1. Calculate a straight skeleton of each polygon, currently using [geo_buffer](https://docs.rs/geo-buffer/latest/geo_buffer/fn.skeleton_of_polygon_to_linestring.html)
2. Clean up that output to get the "center line" of the polygon
3. Walk along that line at regular intervals
4. Project a perpendicular line left and right
5. Intersect with the original polygon
6. Record the width

## Contributing

If you know an existing tool that solves the center-line problem well for the
example inputs, please open an issue and let me know; I'd love to avoid
reinventing wheels!

If you find or draw an interesting test case, you can add it
[here](https://github.com/dabreegster/polygon-width/blob/main/web/src/test_cases.ts)
by drawing the polygon and then pressing "Copy polygon as WKT" or manually
exporting your polygon to GeoJSON or WKT.

## Related work

There are many other packages solving at least part of this problem. This one
has an interactive web app (by compiling Rust to WebAssembly), making it as
easy as possible to try out the algorithm on your input data and tune
parameters.

- [Python centerline](https://centerline.readthedocs.io/en/latest/index.html)
- [centerline_rs](https://codeberg.org/eadf/centerline_rs.git)
