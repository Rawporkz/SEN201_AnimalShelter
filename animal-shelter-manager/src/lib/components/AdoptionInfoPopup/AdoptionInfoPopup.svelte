<!-- 
lib/components/AdoptionInfoPopup/AdoptionInfoPopup.svelte

This file defines a reusable AdoptionInfoPopup component that displays 
adoption information with tabbed navigation between animal and adopter views.
-->
<script lang="ts">
  import "./style.scss";
  import { User, Dog } from "@lucide/svelte";
  import AdopterInfo from "./AdopterInfo/AdopterInfo.svelte";
  import AnimalInfo from "./AnimalInfo/AnimalInfo.svelte";
  import ClosePopupButton from "../ClosePopupButton/ClosePopupButton.svelte";

  // Type definition for tab selection
  type Tab = "animal" | "adopter";

  // Currently active tab state
  let activeTab: Tab = "animal";

  // Controls whether the popup is visible
  export let isOpen = false;

  /**
   * Switches the active tab to the specified tab
   * @param tab - The tab to switch to ("animal" or "adopter")
   */
  function selectTab(tab: Tab) {
    activeTab = tab;
  }

  /**
   * Closes the popup by setting isOpen to false
   */
  function closePopup() {
    isOpen = false;
  }
</script>

{#if isOpen}
  <div
    class="animal-profile-overlay"
    role="button"
    tabindex="0"
    aria-label="Close popup"
    on:click={closePopup}
    on:keydown={(e) => {
      if (e.key === "Enter" || e.key === " ") closePopup();
    }}
  >
    <div
      class="popup-content"
      role="dialog"
      aria-modal="true"
      on:click|stopPropagation
      tabindex="0"
      aria-label="Adoption Info Popup"
      on:keydown={(e) => {
        if (e.key === "Escape") closePopup();
      }}
    >
      <div class="close-button-wrapper">
        <ClosePopupButton on:click={closePopup} />
      </div>
      <div class="tab-content">
        {#if activeTab === "adopter"}
          <AdopterInfo
            name="John Doe"
            occupation="Software Engineer"
            annualIncome="100k-150k THB"
            email="john.doe@example.com"
            phoneNumber="081-234-5678"
            streetAddress="123 Main St, Bangkok"
            country="Thailand"
            numOfHousehold={4}
            numOfChildren={1}
          />
        {:else}
          <AnimalInfo
            animalName="Buddy"
            birthMonth="4/2023"
            age="2 years old"
            species="Dog"
            breed="Beagle"
            sex="Male"
            neutered="Yes"
            imageUrl="https://example.com/buddy.jpg"
            status="Adopted"
            admissionDate="1/10/2024"
            adoptionDate="1/10/2025"
            appearance="Big brown eyes, Black back and tail, White legs, Has a white dot on the back near its tail"
            bioCharacteristics="A classic tri-color Beagle, this sweet boy is a perfect example of his breed: curious, friendly, and always ready for adventure. Great with kids, so he loves to follow his nose! He is excellent with children and other pets, thriving on companionship and daily walks."
          />
        {/if}
      </div>
    </div>
  </div>

  <div class="tab-container">
    <button
      class="tab-button"
      class:active={activeTab === "adopter"}
      on:click={() => selectTab("adopter")}
      aria-label="Adopter Profile"
    >
      <User size={40} />
    </button>

    <button
      class="tab-button"
      class:active={activeTab === "animal"}
      on:click={() => selectTab("animal")}
      aria-label="Animal Info"
    >
      <Dog size={40} />
    </button>
  </div>
{/if}
