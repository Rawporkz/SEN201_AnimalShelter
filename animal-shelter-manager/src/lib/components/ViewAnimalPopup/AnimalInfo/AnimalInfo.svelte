<!-- 
ViewAnimalModal/AnimalInfo.svelte

This file defines a reusable AnimalInfo component.
-->
<script lang="ts">
  import type { Animal } from "../../../utils/animal-utils";
  import {
    formatTimestamp,
    calculateAge,
    getStatusDisplayText,
  } from "../../../utils/animal-utils";
  import { ImageOff } from "@lucide/svelte";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import ClosePopupButton from "../../ClosePopupButton/ClosePopupButton.svelte";

  // Props
  interface Props {
    /** The animal to display information about */
    animal: Animal;
    /** The adopter information (optional) */
    adoption_timestamp: number;
    /** Callback function to close the popup */
    onclose?: () => void;
  }

  /** Component props with default values */
  const { animal, adoption_timestamp, onclose }: Props = $props();

  /** Helper to display neutered status as Yes/No */
  const getNeuteredText = (neutered: boolean) => (neutered ? "Yes" : "No");

  /** Helper to display birth month/year as string */
  const getBirthMonthYear = (month: number, year: number) => `${month}/${year}`;

  /**
   * Returns the appropriate CSS class for the animal's status
   * @param status - The status of the animal
   * @returns The CSS class for the status
   */
  const getStatusClass = (status: string) => {
    return status.toLowerCase().replace(" ", "-");
  };
</script>

<div class="animal-info-modal">
  <div class="close-button-wrapper">
    <ClosePopupButton onclick={onclose} />
  </div>
  <div class="animal-profile-left">
    <div class="animal-image">
      {#if animal?.image_path}
        <img src={convertFileSrc(animal.image_path)} alt="" />
      {:else}
        <div class="placeholder-image">
          <ImageOff size={48} />
        </div>
      {/if}
    </div>

    <div class="status-section">
      <div class="status-label">Status</div>
      <button
        class="status-badge {getStatusClass(
          getStatusDisplayText(animal?.status),
        )}"
      >
        {getStatusDisplayText(animal?.status)}
      </button>
    </div>

    <div class="dates-section">
      <div class="date-item">
        <div class="date-label">Admission Date</div>
        <div class="date-value">
          {animal?.admission_timestamp
            ? formatTimestamp(animal.admission_timestamp)
            : "Unknown"}
        </div>
      </div>
      <!-- Adoption date is not in Animal, so leave blank or add if available -->
      <div class="date-item">
        <div class="date-label">Adoption Date</div>
        <div class="date-value">
          {adoption_timestamp
            ? formatTimestamp(adoption_timestamp)
            : "Not adopted"}
        </div>
      </div>
    </div>
  </div>

  <div class="animal-profile-right">
    <div class="info-grid">
      <div class="info-row">
        <div class="info-item">
          <div class="info-label">Name</div>
          <div class="info-value">{animal?.name ?? "Unknown"}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Birth Month/Year</div>
          <div class="info-value">
            {animal && animal.birth_year != null && animal.birth_month != null
              ? `${getBirthMonthYear(animal.birth_month, animal.birth_year)} (${calculateAge(animal.birth_year, animal.birth_month)})`
              : "Unknown"}
          </div>
        </div>
      </div>

      <div class="info-row">
        <div class="info-item">
          <div class="info-label">Specie</div>
          <div class="info-value">{animal?.specie ?? "Unknown"}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Breed</div>
          <div class="info-value">{animal?.breed ?? "Unknown"}</div>
        </div>
      </div>

      <div class="info-row">
        <div class="info-item">
          <div class="info-label">Sex</div>
          <div class="info-value">{animal?.sex ?? "Unknown"}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Neutered</div>
          <div class="info-value">
            {animal ? getNeuteredText(animal.neutered) : "Unknown"}
          </div>
        </div>
      </div>
    </div>

    <div class="divider"></div>

    <!-- Appearance and bioCharacteristics are not in Animal, so omit or add if available in future -->
    <div class="description-section">
      <div class="description-item">
        <div class="description-label">Appearance</div>
        <div class="description-value">{animal?.appearance ?? "Unknown"}</div>
      </div>

      <div class="description-item">
        <div class="description-label">Bio & Characteristics</div>
        <div class="description-value">
          {animal?.bio ?? "No Description Available"}
        </div>
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
