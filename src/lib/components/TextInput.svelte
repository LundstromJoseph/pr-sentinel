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

  const colors = {
    default: "text-text-default",
    error: "text-text-error",
  };

  let textColorClass = $derived(colors[textColor]);
</script>

<input
  type="text"
  class="border-b w-full {textColorClass} border-border-default focus:border-button-primary p-2 outline-none"
  {placeholder}
  bind:value
/>
