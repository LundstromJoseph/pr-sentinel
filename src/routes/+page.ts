import { createState } from "$lib/appstate.svelte";

export const ssr = false;
export const prerender = false;

export function load() {
  const state = createState();
  return { state };
}
