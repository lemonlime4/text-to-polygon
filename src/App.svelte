<script lang="ts">
  import type { Attachment } from "svelte/attachments";
  import { load_font, to_desmos_expr } from "./lib/wasm/rust";
  import DebugComparison from "./lib/DebugComparison.svelte";

  let inputText: string = $state("The five boxing wizards jump quickly.");
  let fileList: FileList = $state(new DataTransfer().files);
  let fontFile: File | undefined = $derived(fileList[0]);
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
  {let fontLoaded: Promise<void> = $derived(fontFile.bytes().then(load_font))}
  {#await fontLoaded then _}
    <p>
      Loaded font "{fontFile.name}" - {fontFile.size} bytes
    </p>
    <svelte:boundary>
      {let polygonLatex = $derived(to_desmos_expr(inputText))}
      {#if polygonLatex !== undefined}
        <p style="overflow: scroll;">
          <code>
            {polygonLatex}
          </code>
        </p>
        <p>
          <button onclick={() => navigator.clipboard.writeText(polygonLatex)}
            >Copy</button
          >
          - {polygonLatex.length} characters
        </p>
      {/if}
      {#snippet failed(error)}
        <p style:color="red">{error}</p>
      {/snippet}
    </svelte:boundary>
  {/await}
{/if}

{let showDebug = $state(true)}
<p>
  <label><input type="checkbox" bind:checked={showDebug} />Debug ui</label>
</p>
{#if fontFile && showDebug}
  <DebugComparison {inputText} {fontFile} />
{/if}
