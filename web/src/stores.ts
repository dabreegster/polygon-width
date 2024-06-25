import { type Map } from "maplibre-gl";
import { writable, type Writable } from "svelte/store";

export let map: Writable<Map | null> = writable(null);
