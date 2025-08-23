<script lang="ts">
  interface Props {
    value: string;
    placeholder?: string;
    textColor?: "default" | "error";
  }

  let {
    value = $bindable(),
    placeholder,
    textColor = "default",
  }: Props = $props();

  let textColorClass = $derived(
    textColor === "error" ? "text-error-colors" : "text-default-colors"
  );
</script>

<input
  type="text"
  class="border-b w-full {textColorClass} border-blue-900 focus:border-blue-600 p-2 outline-none"
  {placeholder}
  bind:value
/>
