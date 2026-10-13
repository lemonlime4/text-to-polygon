<script lang="ts">
  import type { Attachment } from "svelte/attachments";
  import { to_svg_path, type FontSettings } from "./wasm/rust";

  let {
    inputText,
    fontFile,
    fontSettings,
  }: { inputText: string; fontFile: File | null; fontSettings: FontSettings } =
    $props();

  $effect(() => {
    if (!fontFile) return;
    let promise = fontFile?.arrayBuffer()?.then((fontArrayBuffer) => {
      let fontFace = new FontFace("User font", fontArrayBuffer);
      document.fonts.add(fontFace);
      return fontFace.load();
    });
    return () => promise.then((fontFace) => document.fonts.delete(fontFace));
  });

  let resizeSvg: Attachment<SVGSVGElement> = (svg) => {
    // inputText; // artificial dependency
    console.log("resizing svg");
    const { x, y, width, height } = svg.getBBox();
    svg.setAttribute("viewBox", `${x} ${y} ${width} ${height}`);
  };
</script>

{let compare = $state(true)}
{let svgOffsetX = $state(0)}
{let svgOffsetY = $state(0.5)}
{let svgScale = $state(1.0)}
<div
  style="width: 30em;
  padding: 0.5em;
  {compare ? 'background: rgb(0 0 0 / .1);' : ''}"
>
  <p style="margin: 0;">
    <label>
      <input type="checkbox" bind:checked={compare} /> Compare p and svg
    </label>
  </p>
  {#if compare}
    <label style="display: flex; width: 100%;">
      x offset&nbsp;
      <input
        type="range"
        style="flex: 1;"
        bind:value={svgOffsetX}
        min="-.5"
        max=".5"
        step="any"
      />
    </label>
    <label style="display: flex; width: 100%;">
      y offset&nbsp;
      <input
        type="range"
        style="flex: 1;"
        bind:value={svgOffsetY}
        min="-.5"
        max=".5"
        step="any"
      />
    </label>
    <label style="display: flex; width: 100%;">
      svg scale&nbsp;
      <input
        type="range"
        style="flex: 1;"
        bind:value={svgScale}
        min="0.9"
        max="1.1"
        step="any"
      />
    </label>
  {/if}
</div>
{#if fontFile !== null}
  {let textWidth: number = $state(0)}
  {#await fontFile.bytes() then fontFileBytes}
    <svelte:boundary>
      {let path = $derived(to_svg_path(inputText, fontSettings))}

      <div style="font-family: 'User font'; font-size: 2em;">
        <pre
          bind:clientWidth={textWidth}
          style="
        font-family: inherit;
        display: inline-block;
        {compare
            ? 'color: rgb(255 0 0/ 0.5); margin: 0;'
            : ''}">{inputText.trim()}</pre>
        <br />
        <svg
          {@attach resizeSvg}
          width={textWidth * (compare ? svgScale : 1.0)}
          style={compare
            ? `translate: ${svgOffsetX}em ${svgOffsetY}lh;
          position: relative;
          left: 0;
          top: -1lh;`
            : ""}
          ><path
            fill={compare ? "rgb(0 0 255 / 0.5)" : "black"}
            transform="scale(1,-1)"
            d={path}
          ></path></svg
        >
      </div>

      {#snippet failed(error)}
        <p style="color: red">{error}</p>
      {/snippet}
    </svelte:boundary>
  {/await}
{:else}
  <p>Upload a font to compare</p>
{/if}
