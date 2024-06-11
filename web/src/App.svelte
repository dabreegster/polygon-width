<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init, { findWidths } from "backend";
  import { Layout } from "svelte-utils/two_column_layout";
  import { Modal } from "svelte-utils";
  import type { Map } from "maplibre-gl";
  import {
    LineLayer,
    FillLayer,
    GeoJSON,
    MapLibre,
    hoverStateFilter,
  } from "svelte-maplibre";
  import type { FeatureCollection, LineString, Polygon } from "geojson";
  import bbox from "@turf/bbox";
  import {
    PolygonTool,
    PolygonControls,
    PolygonToolLayer,
  } from "maplibre-draw-polygon";
  import Settings from "./Settings.svelte";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

  let input: FeatureCollection<Polygon> | null = null;
  let skeletons: FeatureCollection<LineString> | null = null;
  let perps: FeatureCollection<LineString> | null = null;
  let thickened: FeatureCollection<Polygon, { width: number }> | null = null;
  let wkt_input = "";

  let showInput = true;
  let showSkeletons = true;
  let showPerps = true;
  let showThickened = false;
  let showWkt = false;

  let map: Map;
  let fileInput: HTMLInputElement;
  let polygonTool: PolygonTool | null = null;

  let inputString = "";
  let cfg = {
    remove_holes: 100.0,

    flip_orientation: false,
    filter_skeletons_outside: true,
    filter_skeletons_near_boundary: 0.1,
    join_skeletons: true,

    make_perps_step_size: 5.0,
  };
  let shouldZoom = true;

  async function handleInput(gj: string, cfg: any) {
    if (!gj) {
      return;
    }
    await init();

    let results = JSON.parse(findWidths(gj, cfg));
    input = results.input;
    skeletons = results.skeletons;
    perps = results.perps;
    thickened = results.thickened;
    wkt_input = results.wkt_input;

    if (shouldZoom) {
      map?.fitBounds(bbox(input!) as [number, number, number, number], {
        animate: false,
        padding: 10,
      });
      // Only do it once per input; when cfg changes, don't jump around
      shouldZoom = false;
    }
  }
  $: handleInput(inputString, cfg);

  async function loadFile(e: Event) {
    shouldZoom = true;
    inputString = await fileInput.files![0].text();
  }

  function startPolygonTool() {
    if (!map) {
      return;
    }
    polygonTool = new PolygonTool(map);
    polygonTool.startNew();
    polygonTool.addEventListenerSuccess(async (f) => {
      polygonTool = null;
      shouldZoom = true;
      inputString = JSON.stringify(f);
    });
    polygonTool.addEventListenerFailure(() => {
      polygonTool = null;
    });
  }
</script>

<Layout>
  <div slot="left">
    <h1>Polygon width</h1>

    {#if polygonTool}
      <PolygonControls {polygonTool} />
    {:else}
      <label>
        Load a .geojson file with polygons
        <input bind:this={fileInput} on:change={loadFile} type="file" />
      </label>

      <div>
        <button type="button" on:click={startPolygonTool}>
          Draw your own polygon
        </button>
      </div>

      {#if wkt_input}
        <div>
          <button on:click={() => (showWkt = true)}>Copy polygon as WKT</button>
        </div>
      {/if}
    {/if}

    <hr />

    <label>
      <input type="checkbox" bind:checked={showInput} />
      Show input polygons
    </label>
    <label>
      <input type="checkbox" bind:checked={showSkeletons} />
      Show center line
    </label>
    <label>
      <input type="checkbox" bind:checked={showPerps} />
      Show perpendicular lines
    </label>
    <label>
      <input type="checkbox" bind:checked={showThickened} />
      Show thickened polygons
    </label>

    <hr />

    <Settings bind:cfg />
  </div>
  <div slot="main" style="position:relative; width: 100%; height: 100vh;">
    <MapLibre
      style={`https://api.maptiler.com/maps/dataviz/style.json?key=${maptilerApiKey}`}
      standardControls
      hash
      bind:map
    >
      <PolygonToolLayer />

      {#if input}
        <GeoJSON data={input}>
          <FillLayer
            paint={{ "fill-color": "black", "fill-opacity": 0.5 }}
            layout={{ visibility: showInput ? "visible" : "none" }}
          />
        </GeoJSON>
      {/if}
      {#if skeletons}
        <GeoJSON data={skeletons} generateId>
          <LineLayer
            manageHoverState
            paint={{
              "line-color": hoverStateFilter("red", "blue"),
              "line-width": 4,
            }}
            layout={{ visibility: showSkeletons ? "visible" : "none" }}
          />
        </GeoJSON>
      {/if}
      {#if perps}
        <GeoJSON data={perps}>
          <LineLayer
            paint={{ "line-color": "green", "line-width": 2 }}
            layout={{ visibility: showPerps ? "visible" : "none" }}
          />
        </GeoJSON>
      {/if}
      {#if thickened}
        <GeoJSON data={thickened}>
          <FillLayer
            paint={{ "fill-color": "cyan", "fill-opacity": 0.5 }}
            layout={{ visibility: showThickened ? "visible" : "none" }}
          />
        </GeoJSON>
      {/if}
    </MapLibre>
  </div>
</Layout>

{#if showWkt}
  <Modal on:close={() => (showWkt = false)}>
    <textarea rows="10">{wkt_input}</textarea>
  </Modal>
{/if}
