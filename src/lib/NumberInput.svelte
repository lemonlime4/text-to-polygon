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
      let validText =
        s !== "" &&
        !s.includes("x") &&
        !s.includes("X") &&
        !s.includes("b") &&
        !s.includes("B");
      invalidText = !validText;
      if (validText) {
        let x = +s;
        if (Number.isFinite(x)) {
          numValue = x;
        }
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
