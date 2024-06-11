<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init, { findWidths } from "backend";
  import { Layout } from "svelte-utils/two_column_layout";
  import type { Map } from "maplibre-gl";
  import { LineLayer, FillLayer, GeoJSON, MapLibre } from "svelte-maplibre";
  import type { FeatureCollection, LineString, Polygon } from "geojson";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

  let input: FeatureCollection<Polygon> | null = null;
  let skeletons: FeatureCollection<LineString> | null = null;
  let perps: FeatureCollection<LineString> | null = null;
  let thickened: FeatureCollection<Polygon, { width: number }> | null = null;

  let showInput = true;
  let showSkeletons = true;
  let showPerps = true;
  let showThickened = false;

  let map: Map;
  let fileInput: HTMLInputElement;
  async function loadFile(e: Event) {
    await init();

    let text = await fileInput.files![0].text();
    let results = JSON.parse(findWidths(text));
    input = results.input;
    skeletons = results.skeletons;
    perps = results.perps;
    thickened = results.thickened;

    // TODO zoom
  }
</script>

<Layout>
  <div slot="left">
    <h1>Polygon width</h1>

    <label>
      Load a .geojson file
      <input bind:this={fileInput} on:change={loadFile} type="file" />
    </label>

    <label>
      <input type="checkbox" bind:checked={showInput} />
      Show input polygons
    </label>
    <label>
      <input type="checkbox" bind:checked={showSkeletons} />
      Show skeletons
    </label>
    <label>
      <input type="checkbox" bind:checked={showPerps} />
      Show perpendicular lines
    </label>
    <label>
      <input type="checkbox" bind:checked={showThickened} />
      Show thickened polygons
    </label>
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={`https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
    >
      {#if input}
        <GeoJSON data={input}
          ><FillLayer
            paint={{ "fill-color": "black", "fill-opacity": 0.5 }}
            layout={{ visibility: showInput ? "visible" : "none" }}
          /></GeoJSON
        >
      {/if}
      {#if skeletons}
        <GeoJSON data={skeletons}
          ><LineLayer
            paint={{ "line-color": "red", "line-width": 2 }}
            layout={{ visibility: showSkeletons ? "visible" : "none" }}
          /></GeoJSON
        >
      {/if}
      {#if perps}
        <GeoJSON data={perps}
          ><LineLayer
            paint={{ "line-color": "green", "line-width": 1 }}
            layout={{ visibility: showPerps ? "visible" : "none" }}
          /></GeoJSON
        >
      {/if}
      {#if thickened}
        <GeoJSON data={thickened}
          ><FillLayer
            paint={{ "fill-color": "cyan", "fill-opacity": 0.5 }}
            layout={{ visibility: showThickened ? "visible" : "none" }}
          /></GeoJSON
        >
      {/if}
    </MapLibre>
  </div>
</Layout>
