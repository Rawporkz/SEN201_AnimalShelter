<!-- 
AdoptionInfoPopup/AnimalInfo.svelte

This file defines a reusable AnimalInfo component.
-->
<script lang="ts">
  // Props
  interface Props {
    /** Animal's name */
    animalName?: string;
    /** Animal's birth month/year */
    birthMonth?: string;
    /** Animal's age description */
    age?: string;
    /** Animal species (Dog, Cat, etc.) */
    species?: string;
    /** Animal breed */
    breed?: string;
    /** Animal's sex (Male/Female) */
    sex?: string;
    /** Whether animal is neutered (Yes/No) */
    neutered?: string;
    /** URL to animal's image */
    imageUrl?: string;
    /** Animal's current status (Adopted, Available, Pending) */
    status?: string;
    /** Date animal was admitted to shelter */
    admissionDate?: string;
    /** Date animal was adopted */
    adoptionDate?: string;
    /** Description of animal's physical appearance */
    appearance?: string;
    /** Description of animal's bio and characteristics */
    bioCharacteristics?: string;
  }

  const {
    animalName = "Animal Name 1",
    birthMonth = "4/2023",
    age = "2 years old",
    species = "Dog",
    breed = "Beagle",
    sex = "Male",
    neutered = "Yes",
    imageUrl = "",
    status = "Adopted",
    admissionDate = "1/10/2024",
    adoptionDate = "1/10/2025",
    appearance = "Big brown eyes, Black back and tail, White legs, Has a white dot on the back near its tail",
    bioCharacteristics = "A classic tri-color Beagle, this sweet boy is a perfect example of his breed: curious, friendly, and always ready for adventure. Great with kids, so he loves to follow his nose! He is excellent with children and other pets, thriving on companionship and daily walks.",
  }: Props = $props();

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
  <div class="animal-profile-left">
    <div class="animal-image">
      {#if imageUrl}
        <img src={imageUrl} alt="" />
      {:else}
        <div class="placeholder-image">No Image</div>
      {/if}
    </div>

    <div class="status-section">
      <div class="status-label">Status</div>
      <button class="status-badge {getStatusClass(status)}">
        {status}
      </button>
    </div>

    <div class="dates-section">
      <div class="date-item">
        <div class="date-label">Admission Date</div>
        <div class="date-value">{admissionDate}</div>
      </div>
      <div class="date-item">
        <div class="date-label">Adoption Date</div>
        <div class="date-value">{adoptionDate}</div>
      </div>
    </div>
  </div>

  <div class="animal-profile-right">
    <div class="info-grid">
      <div class="info-row">
        <div class="info-item">
          <div class="info-label">Name</div>
          <div class="info-value">{animalName}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Birth Month/Year</div>
          <div class="info-value">{birthMonth} ({age})</div>
        </div>
      </div>

      <div class="info-row">
        <div class="info-item">
          <div class="info-label">Species</div>
          <div class="info-value">{species}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Breed</div>
          <div class="info-value">{breed}</div>
        </div>
      </div>

      <div class="info-row">
        <div class="info-item">
          <div class="info-label">Sex</div>
          <div class="info-value">{sex}</div>
        </div>
        <div class="info-item">
          <div class="info-label">Neutered</div>
          <div class="info-value">{neutered}</div>
        </div>
      </div>
    </div>

    <div class="divider"></div>

    <div class="description-section">
      <div class="description-item">
        <div class="description-label">Appearance</div>
        <div class="description-value">{appearance}</div>
      </div>

      <div class="description-item">
        <div class="description-label">Bio & Characteristics</div>
        <div class="description-value">{bioCharacteristics}</div>
      </div>
    </div>
  </div>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
