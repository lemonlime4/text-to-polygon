<script lang="ts">
  import {
    load_font,
    to_desmos_expr,
    type FontSettings,
    type SamplingMethod,
  } from "./lib/wasm/rust";
  import DebugComparison from "./lib/DebugComparison.svelte";
  import { onMount } from "svelte";
  import { defaultExpressions, setExpressions } from "./lib/calc";
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

  $effect(() => console.log(samplingMethod));

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
        console.log(e);
        return { file: fontFile, error: `${e}` };
      }
    }),
  );

  $effect(() => {
    fontLoaded;
    let pointLatex = to_desmos_expr(inputText, fontSettings, samplingMethod);
    if (pointLatex === undefined) return;
    calc.setExpression({ id: "P", latex: "P=" + pointLatex });
  });
</script>

<h1>Text to desmos</h1>
<div class="container">
  <div class="panel">
    <input
      style:display="block"
      type="file"
      // accept=".ttf, .otf, .woff, .woff2, font/ttf, font/otf, font/woff, font/woff2"
      bind:files={fileList}
    />
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

    <h2>Text settings</h2>
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
          <p>Flatness tolerance</p>
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
          ><input type="checkbox" bind:checked={showDebug} />Debug UI for text
          shaping</label
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
