<script lang="ts">
  import type { Attachment } from "svelte/attachments";
  import { to_svg_path, type FontSettings } from "./wasm/rust";
  import Slider from "./Slider.svelte";

  type Props = {
    inputText: string;
    fontFile: File | undefined;
    fontSettings: FontSettings;
  };

  let { inputText, fontFile, fontSettings }: Props = $props();

  $effect(() => {
    if (!(fontFile instanceof File)) return;
    let promise = fontFile?.arrayBuffer()?.then((fontArrayBuffer) => {
      let fontFace = new FontFace("User font", fontArrayBuffer);
      document.fonts.add(fontFace);
      return fontFace.load();
    });
    return () => promise.then((fontFace) => document.fonts.delete(fontFace));
  });

  let resizeSvg: Attachment<SVGSVGElement> = (svg) => {
    fontFile;
    inputText; // artificial dependency
    console.log("resizing svg");
    const { x, y, width, height } = svg.getBBox();
    svg.setAttribute("viewBox", `${x} ${y} ${width} ${height}`);
  };

  let compare = $state(true);
  let svgOffsetX = $state(0);
  let svgOffsetY = $state(0.5);
  let svgScale = $state(1.0);
</script>

<div class={["overlap-settings", { compare }]}>
  <p style="margin: 0;">
    <label>
      <input type="checkbox" bind:checked={compare} /> Overlap p and svg
    </label>
  </p>
  {#if compare}
    {let rangeInputProps = { style: "flex: 1;" }}
    <label style="display: flex; width: 100%;">
      x offset&nbsp;
      <Slider min={-0.5} max={0.5} bind:value={svgOffsetX} {rangeInputProps} />
    </label>
    <label style="display: flex; width: 100%;">
      y offset&nbsp;
      <Slider min={-0.5} max={0.5} bind:value={svgOffsetY} {rangeInputProps} />
    </label>
    <label style="display: flex; width: 100%;">
      svg scale&nbsp;
      <Slider min={-0.9} max={1.1} bind:value={svgScale} {rangeInputProps} />
    </label>
  {/if}
</div>
{#if fontFile instanceof File}
  {let textWidth: number = $state(0)}
  {let path = $derived((void fontFile, to_svg_path(inputText, fontSettings)))}

  <div class="wrapper">
    <pre
      class={["pre", { compare }]}
      bind:clientWidth={textWidth}>{inputText.trim()}</pre>
    <br />
    <div
      class={["svg", { compare }]}
      style="--offset-x: {svgOffsetX}em; --offset-y: {svgOffsetY}lh;"
    >
      <svg {@attach resizeSvg} width={textWidth * (compare ? svgScale : 1.0)}
        ><path
          fill={compare ? "rgb(0 0 255 / 0.5)" : "black"}
          transform="scale(1,-1)"
          d={path}
        ></path></svg
      >
    </div>
  </div>
{:else}
  <p style="margin: 1em 1.5em;">Upload a font to compare</p>
{/if}

<style>
  .overlap-settings {
    padding: 0.5em 1.5em;
    &.compare {
      background: rgb(0 0 0 / 0.1);
    }
  }
  .wrapper {
    font-family: "User font";
    font-size: 2em;
    margin-left: 1em;
  }

  .pre {
    margin: 0;
    position: relative;
    font-family: inherit;
    display: inline-block;
    &.compare {
      color: rgb(255 0 0 / 0.5);
      margin: 0;
    }
  }
  .svg {
    position: relative;
    &.compare {
      top: calc(var(--offset-y) - 1lh);
      & > svg {
        translate: calc(var(--offset-x)) 0;
      }
    }
  }
  .pre::before {
    font-family: initial;
    font-size: 0.5em;
    position: absolute;
    inset: 50% auto auto 0;
    transform: translate(calc(-1em - 100%), -50%);
    content: "html";
  }
  .svg::before {
    font-family: initial;
    font-size: 0.5em;
    position: absolute;
    inset: 50% auto auto 0;
    transform: translate(calc(-1em - 100%), -50%);
    content: "svg";
  }
  .compare.svg::before {
    color: rgb(0 0 255 / 0.5);
  }
</style>
