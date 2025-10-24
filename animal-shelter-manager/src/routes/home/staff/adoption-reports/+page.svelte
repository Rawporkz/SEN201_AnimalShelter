<!-- 
routes/home/staff/adoption-reports/+page.svelte

This page displays all adoption reports for staff members to review.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { info, error } from "@tauri-apps/plugin-log";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";
  import { logoutUser } from "$lib/utils/authentication-utils";
  import type { PageData } from "./$types";
  import SearchBar from "$lib/components/SearchBar/SearchBar.svelte";
  import AnimalAdoptionInfoRow from "$lib/components/AnimalAdoptionInfoRow/AnimalAdoptionInfoRow.svelte";
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
    getAnimalById,
    getAdoptionRequestById,
  } from "$lib/utils/animal-utils";
  import {
    type AnimalAdoptionReport,
    get_adoption_reports,
  } from "./adoption-reports-utils";
  import { SlidersHorizontal, Eye } from "@lucide/svelte";
  import { navigationMap } from "../navigation-utils";

  interface Props {
    data: PageData;
  }

  const { data }: Props = $props();

  /** The current search query entered by the user. */
  let searchQuery = $state("");
  /** Controls the visibility of the filter modal. */
  let isFilterModalOpen = $state(false);
  /** Stores the current filter selections made by the user. */
  let filterSelections: FilterSelections | null = $state(null);

  /** Controls the visibility of the animal view modal. */
  let isViewModalOpen = $state(false);
  /** The animal currently selected for viewing in the modal. */
  let selectedAnimal: Animal | null = $state(null);
  /** The adopter information associated with the selected animal, if applicable. */
  let selectedAdopter: AdoptionRequest | null = $state(null);

  /** Store of adoption requests to be displayed. */
  let displayedRequests: AnimalAdoptionReport[] = $state(
    data.adoptionRequests || [],
  );

  /** List of filter criteria to display in the filter modal. */
  const filterCriteria = [
    FilterCriteria.SEX,
    FilterCriteria.SPECIES_AND_BREEDS,
    FilterCriteria.ADMISSION_DATE,
    FilterCriteria.ADOPTION_DATE,
  ];

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

  /**
   * Opens the filter modal.
   */
  function handleFilterClick(): void {
    isFilterModalOpen = true;
  }

  /**
   * Handles filter modal close and updates selections.
   * @param filterSelections - The filter selections made by the user.
   */
  async function handleFilterClose(
    filterSelections: FilterSelections,
  ): Promise<void> {
    info("Filter selections:" + filterSelections);
    filterSelections = filterSelections;
    isFilterModalOpen = false;

    // Fetch adoption reports without filters initially
    const adoptionRequests: AnimalAdoptionReport[] =
      await get_adoption_reports(filterSelections);

    displayedRequests = adoptionRequests;
  }

  /**
   * Handles viewing animal and adopter details.
   *
   * @param animalSummary - The summary of the animal.
   * @param requestSummary - The summary of the adoption request.
   */
  async function handleViewRequest(
    animalSummary: AnimalSummary,
    requestSummary: AdoptionRequest,
  ): Promise<void> {
    const [animal, adopter] = await Promise.all([
      getAnimalById(animalSummary.id),
      getAdoptionRequestById(requestSummary.id),
    ]);
    selectedAnimal = animal;
    selectedAdopter = adopter;
    isViewModalOpen = true;
  }

  /**
   * Closes the view modal.
   */
  function handleCloseViewModal(): void {
    isViewModalOpen = false;
    selectedAnimal = null;
    selectedAdopter = null;
  }

  /** Derived store of adoption requests filtered based on search query. */
  let searchedRequests = $derived(
    (displayedRequests || []).filter(({ animal, adoption }) => {
      if (!searchQuery) return true;

      const query = searchQuery.toLowerCase();

      return (
        animal.name.toLowerCase().includes(query) ||
        animal.id.toLowerCase().includes(query) ||
        animal.breed.toLowerCase().includes(query) ||
        animal.specie.toLowerCase().includes(query) ||
        adoption.name.toLowerCase().includes(query) ||
        adoption.email.toLowerCase().includes(query)
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
      <h1 class="page-title">Adoption Reports</h1>
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
    </div>
    <div class="report-list">
      {#each searchedRequests as { animal, adoption } (animal.id)}
        <AnimalAdoptionInfoRow
          animalSummary={animal}
          adoptionRequest={adoption}
        >
          {#snippet actions()}
            <button
              class="action-button view-button"
              onclick={() => handleViewRequest(animal, adoption)}
            >
              <Eye size={16} />
              <span>View</span>
            </button>
          {/snippet}
        </AnimalAdoptionInfoRow>
      {/each}
    </div>
  </main>
</div>
<FilterModal
  isVisible={isFilterModalOpen}
  criteriaList={filterCriteria}
  currentSelections={filterSelections}
  onClose={handleFilterClose}
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
