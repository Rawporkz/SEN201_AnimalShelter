<!--
AnimalAdoptionInfoRow.svelte

Reusable component that displays both animal information and adopter/requester
information in a horizontal row layout.
-->

<script lang="ts">
  import type { AnimalSummary, AdoptionRequest } from "$lib/utils/animal-utils";
  import { formatTimestamp } from "$lib/utils/animal-utils";
  import { ImageOff } from "@lucide/svelte";
  import type { Snippet } from "svelte";

  // Props
  interface Props {
    /** Animal summary data to display */
    animalSummary: AnimalSummary;
    /** Adoption request data to display (for adopter info) */
    adoptionRequest: AdoptionRequest;
    /** Whether to show the "by" word between animal and adopter columns */
    showBySeparator?: boolean;
    /** Action buttons or components to display on the right side */
    actions?: Snippet;
  }

  const {
    animalSummary,
    adoptionRequest,
    showBySeparator = true,
    actions: actions,
  }: Props = $props();

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

<div class="animal-adoption-info-row">
  <div class="left-section">
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
      <h2 class="animal-name">{animalSummary.name}</h2>

      <div class="animal-info-grid">
        <div class="info-item">
          <span class="info-label">ID:</span>
          <span class="info-value">{animalSummary.id}</span>
        </div>

        <div class="info-item">
          <span class="info-label">Since:</span>
          <span class="info-value"
            >{formatTimestamp(animalSummary.admission_timestamp)}</span
          >
        </div>
      </div>
    </div>
  </div>

  {#if showBySeparator}
    <div class="by-separator">by</div>
  {/if}

  <div class="right-section">
    <div class="adopter-details">
      <h2 class="adopter-name">{adoptionRequest.name}</h2>

      <div class="adopter-info-grid">
        <div class="info-item">
          <span class="info-label">Email:</span>
          <span class="info-value">{adoptionRequest.email}</span>
        </div>

        <div class="info-item">
          <span class="info-label">On:</span>
          <span class="info-value"
            >{formatTimestamp(adoptionRequest.adoption_timestamp)}</span
          >
        </div>
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
