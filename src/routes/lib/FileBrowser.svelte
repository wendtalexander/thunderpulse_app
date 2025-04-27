<script lang="ts">
  import { open } from "@tauri-apps/plugin-dialog";

  let { onFileSelected } = $props<{
    onFileSelected: (selectedPath: string | string[] | null) => void;
  }>();
  let isLoading = $state(false); // Use $state for local reactive state
  let errorMessage = $state<string | null>(null); // Use $state
  async function openFileSelector() {
    isLoading = true;
    errorMessage = null;
    let selectedPath: string | string[] | null = null;

    try {
      const selected = await open({
        multiple: false,
        title: "Select File(s)",
      });

      if (selected === null) {
        console.log("File selection cancelled.");
        selectedPath = null;
      } else {
        console.log("File(s) selected in component:", selected);
        selectedPath = selected;
      }
    } catch (error) {
      console.error("Error opening file dialog:", error);
      errorMessage = `Error opening dialog: ${error}`;
      selectedPath = null;
    } finally {
      isLoading = false;
      // --- Rune-based approach ---
      // 2. Call the function passed down via props
      onFileSelected(selectedPath);
      // --- End Rune-based approach ---
    }
  }
</script>

<div class="file-browser-controls">
  <!-- Use the reactive variables directly -->
  <button onclick={openFileSelector} disabled={isLoading}>
    {#if isLoading}
      Loading...
    {:else}
      Browse Files
    {/if}
  </button>

  {#if errorMessage}
    <p class="error-message">Error: {errorMessage}</p>
  {/if}
</div>

<style>
  /* Styles remain the same */
  .file-browser-controls {
    padding: 1em 0;
    border: 1px dashed lightgray;
    margin-bottom: 1em;
    padding: 0.5em;
  }
  .error-message {
    color: red;
    font-size: 0.9em;
    margin-top: 0.5em;
  }

  button {
    border-radius: 8px;
    border: 1px solid transparent;
    padding: 0.6em 1.2em;
    font-size: 1em;
    font-weight: 500;
    font-family: inherit;
    color: #0f0f0f;
    background-color: #ffffff;
    transition: border-color 0.25s;
    box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    cursor: pointer;
  }
  button:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }
</style>
