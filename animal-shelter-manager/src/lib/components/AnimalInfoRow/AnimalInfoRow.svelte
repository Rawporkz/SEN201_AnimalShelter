<!--
  AnimalInfoRow.svelte

  Reusable component that displays animal information in a horizontal row layout,
  showing animal image, name, ID, species/breed, sex, and admission date.
-->

<script lang="ts">
  import type { AnimalSummary } from "$lib/utils/animal-utils";
  import {
    formatTimestamp,
    getStatusDisplayText,
  } from "$lib/utils/animal-utils";
  import { ImageOff } from "@lucide/svelte";
  import type { Snippet } from "svelte";

  // Props
  interface Props {
    /** Animal summary data to display */
    animalSummary: AnimalSummary;
    /** Whether to show the status indicator next to the name */
    showStatus?: boolean;
    /** Action buttons or components to display on the right side */
    children?: Snippet;
  }

  const { animalSummary, showStatus = false, children }: Props = $props();

  /** Flag to track if the animal has a valid image */
  let hasValidImage: boolean = $state(!!animalSummary.image_path);

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
    {#if animalSummary.image_path && hasValidImage}
      <img
        src={animalSummary.image_path}
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
      {#if showStatus && animalSummary.status}
        <div class="status-indicator status-{animalSummary.status}">
          <div class="status-tooltip">
            {getStatusDisplayText(animalSummary.status)}
          </div>
        </div>
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
          >{formatTimestamp(animalSummary.admission_timestamp)}</span
        >
      </div>

      <div class="info-item">
        <span class="info-label">Sex:</span>
        <span class="info-value">{animalSummary.sex}</span>
      </div>
    </div>
  </div>

  {#if children}
    <div class="action-buttons">
      {@render children()}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "./style.scss";
</style>
