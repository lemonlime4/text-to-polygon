<script lang="ts">
  import {
    load_font,
    to_desmos_expr,
    type FontSettings,
    type SamplingMethod,
  } from "./lib/wasm/rust";
  import DebugComparison from "./lib/DebugComparison.svelte";
  import { onMount } from "svelte";
  import {
    defaultExpressions,
    numberToLatex,
    setExpressions,
  } from "./lib/calc";
  import type { Attachment } from "svelte/attachments";
  import Slider from "./lib/Slider.svelte";
  import NumberInput from "./lib/NumberInput.svelte";

  let inputText: string = $state("five");
  let fileList: FileList = $state(new DataTransfer().files);
  let fontFile: File | undefined = $derived(fileList[0]);

  let fontSettings: FontSettings = $state({ font_size: 5.0 });

  type UnionToIntersection<U> = (
    U extends any ? (arg: U) => void : never
  ) extends (arg: infer I) => void
    ? I
    : never;
  type KeysOf<U> = keyof UnionToIntersection<U>;

  let methodState: UnionToIntersection<SamplingMethod> = $state({
    Subdivision: { tolerance: 2 },
    ControlPoints: { to_cubic: false },
  });
  let methodName = $state("Subdivision" as KeysOf<SamplingMethod>);
  let samplingMethod = $derived({
    [methodName]: methodState[methodName],
  } as SamplingMethod);

  let calc: Desmos.Calculator;
  const initializeCalc: Attachment<HTMLElement> = (elem) => {
    calc = Desmos.GraphingCalculator(elem, {});
    // calc._calc.graphSettings.config.editOnWeb = true;
    setExpressions(calc, defaultExpressions);
    // todo: this causes incorrect home button behavior after resize
    // calc.setDefaultState(calc.getState());
    return () => calc.destroy();
  };

  // todo split this into two separate derived variables, one for file and one for error
  let fontLoaded: File | { file: File; error: string } | undefined = $derived(
    await fontFile?.bytes().then(async (bytes) => {
      try {
        load_font(bytes);
        return fontFile;
      } catch (e) {
        return { file: fontFile, error: `${e}` };
      }
    }),
  );

  $effect(() => {
    fontLoaded;
    let latexOutput = to_desmos_expr(inputText, fontSettings, samplingMethod);
    if (latexOutput === undefined) return;
    let { latex, advance } = latexOutput;
    calc.setExpression({ id: "points", latex: "P=" + latex });
    calc.setExpression({
      id: "advance",
      latex: numberToLatex`P_a=1\\cdot  \\left(${advance.x},${advance.y}\\right)`,
    });
  });

  $effect(() => {
    calc.setExpression({
      id: "font-size",
      latex: numberToLatex`P_s=1\\cdot${fontSettings.font_size}`,
    });
  });

  function exportToClipboard() {
    const state = JSON.stringify(JSON.stringify(calc.getState()));
    navigator.clipboard.writeText(`Calc.setState(JSON.parse(${state}))`);
  }
</script>

<svelte:window
  on:beforeunload={(event) => {
    event.preventDefault();
  }}
/>

<h1>Text to desmos</h1>
<div class="container">
  <div class="panel">
    <p style="margin-bottom: 1em;">Convert a text segment to polygons</p>
    <input
      type="text"
      bind:value={inputText}
      style="width: 100%; padding: 0.3em 0 0.3em 0.5em; font-family: serif;"
    />

    {#if fontLoaded instanceof File}
      <p>
        Loaded font "{fontLoaded.name}" - {fontLoaded.size} bytes
      </p>
    {:else if fontLoaded}
      <p style="color: red; margin-top: 1em;">
        Error loading {fontLoaded.file.name} - {fontLoaded.error}
      </p>
    {/if}

    <p style="margin-top: 1em">
      <button onclick={exportToClipboard}>Export</button> - copies code to
      clipboard, open a new desmos tab and paste it in developer console
      <br />
      see
      <a
        href="https://www.desmos.com/api/v1.12/docs/index.html#document-author-features"
        target="_blank">this</a
      > page for info on readonly expressions
    </p>

    <h2>Text settings</h2>
    <div class="section">
      <p>Font:</p>
      <input
        style:display="block"
        type="file"
        // accept=".ttf, .otf, .woff, .woff2, font/ttf, font/otf, font/woff, font/woff2"
        bind:files={fileList}
      />
    </div>
    <div class="section">
      <p>Font size</p>
      <NumberInput bind:value={fontSettings.font_size} />
      <br />
      <Slider bind:value={fontSettings.font_size} min={0.5} max={16} />
    </div>
    <div class="section">
      <p>Sampling method</p>
      <p>
        <label
          ><input type="radio" bind:group={methodName} value="ControlPoints" /> control
          points (debug)</label
        >
        <label
          ><input type="radio" bind:group={methodName} value="Subdivision" /> subdivision</label
        >
      </p>

      <div style="margin: 0.5em 0 0 1.5em;">
        {#if methodName === "Subdivision"}
          <p>Flatness tolerance (in font space units)</p>
          <NumberInput bind:value={methodState.Subdivision.tolerance} />
          <br />
          <Slider
            min={0.5}
            max={10}
            bind:value={methodState.Subdivision.tolerance}
          />
        {:else if methodName === "ControlPoints"}
          <p>
            <input
              type="checkbox"
              bind:checked={methodState.ControlPoints.to_cubic}
            />
            Convert quadratic curves to cubic
          </p>
        {/if}
      </div>
    </div>

    <!-- debug ui -->
    <div class="section">
      {let showDebug = $state(false)}
      <p>
        <label
          ><input type="checkbox" bind:checked={showDebug} />debug text shaping</label
        >
      </p>
      {#if showDebug}
        <DebugComparison
          {inputText}
          fontFile={((x) => (x instanceof File ? x : undefined))(fontLoaded)}
          {fontSettings}
        />
      {/if}
    </div>
  </div>

  <div {@attach initializeCalc} style="height: 100%;"></div>
</div>

<style>
  div.container {
    display: grid;
    grid-template-columns: 22em 2fr;
    flex: 1;
    gap: 1em;
  }

  /* .container > .panel {
    overflow-y: auto;
  } */

  .section {
    margin-top: 1em;
  }

  @media (width < 50em) {
    /* todo */
  }

  h1 {
    margin-top: 0;
  }

  p {
    margin: 0;
  }
</style>
