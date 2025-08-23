<script lang="ts">
  let { value = $bindable(false), label }: { value: boolean; label: string } =
    $props();
</script>

<input type="checkbox" bind:checked={value} id={label} />
<label class="text-default-colors" for={label}>{label}</label>
