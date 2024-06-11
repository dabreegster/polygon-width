<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init, { findWidths } from "backend";
  import { Geocoder } from "svelte-utils/map";
  import { Layout } from "svelte-utils/two_column_layout";
  import type { Map } from "maplibre-gl";
  import { FillLayer, GeoJSON, MapLibre } from "svelte-maplibre";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    await init();

    let text = await fileInput.files![0].text();
    let results = findWidths(text);
    console.log(results);
  }

  let map: Map;
</script>

<Layout>
  <div slot="left">
    <h1>Polygon width</h1>

    <label>
      Load a .geojson file
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={`https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
    >
      <Geocoder {map} apiKey={maptilerApiKey} />
    </MapLibre>
  </div>
</Layout>
