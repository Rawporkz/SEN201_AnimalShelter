<!-- 
lib/components/AdoptionInfoPopup/AnimalInfo.svelte

This file defines a reusable AnimalInfo component.
-->
<script lang="ts">
  import "./style.scss";
  import { emit } from "@tauri-apps/api/event";
  import ClosePopupButton from "../../ClosePopupButton/ClosePopupButton.svelte";

  // Animal Information
  export let animalName: string = "Animal Name 1";
  export let birthMonth: string = "4/2023";
  export let age: string = "2 years old";
  export let species: string = "Dog";
  export let breed: string = "Beagle";
  export let sex: string = "Male";
  export let neutered: string = "Yes";
  export let imageUrl: string = "";

  // Status
  export let status: string = "Adopted"; // "Adopted", "Available", "Pending"

  // Dates
  export let admissionDate: string = "1/10/2024";
  export let adoptionDate: string = "1/10/2025";

  // Description
  export let appearance: string =
    "Big brown eyes, Black back and tail, White legs, Has a white dot on the back near its tail";
  export let bioCharacteristics: string =
    "A classic tri-color Beagle, this sweet boy is a perfect example of his breed: curious, friendly, and always ready for adventure. Great with kids, so he loves to follow his nose! He is excellent with children and other pets, thriving on companionship and daily walks.";

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
