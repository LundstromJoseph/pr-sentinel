<script lang="ts">
  interface Props {
    width?: number;
    height?: number;
  }

  let { width = 24, height = 24 }: Props = $props();
</script>

<svg
  class="dark:fill-white fill-black"
  {width}
  {height}
  viewBox="0 0 24 24"
  fill="none"
>
  <path
    d="M4 12H20M12 4V20"
    class="dark:stroke-white stroke-black"
    stroke-width="2"
    stroke-linecap="round"
    stroke-linejoin="round"
  />
</svg>
