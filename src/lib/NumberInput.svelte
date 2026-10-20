<script lang="ts">
  type Props = { value: number; small?: boolean; error?: boolean };

  let {
    value: numValue = $bindable(),
    small = false,
    error = false,
  }: Props = $props();
  let text = $derived(numValue.toString().replace("+", ""));
  let invalidText = $state(false);
</script>

<input
  type="text"
  bind:value={
    () => text,
    (s) => {
      let x = +s;
      // todo improve decimal literal validation
      let validText =
        Number.isFinite(x) &&
        x === Number.parseFloat(s) &&
        !s.includes("x") &&
        !s.includes("b");
      invalidText = !validText;
      if (validText) {
        numValue = x;
      }
      text = s;
    }
  }
  class={{ error: error || invalidText, small }}
/>

<style>
  .error {
    background-color: rgb(255 0 0 / 0.1);
  }
  .small {
    min-width: 0;
    field-sizing: content;
  }
</style>
