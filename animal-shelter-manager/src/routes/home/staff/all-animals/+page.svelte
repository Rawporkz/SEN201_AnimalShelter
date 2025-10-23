<!-- 
all-animals/+page.svelte

This page displays all animals in the shelter system for staff members.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { error } from "@tauri-apps/plugin-log";
  import { info } from "@tauri-apps/plugin-log";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";
  import { logoutUser } from "$lib/utils/authentication-utils";
  import type { PageData } from "./$types";
  import SearchBar from "$lib/components/SearchBar/SearchBar.svelte";
  import AnimalInfoRow from "$lib/components/AnimalInfoRow/AnimalInfoRow.svelte";
  import FilterModal from "$lib/components/FilterModal/FilterModal.svelte";
  import ViewAnimalModal from "$lib/components/ViewAnimalPopup/ViewAnimalModal.svelte";
  import {
    FilterCriteria,
    type FilterSelections,
  } from "$lib/utils/filter-utils";
  import {
    type AnimalSummary,
    type Animal,
    type AdoptionRequest,
    getAnimals,
    getAnimalById,
    getAdoptionRequests,
    getAdoptionRequestById,
  } from "$lib/utils/animal-utils";
  import {
    Plus,
    SlidersHorizontal,
    Eye,
    Pencil,
    ClipboardList,
  } from "@lucide/svelte";
  import { navigationMap } from "../navigation-utils";

  // Props
  interface Props {
    /** The data passed to the page component. */
    data: PageData;
  }

  const { data }: Props = $props();

  /**
   * Handles navigation when a sidebar item is clicked.
   * Navigates to the corresponding route based on the navigation mapping.
   *
   * @param item - The navigation item that was clicked.
   */
  function handleNavigation(item: string): void {
    const route: string | undefined = navigationMap[item];
    if (route) {
      goto(route);
    }
  }

  /**
   * Handles user sign out process.
   * Logs out the user and redirects to the authentication page.
   */
  async function handleSignOut(): Promise<void> {
    try {
      const logoutSuccess: boolean = await logoutUser();
      if (logoutSuccess) {
        goto("/authentication");
      }
    } catch (err) {
      error(`Sign out failed: ${err}`);
    }
  }

  /** The current search query entered by the user. */
  let searchQuery: string = $state("");
  /** Controls the visibility of the filter modal. */
  let isFilterModalOpen: boolean = $state(false);
  /** Stores the current filter selections made by the user. */
  let filterSelections: FilterSelections | null = $state(null);

  /** Controls the visibility of the animal view modal. */
  let isViewModalOpen: boolean = $state(false);
  /** The animal currently selected for viewing in the modal. */
  let selectedAnimal: Animal | null = $state(null);
  /** The adopter information associated with the selected animal, if applicable. */
  let selectedAdopter: AdoptionRequest | null = $state(null);

  /** List of filter criteria to display in the filter modal. */
  const filterCriteria = [
    FilterCriteria.STATUS,
    FilterCriteria.SEX,
    FilterCriteria.SPECIES_AND_BREEDS,
    FilterCriteria.ADMISSION_DATE,
  ];

  /**
   * Opens the filter modal.
   */
  function handleFilterClick(): void {
    isFilterModalOpen = true;
  }

  /**
   * Handles filter modal close and updates selections.
   *
   * @param selections - The filter selections made by the user.
   */
  async function handleFilterClose(
    selections: FilterSelections,
  ): Promise<void> {
    filterSelections = selections;
    info("Filter selections:" + filterSelections);
    isFilterModalOpen = false;
    displayedAnimals = await getAnimals(filterSelections);
  }

  /**
   * Handles viewing animal details.
   *
   * @param animalSummary - The summary of the animal to view.
   */
  async function handleViewAnimal(animalSummary: AnimalSummary): Promise<void> {
    selectedAnimal = await getAnimalById(animalSummary.id);

    if (
      animalSummary.status === "adopted" ||
      animalSummary.status === "requested"
    ) {
      const allRequests = await getAdoptionRequests({
        animal_id: animalSummary.id,
      });
      const requestSummary = allRequests.find(
        (req) => req.animal_id === animalSummary.id,
      );
      if (requestSummary) {
        selectedAdopter = await getAdoptionRequestById(requestSummary.id);
      } else {
        selectedAdopter = null;
      }
    } else {
      selectedAdopter = null;
    }

    isViewModalOpen = true;
  }

  /**
   * Handles editing animal details.
   *
   * @param animalId - The ID of the animal to edit.
   */
  function handleEditAnimal(animalId: string): void {
    // Navigate to edit page or open edit modal
    info("Edit animal:" + animalId);
  }

  /**
   * Handles adoption request management.
   *
   * @param animalId - The ID of the animal for which to manage requests.
   */
  function handleManageRequest(animalId: string): void {
    // Navigate to request management or open modal
    info("Handle request for:" + animalId);
  }

  /**
   * Handles admitting a new animal
   */
  function handleAdmitAnimal(): void {
    // Navigate to admit animal page
    goto("/admit-animal-form");
  }

  /**
   * Closes the view modal
   */
  function handleCloseViewModal(): void {
    isViewModalOpen = false;
    selectedAnimal = null;
    selectedAdopter = null;
  }

  /** Store of animals to be displayed. */
  let displayedAnimals: AnimalSummary[] = $state(data.animals || []);

  /** Derived store of animals filtered based on search query and filter selections. */
  let filteredAnimals = $derived(
    (displayedAnimals || []).filter((animal) => {
      if (!searchQuery) return true;

      const query = searchQuery.toLowerCase();
      return (
        animal.name.toLowerCase().includes(query) ||
        animal.id.toLowerCase().includes(query) ||
        animal.breed.toLowerCase().includes(query) ||
        animal.specie.toLowerCase().includes(query)
      );
    }),
  );
</script>

<div class="staff-layout">
  <div class="sidebar-container">
    <SideBar
      username={data.currentUser?.username ?? "Staff User"}
      role="Staff"
      onNavigate={handleNavigation}
      onSignOut={handleSignOut}
    />
  </div>

  <main class="main-content">
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
  </main>
</div>

<FilterModal
  isVisible={isFilterModalOpen}
  criteriaList={filterCriteria}
  currentSelections={filterSelections}
  onClose={(selections) => handleFilterClose(selections)}
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
