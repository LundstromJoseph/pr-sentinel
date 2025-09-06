<script lang="ts">
  import Typography from "$lib/components/Typography.svelte";
  import TextInput from "$lib/components/TextInput.svelte";
  import Button from "$lib/components/Button.svelte";
  import Spinner from "$lib/components/Spinner.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import type { AppState } from "$lib/appstate.svelte";

  interface Props {
    appState: AppState;
  }
  let { appState }: Props = $props();

  let token = $state(appState.config.github_token ?? "");

  let verifiedUsername = $state<string | null>(null);

  let tokenStatus = $state<
    "verified" | "unverified" | "dirty" | "saved" | "loading"
  >("dirty");

  $effect(() => {
    if (appState.config.github_token !== token) {
      tokenStatus = "dirty";
    }
  });

  async function verifyToken() {
    tokenStatus = "loading";
    const username = await invoke<string>("verify_token", { token }).catch(
      (e) => {
        console.error(e);
        tokenStatus = "unverified";
      }
    );
    if (username) {
      tokenStatus = "verified";
      verifiedUsername = username;
    }
  }

  async function saveToken() {
    tokenStatus = "loading";
    await invoke("save_token", { token, username: verifiedUsername }).catch(
      (e) => {
        console.error(e);
        tokenStatus = "unverified";
      }
    );
    tokenStatus = "saved";
  }
</script>

<section class="flex flex-col gap-2 p-2">
  <Typography component="h5">GitHub token</Typography>
  <TextInput
    bind:value={token}
    placeholder="ghp_xxxxx"
    textColor={tokenStatus === "unverified" ? "error" : "default"}
  />

  {#if tokenStatus !== "loading"}
    <div class="flex gap-4">
      <Button
        onClick={verifyToken}
        classes="w-20"
        enabled={["dirty", "unverified"].includes(tokenStatus)}
      >
        <Typography>{"Verify"}</Typography>
      </Button>

      <Button
        onClick={saveToken}
        classes="w-20"
        enabled={["verified"].includes(tokenStatus)}
      >
        <Typography>
          {"Save"}
        </Typography>
      </Button>
    </div>
  {/if}

  {#if tokenStatus === "loading"}
    <Spinner />
  {/if}

  {#if tokenStatus === "unverified"}
    <Typography color="error">Invalid token</Typography>
  {/if}
  {#if tokenStatus === "verified"}
    <Typography>Token verified for {verifiedUsername}</Typography>
  {/if}
  {#if tokenStatus === "saved"}
    <Typography>Token for {verifiedUsername} is saved</Typography>
  {/if}
</section>
