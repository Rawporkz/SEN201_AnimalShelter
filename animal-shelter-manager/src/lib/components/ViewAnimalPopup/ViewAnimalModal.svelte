<!-- 
ViewAnimalModal.svelte

Reusable modal popup component to display detailed information
about an animal and its adopter (if applicable).
-->

<script lang="ts">
  import { User, Dog } from "@lucide/svelte";
  import AdopterInfo from "./AdopterInfo/AdopterInfo.svelte";
  import AnimalInfo from "./AnimalInfo/AnimalInfo.svelte";
  import type { Animal } from "../../utils/animal-utils";
  import type { AdoptionRequest } from "../../utils/animal-utils";

  /** Type definition for tab selection in the adoption info popup */
  type Tab = "animal" | "adopter";

  // Props
  interface Props {
    /** The animal to display information about */
    animal: Animal;
    /** The adopter information (optional) */
    adopter?: AdoptionRequest | null;
    /** Controls whether the popup is visible */
    isOpen?: boolean;
    /** Callback function to close the popup */
    onClose?: () => void;
  }

  /** Component props with default values */
  const { animal, adopter, isOpen = false, onClose }: Props = $props();

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
      <div class="tab-content">
        {#if adopter && activeTab === "adopter"}
          <AdopterInfo {adopter} onclose={closePopup} />
        {:else}
          <AnimalInfo
            {animal}
            adoption_timestamp={adopter?.adoptionTimestamp ?? 0}
            onclose={closePopup}
          />
        {/if}
      </div>
    </div>
  </div>

  {#if adopter}
    <div class="tab-container">
      <div class="tab-slider" class:bottom={activeTab === "animal"}></div>
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
{/if}

<style lang="scss">
  @use "./style.scss";
</style>
