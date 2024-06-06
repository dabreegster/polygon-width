# polygon-width

This is an experimental tool to take polygons representing pavements or roads and to calculate width at regular intervals along it. The overall approach is:

1) Calculate a "center line" of each polygon, currently using `geo_buffer::skeleton_of_polygon_to_linestring`
2) Walk along that line at regular intervals
3) Project a perpendicular line left and right
4) Intersect with the original polygon

Currently everything's tuned for proprietary Ordnance Survey input, but the approach should work on any sort of thickened linear object.
