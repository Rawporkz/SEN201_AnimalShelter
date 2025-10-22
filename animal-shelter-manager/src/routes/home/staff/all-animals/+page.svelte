<!-- 
all-animals/+page.svelte

This page displays all animals in the shelter system for staff members.
-->

<script lang="ts">
  import SearchBar from "$lib/components/SearchBar/SearchBar.svelte";
  import AnimalInfoRow from "$lib/components/AnimalInfoRow/AnimalInfoRow.svelte";
  import FilterModal from "$lib/components/FilterModal/FilterModal.svelte";
  import ViewAnimalModal from "$lib/components/ViewAnimalPopup/ViewAnimalModal.svelte";
  import {
    FilterCriteria,
    type FilterSelections,
  } from "$lib/components/FilterModal/filter-utils";
  import {
    type AnimalSummary,
    type Animal,
    type AdoptionRequest,
    AnimalStatus,
  } from "$lib/utils/animal-utils";
  import {
    Plus,
    SlidersHorizontal,
    Eye,
    Pencil,
    ClipboardList,
  } from "@lucide/svelte";

  // Mock data
  let animals: AnimalSummary[] = $state([
    {
      id: "120312",
      name: "Bob",
      specie: "Dog",
      breed: "Golden Retriever",
      sex: "Male",
      admission_timestamp: 1728432000,
      status: AnimalStatus.AVAILABLE,
      image_path:
        "https://th.bing.com/th/id/R.9003028d7ec6ff58a70a2be15a05ffed?rik=9cLEFO6s9oiGeg&riu=http%3a%2f%2fwww.publicdomainpictures.net%2fpictures%2f40000%2fvelka%2fgolden-retriever-dog-1364426710r9x.jpg&ehk=Z8ZK9mRUJe0rT61EYByfWPUGg1BEToYpGPK3bCz1aTU%3d&risl=&pid=ImgRaw&r=0",
    },
    {
      id: "542312",
      name: "Besoz",
      specie: "Cat",
      breed: "Foldex",
      sex: "Female",
      admission_timestamp: 1736380800,
      status: AnimalStatus.REQUESTED,
      image_path:
        "https://tse1.mm.bing.net/th/id/OIP.YurrNbVEnySArvYoqFUdXgHaE5?cb=12&rs=1&pid=ImgDetMain&o=7&rm=3",
    },
    {
      id: "123121",
      name: "Jeff",
      specie: "Dog",
      breed: "Beagle",
      sex: "Male",
      admission_timestamp: 1757808000,
      status: AnimalStatus.ADOPTED,
      image_path:
        "https://tse1.mm.bing.net/th/id/OIP.sn4JPFB9pw2PaI3Ao6DWigHaFX?cb=12&rs=1&pid=ImgDetMain&o=7&rm=3",
    },
  ]);

  // Search and filter state
  let searchQuery = $state("");
  let isFilterModalOpen = $state(false);
  let filterSelections: FilterSelections | null = $state(null);

  // View modal state
  let isViewModalOpen = $state(false);
  let selectedAnimal: Animal | null = $state(null);
  let selectedAdopter: AdoptionRequest | null = $state(null);

  // Filter criteria to show in modal
  const filterCriteria = [
    FilterCriteria.STATUS,
    FilterCriteria.SEX,
    FilterCriteria.SPECIES_AND_BREEDS,
    FilterCriteria.ADMISSION_DATE,
  ];

  /**
   * Opens the filter modal
   */
  function handleFilterClick(): void {
    isFilterModalOpen = true;
  }

  /**
   * Handles filter modal close and updates selections
   */
  function handleFilterClose(selections: FilterSelections): void {
    filterSelections = selections;
    isFilterModalOpen = false;
  }

  /**
   * Handles viewing animal details
   */
  function handleViewAnimal(animalSummary: AnimalSummary): void {
    // In real implementation, fetch full animal data
    selectedAnimal = animalSummary as any; // Cast for demo
    selectedAdopter =
      animalSummary.status === "adopted" || animalSummary.status === "requested"
        ? ({} as any)
        : null;
    isViewModalOpen = true;
  }

  /**
   * Handles editing animal details
   */
  function handleEditAnimal(animalId: string): void {
    // Navigate to edit page or open edit modal
    console.log("Edit animal:", animalId);
  }

  /**
   * Handles adoption request management
   */
  function handleManageRequest(animalId: string): void {
    // Navigate to request management or open modal
    console.log("Handle request for:", animalId);
  }

  /**
   * Handles admitting a new animal
   */
  function handleAdmitAnimal(): void {
    // Navigate to admit animal page
    console.log("Admit new animal");
  }

  /**
   * Closes the view modal
   */
  function handleCloseViewModal(): void {
    isViewModalOpen = false;
    selectedAnimal = null;
    selectedAdopter = null;
  }

  // Filtered animals based on search query
  let filteredAnimals = $derived(
    animals.filter((animal) => {
      if (!searchQuery) return true;

      const query = searchQuery.toLowerCase();
      return (
        animal.name.toLowerCase().includes(query) ||
        animal.id.toLowerCase().includes(query) ||
        animal.breed.toLowerCase().includes(query) ||
        animal.specie.toLowerCase().includes(query)
      );
    })
  );
</script>

<div class="all-animals-page">
  <div class="page-header">
    <h1 class="page-title">All Animals</h1>
  </div>

  <div class="controls-bar">
    <SearchBar
      bind:value={searchQuery}
      placeholder="Search for names, IDs, breeds, and more..."
    />

    <button class="filter-button" onclick={handleFilterClick}>
      <SlidersHorizontal size={16} />
      <span>Filters</span>
    </button>

    <button class="admit-button" onclick={handleAdmitAnimal}>
      <Plus size={16} />
      <span>Admit Animal</span>
    </button>
  </div>

  <div class="animals-list">
    {#each filteredAnimals as animal (animal.id)}
      <AnimalInfoRow animalSummary={animal} showStatus={true}>
        {#snippet actions()}
          {#if animal.status === "available"}
            <button
              class="action-button view-button"
              onclick={() => handleViewAnimal(animal)}
            >
              <Eye size={16} />
              <span>View</span>
            </button>
            <button
              class="action-button edit-button"
              onclick={() => handleEditAnimal(animal.id)}
            >
              <Pencil size={16} />
              <span>Edit</span>
            </button>
          {:else if animal.status === "requested"}
            <button
              class="action-button view-button"
              onclick={() => handleViewAnimal(animal)}
            >
              <Eye size={16} />
              <span>View</span>
            </button>
            <button
              class="action-button request-button"
              onclick={() => handleManageRequest(animal.id)}
            >
              <ClipboardList size={16} />
              <span>Handle Request</span>
            </button>
          {:else if animal.status === "adopted"}
            <button
              class="action-button view-button"
              onclick={() => handleViewAnimal(animal)}
            >
              <Eye size={16} />
              <span>View</span>
            </button>
          {/if}
        {/snippet}
      </AnimalInfoRow>
    {/each}
  </div>
</div>

<FilterModal
  isVisible={isFilterModalOpen}
  criteriaList={filterCriteria}
  currentSelections={filterSelections}
  onclose={handleFilterClose}
/>

{#if selectedAnimal}
  <ViewAnimalModal
    animal={selectedAnimal}
    adopter={selectedAdopter}
    isOpen={isViewModalOpen}
    onClose={handleCloseViewModal}
  />
{/if}

<style lang="scss">
  @use "./style.scss";
</style>
