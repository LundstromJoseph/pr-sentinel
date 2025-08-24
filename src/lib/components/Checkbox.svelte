<script lang="ts">
  let { value = $bindable(false), label }: { value: boolean; label: string } =
    $props();
</script>

<div class="flex items-center gap-1">
  <input type="checkbox" bind:checked={value} id={label} />
  <label class="text-default-text" for={label}>{label}</label>
</div>
