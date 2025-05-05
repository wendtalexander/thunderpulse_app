<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import FileBrowser from "./lib/FileBrowser.svelte";
  import { onMount } from "svelte";

  import Highcharts from "highcharts";
  // import * as Highcharts from "highcharts";
  import "highcharts/modules/exporting";
  import "highcharts/modules/boost";

  import { Chart } from "@highcharts/svelte";

  interface SeriesObject {
    data: [number, number][];
    lineWidth: number;
    boostThreshold: number;
  }

  function getHighchartsSeries(
    n: number,
    s: number,
  ): Promise<Highcharts.SeriesLineOptions[]> {
    return invoke<SeriesObject[]>("get_series", { n, s }).then((seriesArr) =>
      seriesArr.map((s) => ({
        type: "line",
        data: s.data,
        lineWidth: s.lineWidth,
        boostThreshold: s.boostThreshold,
      })),
    );
  }
  let n = 48000;
  let s = 16;
  let seriesPromise = getHighchartsSeries(n, s);

  let baseOptions: Highcharts.Options = {
    chart: { zooming: { type: "x" } },
    title: { text: `Highcharts drawing ${n * s} points across ${s} series` },
    legend: { enabled: false },
    boost: {
      useGPUTranslations: true,
      debug: { timeRendering: true, timeSetup: true },
    },
    xAxis: { min: 0, max: n, ordinal: true },
    navigator: { xAxis: { ordinal: false, min: 0, max: 10 } },
    yAxis: { min: -3, max: 9 },
    subtitle: { text: "Using the Boost module" },
    tooltip: { valueDecimals: 2 },
  };

  let { onFileSelected } = $props<{
    onFileSelected: (selectedPath: string | string[] | null) => void;
  }>();
</script>

<main class="container">
  <h1>Thunderpulse</h1>
  <div class="two-column-container">
    <div class="column left-column">
      <h2>Left Column</h2>
      <FileBrowser {onFileSelected} />
    </div>
    <div class="column righ-column">
      <h2>Right Column</h2>
      {#await seriesPromise}
        <p>Loading chart data...</p>
      {:then series}
        <Chart options={{ ...baseOptions, series }} />
      {:catch error}
        <p style="color:crimson;">Error loading chart: {error}</p>
      {/await}
    </div>
  </div>
</main>

<style>
  :root {
    font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
    font-size: 16px;
    line-height: 24px;
    font-weight: 400;

    color: #0f0f0f;
    background-color: #f6f6f6;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-moothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
  }
  .two-column-container {
    display: grid;
    /* Creates two columns of equal fractional width */
    grid-template-columns: 33.33% 66.66%;
    /* Optional: Adds a gap between the columns */
    gap: 1rem; /* Adjust the gap as needed */
    /* Ensures the layout takes at least the full viewport height */
    min-height: 100vh;
    /* Optional: Add padding around the whole container */
    /* padding: 1rem; */
    box-sizing: border-box; /* Include padding/border in width calculation */
  }

  .column {
    /* Optional: Add padding inside each column */
    padding: 1rem;
    /* Optional: Add borders for visualization during development */
    border: 1px dashed #ccc;
  }

  /* Optional: Specific styles for left/right columns if needed */
  .left-column {
    /* background-color: #f8f8f8; */ /* Example background */
  }

  .right-column {
    /* Styles for the main content area */
  }

  .container {
    margin: 0;
    padding-top: 1vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;
  }

  h1 {
    text-align: center;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      color: #f6f6f6;
      background-color: #2f2f2f;
    }
  }
</style>
