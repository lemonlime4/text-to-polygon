<script lang="ts">
  type Props = { value: number; small?: boolean; error?: boolean };

  let { value = $bindable(), small = false, error = false }: Props = $props();
  let text = $derived(value.toString());
  let invalidText = $state(false);

  $effect(() => console.log(small));
</script>

<input
  type="text"
  bind:value={
    () => text,
    (s) => {
      text = s;
      let x = +s;
      if (Number.isFinite(x) && x === Number.parseFloat(s)) {
        value = x;
        invalidText = false;
      } else {
        invalidText = true;
      }
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
