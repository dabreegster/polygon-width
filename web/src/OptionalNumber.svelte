<script lang="ts">
  export let label: string;
  export let value: number | undefined;
  export let defaultNumber: number;
  // Use both as step size and minimum
  export let step: number;

  let checked = value != undefined;

  $: if (checked && value == undefined) {
    value = defaultNumber;
  } else if (!checked) {
    value = undefined;
  }
</script>

<label>
  <input type="checkbox" bind:checked />
  {label}

  <input type="number" bind:value disabled={!checked} {step} min={step} />
</label>
