<!-- 
lib/components/AdoptionInfoPopup/AdoptionInfoPopup.svelte

This file defines a reusable AdoptionInfoPopup component.
-->
<script lang="ts">
  import "./style.scss";
  import { User, Dog } from "@lucide/svelte";
  import AdopterInfo from "./AdopterInfo/AdopterInfo.svelte";
  import AnimalInfo from "./AnimalInfo/AnimalInfo.svelte";

  type Tab = "animal" | "adopter";

  let activeTab: Tab = "animal";

  export let isOpen = false;

  function selectTab(tab: Tab) {
    activeTab = tab;
  }

  function closePopup() {
    isOpen = false;
  }
</script>

{#if isOpen}
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
