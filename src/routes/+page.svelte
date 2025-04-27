<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import FileBrowser from "./lib/FileBrowser.svelte";

  // import Highcharts from "highcharts";
  import * as Highcharts from "highcharts";
  import "highcharts/modules/exporting";
  // import Boost from "highcharts/modules/boost";
  import "highcharts/modules/boost";
  // Apply Highcharts modules
  // ExportingModule(Highcharts);
  // Boost(Highcharts);

  import { Chart } from "@highcharts/svelte";
  function getData(n: number) {
    const arr = [];
    let a, b, c, spike;
    for (let i = 0; i < n; i = i + 1) {
      if (i % 100 === 0) {
        a = 2 * Math.random();
      }
      if (i % 1000 === 0) {
        b = 2 * Math.random();
      }
      if (i % 10000 === 0) {
        c = 2 * Math.random();
      }
      if (i % 50000 === 0) {
        spike = 0;
      } else {
        spike = 0;
      }
      arr.push([i, 2 * Math.sin(i / 100) + a + b + c + spike + Math.random()]);
    }
    return arr;
  }

  function getSeries(n: number, s: number) {
    let i = 0;
    const r = [];

    for (; i < s; i++) {
      r.push({
        data: getData(n),
        lineWidth: 1,
        // yAxis: s,
        // xAxis: 0,
        boostThreshold: 1,
      });
    }

    return r;
  }

  const n = 48000,
    s = 32,
    series = getSeries(n, s);

  console.time("line");
  let options: Highcharts.Options = {
    chart: {
      zooming: {
        type: "x",
      },
    },

    title: {
      text: "Highcharts drawing " + n * s + " points across " + s + " series",
    },

    legend: {
      enabled: false,
    },

    boost: {
      useGPUTranslations: true,
      debug: {
        timeRendering: true,
        timeSetup: true,
      },
      // seriesThreshold: 5,
    },

    xAxis: {
      min: 0,
      max: n,
      ordinal: true,
    },

    navigator: {
      xAxis: {
        ordinal: false,
        min: 0,
        max: 10,
      },
    },
    yAxis: {
      min: -3,
      max: 9,
    },

    subtitle: {
      text: "Using the Boost module",
    },

    tooltip: {
      valueDecimals: 2,
    },

    series: series,
  };
  console.timeEnd("line");

  let { onFileSelected } = $props<{
    onFileSelected: (selectedPath: string | string[] | null) => void;
  }>();
</script>

<main class="container">
  <h1>Thunderpluse</h1>
  <div class="two-column-container">
    <div class="column left-column">
      <h2>Left Column</h2>
      <FileBrowser {onFileSelected} />
    </div>
    <div class="column righ-column">
      <h2>Right Column</h2>
      <Chart {options} highcharts={Highcharts} />
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
