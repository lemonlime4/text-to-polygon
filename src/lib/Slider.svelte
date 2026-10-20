<script lang="ts">
  import type { HTMLInputAttributes } from "svelte/elements";
  import NumberInput from "./NumberInput.svelte";

  type Props = {
    value: number;
    min: number;
    max: number;
    rangeInputProps?: HTMLInputAttributes;
  };

  let { value = $bindable(), min, max, rangeInputProps = {} }: Props = $props();
</script>

<NumberInput bind:value={min} small error={value < min} />
<input type="range" {min} {max} step="any" bind:value {...rangeInputProps} />
<NumberInput bind:value={max} small error={value > max} />
