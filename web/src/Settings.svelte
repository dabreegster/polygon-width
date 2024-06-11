<script lang="ts">
  import OptionalNumber from "./OptionalNumber.svelte";

  export let cfg: any;
</script>

<details open>
  <summary>Config</summary>

  <OptionalNumber
    label="Remove holes in input polygons larger than this area (sq m)"
    bind:value={cfg.remove_holes}
    defaultNumber={100}
  />

  <div>
    <label>
      <input type="checkbox" bind:checked={cfg.flip_orientation} />
      Flip orientation of polygons (need to try both depending on input)
    </label>
  </div>

  <div>
    <label>
      <input type="checkbox" bind:checked={cfg.filter_skeletons_outside} />
      Filter skeleton lines outside polygon
    </label>
  </div>

  <OptionalNumber
    label="Filter skeleton lines closer than this to the boundary (m)"
    bind:value={cfg.filter_skeletons_near_boundary}
    defaultNumber={0.1}
  />

  <div>
    <label>
      <input type="checkbox" bind:checked={cfg.join_skeletons} />
      Join skeleton lines
    </label>
  </div>

  <OptionalNumber
    label="Generate perpendicular lines at this step size (m)"
    bind:value={cfg.make_perps_step_size}
    defaultNumber={5.0}
  />
</details>

<style>
  details {
    border: 1px solid white;
    padding: 4px;
  }
</style>
