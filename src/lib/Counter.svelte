<script lang="ts">
  import { get_counter, set_counter } from "./wasm/rust";

  let count = $state(get_counter());
  function increment() {
    set_counter(count + 1);
    count = get_counter();
  }
</script>

<button type="button" class="counter" onclick={increment}>
  Count is {count}
</button>
