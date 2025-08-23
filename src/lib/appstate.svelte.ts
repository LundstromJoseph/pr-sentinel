import { invoke } from "@tauri-apps/api/core";
import type { AppConfig, AppData } from "./types";
import type { get } from "svelte/store";

export type AppState = Awaited<ReturnType<typeof createState>>;

export async function createState() {
  const config: AppConfig = (await invoke("get_config")) as AppConfig;

  const data: AppData = config.github_token
    ? ((await invoke("get_data")) as AppData)
    : { pull_requests: {} };

  let state = $state({
    config,
    data,
  });

  return {
    get config() {
      return state.config;
    },
    set config(config: AppConfig) {
      state.config = config;
    },
    get data() {
      return state.data;
    },
    set data(data: AppData) {
      state.data = data;
    },
  };
}
