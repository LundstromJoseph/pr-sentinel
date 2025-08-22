<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main
  class="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col items-center justify-center p-8"
>
  <h1 class="text-4xl font-bold text-gray-900 dark:text-white mb-8">
    Welcome to Tauri + Svelte
  </h1>

  <div class="flex justify-center items-center space-x-6 mb-8">
    <a href="https://vite.dev" target="_blank" class="group">
      <img
        src="/vite.svg"
        class="h-24 p-6 transition-all duration-300 group-hover:drop-shadow-[0_0_2em_#747bff]"
        alt="Vite Logo"
      />
    </a>
    <a href="https://tauri.app" target="_blank" class="group">
      <img
        src="/tauri.svg"
        class="h-24 p-6 transition-all duration-300 group-hover:drop-shadow-[0_0_2em_#24c8db]"
        alt="Tauri Logo"
      />
    </a>
    <a href="https://svelte.dev" target="_blank" class="group">
      <img
        src="/svelte.svg"
        class="h-24 p-6 transition-all duration-300 group-hover:drop-shadow-[0_0_2em_#ff3e00]"
        alt="SvelteKit Logo"
      />
    </a>
  </div>

  <p class="text-gray-600 dark:text-gray-300 mb-8 text-center">
    Click on the Tauri, Vite, and SvelteKit logos to learn more.
  </p>

  <form
    class="flex justify-center items-center space-x-4 mb-6"
    onsubmit={greet}
  >
    <input
      id="greet-input"
      placeholder="Enter a name..."
      bind:value={name}
      class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg bg-white dark:bg-gray-800 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition-colors"
    />
    <button
      type="submit"
      class="px-6 py-2 bg-blue-600 hover:bg-blue-700 text-white font-medium rounded-lg transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-900"
    >
      Greet
    </button>
  </form>

  {#if greetMsg}
    <p class="text-lg text-green-600 dark:text-green-400 font-medium">
      {greetMsg}
    </p>
  {/if}
</main>
