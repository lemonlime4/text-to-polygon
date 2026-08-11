<script lang="ts">
  import Counter from "./lib/Counter.svelte";

  let str: string = "world";
  let a: number = $state(1);
  let b = $derived(2 * a);
  setInterval(() => a++, 1000);
</script>

<p>Hey {a} * 2 = {b}</p>
<Counter></Counter>
