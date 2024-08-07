<script lang="ts">
  import "@picocss/pico/css/pico.jade.min.css";
  import init, { findWidths } from "backend";
  import { Layout } from "svelte-utils/two_column_layout";
  import { Modal } from "svelte-utils";
  import { Popup } from "svelte-utils/map";
  import type { Map } from "maplibre-gl";
  import {
    LineLayer,
    FillLayer,
    GeoJSON,
    MapLibre,
    hoverStateFilter,
    CircleLayer,
  } from "svelte-maplibre";
  import type {
    FeatureCollection,
    LineString,
    Polygon,
    Geometry,
    GeoJsonProperties,
    Point,
  } from "geojson";
  import bbox from "@turf/bbox";
  import {
    PolygonTool,
    PolygonControls,
    PolygonToolLayer,
  } from "maplibre-draw-polygon";
  import Settings from "./Settings.svelte";
  import { testCases } from "./test_cases";
  import { parse as parseWkt } from "wkt";
  import { layerId } from "./zorder";
  import { map as mapStore } from "./stores";

  let maptilerApiKey = "MZEJTanw3WpxRvt7qDfo";

  let input: FeatureCollection<Polygon> = emptyGj();
  let skeletons: FeatureCollection<LineString> = emptyGj();
  let perps: FeatureCollection<LineString> = emptyGj();
  let thickened: FeatureCollection<
    Polygon,
    { width1: number; width2: number }
  > = emptyGj();
  let centerWithWidth: FeatureCollection<
    LineString,
    { min_width: number; max_width: number }
  > = emptyGj();
  let wkt_input = "";

  let showInput = true;
  let showSkeletons = true;
  let showPerps = true;
  let showThickened = false;
  let showCenterWithWidth = false;
  let showWkt = false;

  let map: Map;
  $: if (map) {
    mapStore.set(map);
    // @ts-expect-error TODO For debugging
    window.map = map;
  }

  let fileInput: HTMLInputElement;
  let polygonTool: PolygonTool | null = null;
  let currentTestCase = "";

  let inputString = "";
  let cfg = {
    remove_holes: 100.0,

    filter_skeletons_outside: true,
    filter_skeletons_near_boundary: 0.1,
    join_skeletons: true,
    remove_short_skeletons: 0.1,

    make_perps_step_size: 5.0,
    perp_midpoint_ratio: 0.5,

    width_granularity: 0.5,
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
    centerWithWidth = results.center_with_width;
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
    currentTestCase = "";
    inputString = await fileInput.files![0].text();
  }

  function startPolygonTool(edit: boolean) {
    if (!map) {
      return;
    }
    polygonTool = new PolygonTool(map);
    if (edit) {
      polygonTool.editExisting(JSON.parse(inputString));
    } else {
      polygonTool.startNew();
    }
    polygonTool.addEventListenerSuccess(async (f) => {
      polygonTool = null;
      shouldZoom = true;
      currentTestCase = "";
      inputString = JSON.stringify(f);
    });
    polygonTool.addEventListenerFailure(() => {
      polygonTool = null;
    });
  }

  function emptyGj<
    G extends Geometry = Geometry,
    P = GeoJsonProperties,
  >(): FeatureCollection<G, P> {
    return {
      type: "FeatureCollection" as const,
      features: [],
    };
  }

  function addLinestringEndpoints(
    input: FeatureCollection<LineString>,
  ): FeatureCollection<Point> {
    let output = {
      type: "FeatureCollection",
      features: [],
    } as FeatureCollection<Point>;
    for (let f of input.features) {
      for (let pt of [
        f.geometry.coordinates[0],
        f.geometry.coordinates[f.geometry.coordinates.length - 1],
      ]) {
        output.features.push({
          type: "Feature",
          properties: {},
          geometry: {
            type: "Point",
            coordinates: JSON.parse(JSON.stringify(pt)),
          },
        });
      }
    }
    return output;
  }

  $: if (currentTestCase != "") {
    // @ts-expect-error
    let test = testCases[currentTestCase];
    shouldZoom = true;
    inputString = JSON.stringify({
      type: "Feature",
      geometry: parseWkt(test),
      properties: {},
    });
  }
</script>

<Layout>
  <div slot="left">
    <h1>Polygon width</h1>
    <a
      href="https://github.com/dabreegster/polygon-width?tab=readme-ov-file#polygon-width"
      target="_blank"
    >
      About
    </a>

    <details open>
      <summary>Input</summary>

      {#if polygonTool}
        <PolygonControls {polygonTool} />
      {:else}
        <label>
          Load a .geojson file with polygons
          <input bind:this={fileInput} on:change={loadFile} type="file" />
        </label>

        <div>
          <button type="button" on:click={() => startPolygonTool(false)}>
            Draw your own polygon
          </button>
        </div>

        {#if wkt_input}
          <div>
            <button class="secondary" on:click={() => (showWkt = true)}>
              Copy polygon as WKT
            </button>
            <button
              class="secondary"
              on:click={() => startPolygonTool(true)}
              disabled={polygonTool != null}
            >
              Edit polygon
            </button>
          </div>
        {/if}

        <label>
          Choose a test case:

          <select bind:value={currentTestCase}>
            <option value="">None</option>
            {#each Object.keys(testCases) as key}
              <option value={key}>{key}</option>
            {/each}
          </select>

          (some from OpenStreetMap)
        </label>
      {/if}
    </details>

    <hr />

    <details open>
      <summary>Layers</summary>

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
      <label>
        <input type="checkbox" bind:checked={showCenterWithWidth} />
        Show center lines with width
      </label>
    </details>

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

      <GeoJSON id="input" data={input}>
        <FillLayer
          {...layerId("input-polygons")}
          paint={{ "fill-color": "black", "fill-opacity": 0.5 }}
          layout={{ visibility: showInput ? "visible" : "none" }}
        />
      </GeoJSON>

      <GeoJSON id="skeletons" data={skeletons} generateId>
        <LineLayer
          {...layerId("skeletons")}
          manageHoverState
          paint={{
            "line-color": hoverStateFilter("red", "blue"),
            "line-width": 4,
          }}
          layout={{ visibility: showSkeletons ? "visible" : "none" }}
        />
      </GeoJSON>

      <GeoJSON id="perps" data={perps}>
        <LineLayer
          {...layerId("perps")}
          paint={{ "line-color": "green", "line-width": 2 }}
          layout={{ visibility: showPerps ? "visible" : "none" }}
        />
      </GeoJSON>

      <GeoJSON id="thickened" data={thickened} generateId>
        <FillLayer
          {...layerId("thickened")}
          manageHoverState
          paint={{
            "fill-color": "cyan",
            "fill-opacity": hoverStateFilter(0.5, 0.8),
          }}
          layout={{ visibility: showThickened ? "visible" : "none" }}
        >
          <Popup let:props>
            <p>
              Width between {props.width1.toFixed(1)}m and {props.width2.toFixed(
                1,
              )}m
            </p>
          </Popup>
        </FillLayer>
      </GeoJSON>

      <GeoJSON id="centerWithWidth" data={centerWithWidth} generateId>
        <LineLayer
          {...layerId("center-with-width")}
          manageHoverState
          paint={{
            "line-color": hoverStateFilter("purple", "black"),
            "line-width": 6,
          }}
          layout={{ visibility: showCenterWithWidth ? "visible" : "none" }}
        >
          <Popup let:props>
            <p>{props.min_width.toFixed(1)} - {props.max_width.toFixed(1)}m</p>
          </Popup>
        </LineLayer>
      </GeoJSON>
      <GeoJSON
        id="centerWithWidth"
        data={addLinestringEndpoints(centerWithWidth)}
      >
        <CircleLayer
          {...layerId("center-endpoints")}
          paint={{
            "circle-opacity": 0,
            "circle-radius": 10,
            "circle-stroke-color": "purple",
            "circle-stroke-width": 2,
          }}
          layout={{ visibility: showCenterWithWidth ? "visible" : "none" }}
        />
      </GeoJSON>
    </MapLibre>
  </div>
</Layout>

{#if showWkt}
  <Modal on:close={() => (showWkt = false)}>
    <textarea rows="10">{wkt_input}</textarea>
  </Modal>
{/if}

<style>
  details {
    border: 1px solid white;
    padding: 4px;
  }
</style>
