<script lang="ts">
  import type { Attachment } from "svelte/attachments";
  import { to_desmos_expr } from "./lib/wasm/rust";
  import DebugComparison from "./lib/DebugComparison.svelte";

  let inputText: string = $state("The five boxing wizards jump quickly.");
  let fileList: FileList = $state(new DataTransfer().files);
  let fontFile: File | null = $derived(fileList[0] || null);
</script>

<h1>Text to desmos</h1>
<input
  style:display="block"
  type="file"
  accept=".ttf, .otf, .woff, .woff2, font/ttf, font/otf, font/woff, font/woff2"
  bind:files={fileList}
/>
<textarea
  bind:value={inputText}
  style="width: 30em; height: 2lh; font-family: serif;"></textarea>

{#if fontFile}
  <p>
    Loaded font "{fontFile.name}" - {fontFile.size} bytes
  </p>
  {#await fontFile.bytes() then fontBytes}
    {let polygonLatex = to_desmos_expr(fontBytes, inputText)}
    <p style="overflow: scroll;">
      <code>
        {polygonLatex}
      </code>
    </p>
    <button onclick={() => navigator.clipboard.writeText(polygonLatex)}
      >Copy</button
    >
  {/await}
{/if}

{let showDebug = $state(false)}
<p>
  <label><input type="checkbox" bind:checked={showDebug} />Debug ui</label>
</p>
{#if showDebug}
  <DebugComparison {inputText} {fontFile} />
{/if}
