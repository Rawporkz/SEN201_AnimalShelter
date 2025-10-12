<!-- 
lib/components/AdoptionInfoPopup/AdoptionInfoPopup.svelte

This file defines a reusable AdoptionInfoPopup component that displays 
adoption information with tabbed navigation between animal and adopter views.
-->

<script lang="ts">
  import { User, Dog } from "@lucide/svelte";
  import AdopterInfo from "./AdopterInfo/AdopterInfo.svelte";
  import AnimalInfo from "./AnimalInfo/AnimalInfo.svelte";
  import ClosePopupButton from "../ClosePopupButton/ClosePopupButton.svelte";
  import { AnimalStatus } from "$lib/utils/animal-utils";
  import { RequestStatus } from "$lib/utils/animal-utils";

  /** Type definition for tab selection in the adoption info popup */
  type Tab = "animal" | "adopter";

  // Props
  interface Props {
    /** Controls whether the popup is visible */
    isOpen?: boolean;
    /** Callback function to close the popup */
    onClose?: () => void;
  }

  /** Component props with default values */
  const { isOpen = false, onClose }: Props = $props();

  /** Currently active tab state - tracks which tab (animal or adopter) is being displayed */
  let activeTab: Tab = $state("animal");

  /**
   * Switches the active tab to the specified tab.
   *
   * @param tab - The tab to switch to ("animal" or "adopter")
   * @returns void
   */
  function selectTab(tab: Tab): void {
    activeTab = tab;
  }

  /**
   * Closes the popup by calling the onClose callback if it exists.
   *
   * @returns void
   */
  function closePopup(): void {
    onClose?.();
  }
</script>

{#if isOpen}
  <div
    class="animal-profile-overlay"
    role="button"
    tabindex="0"
    aria-label="Close popup"
    onclick={closePopup}
    onkeydown={(e) => {
      if (e.key === "Enter" || e.key === " ") closePopup();
    }}
  >
    <div
      class="popup-content"
      role="dialog"
      aria-modal="true"
      onclick={(e) => e.stopPropagation()}
      tabindex="0"
      aria-label="Adoption Info Popup"
      onkeydown={(e) => {
        if (e.key === "Escape") closePopup();
      }}
    >
      <div class="close-button-wrapper">
        <ClosePopupButton onclick={closePopup} />
      </div>
      <div class="tab-content">
        {#if activeTab === "adopter"}
          <AdopterInfo
            adopter={{
              id: "adopter-001",
              name: "John Doe",
              occupation: "Software Engineer",
              annual_income: "150K - 200K THB",
              email: "john.doe@example.com",
              tel_number: "081-234-5678",
              address: "123 Main St, Bangkok",
              country: "Thailand",
              num_people: 4,
              num_children: 1,
              status: RequestStatus.PENDING,
              adoption_timestamp: 20250115,
              request_timestamp: 20240115,
              animal_id: "animal-001",
            }}
          />
        {:else}
          <AnimalInfo
            animal={{
              id: "animal-001",
              name: "Buddy",
              birth_month: 4,
              birth_year: 2021,
              specie: "Dog",
              breed: "Beagle",
              sex: "Male",
              neutered: true,
              image_path: "https://example.com/buddy.jpg",
              status: AnimalStatus.ADOPTED,
              admission_timestamp: 20240110,
              adoption_timestamp: 20250110,
              appearance:
                "Big brown eyes, Black back and tail, White legs, Has a white dot on the back near its tail",
              bio: "A classic tri-color Beagle, this sweet boy is a perfect example of his breed: curious, friendly, and always ready for adventure. Great with kids, so he loves to follow his nose! He is excellent with children and other pets, thriving on companionship and daily walks.",
            }}
          />
        {/if}
      </div>
    </div>
  </div>

  <div class="tab-container">
    <button
      class="tab-button"
      class:active={activeTab === "adopter"}
      onclick={() => selectTab("adopter")}
      aria-label="Adopter Profile"
    >
      <User size={40} />
    </button>

    <button
      class="tab-button"
      class:active={activeTab === "animal"}
      onclick={() => selectTab("animal")}
      aria-label="Animal Info"
    >
      <Dog size={40} />
    </button>
  </div>
{/if}

<style lang="scss">
  @use "./style.scss";
</style>
