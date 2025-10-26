<!--
AnimalInfoRow.svelte

Reusable component that displays animal information in a horizontal row layout,
showing animal image, name, ID, species/breed, sex, and admission date.
-->

<script lang="ts">
  import type { AnimalSummary } from "$lib/utils/data-utils";
  import { formatTimestamp } from "$lib/utils/data-utils";
  import { ImageOff } from "@lucide/svelte";
  import type { Snippet } from "svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";

  // Props
  interface Props {
    /** Animal summary data to display */
    animalSummary: AnimalSummary;
    /** Whether to show the status indicator next to the name */
    status?: Snippet;
    /** Action buttons or components to display on the right side */
    actions?: Snippet;
  }

  const { animalSummary, status: status, actions: actions }: Props = $props();

  /** Flag to track if the animal has a valid image */
  let hasValidImage: boolean = $state(!!animalSummary.imagePath);

  /**
   * Handles image load errors by showing the placeholder instead.
   */
  function handleImageError(): void {
    hasValidImage = false;
  }

  /**
   * Handles successful image load.
   */
  function handleImageLoad(): void {
    hasValidImage = true;
  }
</script>

<div class="animal-info-row">
  <div class="animal-image-container">
    {#if animalSummary.imagePath && hasValidImage}
      <img
        src={convertFileSrc(animalSummary.imagePath)}
        alt="Photo of {animalSummary.name}"
        class="animal-image"
        onerror={handleImageError}
        onload={handleImageLoad}
      />
    {:else}
      <div class="animal-image-placeholder">
        <ImageOff size={32} />
      </div>
    {/if}
  </div>

  <div class="animal-details">
    <div class="animal-name-container">
      <h2 class="animal-name">{animalSummary.name}</h2>
      {#if status}
        {@render status()}
      {/if}
    </div>

    <div class="animal-info-grid">
      <div class="info-item">
        <span class="info-label">ID:</span>
        <span class="info-value">{animalSummary.id}</span>
      </div>

      <div class="info-item">
        <span class="info-label">Specie:</span>
        <span class="info-value"
          >{animalSummary.specie}, {animalSummary.breed}</span
        >
      </div>

      <div class="info-item">
        <span class="info-label">Since:</span>
        <span class="info-value"
          >{formatTimestamp(animalSummary.admissionTimestamp)}</span
        >
      </div>

      <div class="info-item">
        <span class="info-label">Sex:</span>
        <span class="info-value">{animalSummary.sex}</span>
      </div>
    </div>
  </div>

  {#if actions}
    <div class="action-buttons">
      {@render actions()}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "./style.scss";
</style>
