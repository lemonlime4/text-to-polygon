<script lang="ts">
  import {
    load_font,
    to_desmos_expr,
    type FontSettings,
    type SamplingMethod,
  } from "./lib/wasm/rust";
  import DebugComparison from "./lib/DebugComparison.svelte";
  import { onMount } from "svelte";

  let inputText: string = $state("five");
  let fileList: FileList = $state(new DataTransfer().files);
  let fontFile: File | undefined = $derived(fileList[0]);
  let fontSettings: FontSettings = $state({ font_size: 16.0 });
  let samplingMethod: SamplingMethod = $state({
    Subdivision: { tolerance: 0.01 },
  });

  let calcElem: HTMLDivElement;
  let calc: Desmos.Calculator;
  onMount(() => {
    calc = Desmos.GraphingCalculator(calcElem, {
      // expressions: false,
    });
    calc.setExpression({
      id: "1",
      latex: "\\operatorname{polygon}\\left(P\\right)",
    });
    calc.setExpression({
      id: "P",
      latex: "P=\\left[\\right]",
      pointSize: 4,
    });
    return () => calc.destroy();
  });

  $effect(() => {
    $state.snapshot(fontFile);
    let pointLatex = to_desmos_expr(inputText, fontSettings, samplingMethod);
    if (pointLatex === undefined) return;
    calc.setExpression({ id: "P", latex: "P=" + pointLatex });
  });
</script>

<h1>Text to desmos</h1>
<div>
  <p>
    Font size
    <input
      type="range"
      min="0.1"
      max="16"
      step="0.1"
      bind:value={fontSettings.font_size}
    />
  </p>
  <p>Sampling method</p>
  <div>
    <button
      onclick={() => (samplingMethod = { ControlPoints: { to_cubic: false } })}
      >control points (debug)</button
    >
  </div>
  <div>
    <button
      onclick={() => (samplingMethod = { Subdivision: { tolerance: 0.01 } })}
      >subdivision
    </button>
    {#if "Subdivision" in samplingMethod}
      subdivision tolerance
      <input
        type="range"
        min="0.001"
        max="0.1"
        step="any"
        bind:value={samplingMethod.Subdivision.tolerance}
      />
    {/if}
  </div>
</div>
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
      {#snippet failed(error)}
        <p style:color="red">{error}</p>
      {/snippet}
    </svelte:boundary>
  {/await}
{/if}

{let showDebug = $state(false)}
<p>
  <label><input type="checkbox" bind:checked={showDebug} />Debug ui</label>
</p>
{#if fontFile && showDebug}
  <DebugComparison {inputText} {fontFile} {fontSettings} />
{/if}

<div bind:this={calcElem} style="height: 400px;"></div>
