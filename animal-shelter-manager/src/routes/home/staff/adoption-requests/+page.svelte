<!-- 
adoption-requests/+page.svelte

This page displays all adoption requests for staff members to review and manage.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { error } from "@tauri-apps/plugin-log";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";
  import { logoutUser } from "$lib/utils/authentication-utils";
  import type { PageData } from "./$types";
  import { info } from "@tauri-apps/plugin-log";
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
    getAdoptionRequests,
    getAnimalById,
    getAdoptionRequestById,
    getAnimals,
    AnimalStatus,
    RequestStatus,
  } from "$lib/utils/animal-utils";
  import { SlidersHorizontal, Eye, UserCheck } from "@lucide/svelte";
  import { navigationMap } from "../navigation-utils";

  interface Props {
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

  /** List of filter criteria to display in the filter modal. */
  const filterCriteria = [
    FilterCriteria.SEX,
    FilterCriteria.SPECIES_AND_BREEDS,
    FilterCriteria.ADMISSION_DATE,
    FilterCriteria.REQUEST_DATE,
  ];

  /**
   * Opens the filter modal.
   */
  function handleFilterClick(): void {
    isFilterModalOpen = true;
  }

  /** Store of adoption requests to be displayed. */
  let displayedRequests: { animal: Animal; request: AdoptionRequestSummary }[] = $state(data.adoptionRequests || []);

  /**
   * Handles filter modal close and updates selections.
   * @param selections - The filter selections made by the user.
   */
  async function handleFilterClose(
    selections: FilterSelections,
  ): Promise<void> {
    filterSelections = selections;
    info("Filter selections:" + filterSelections);
    isFilterModalOpen = false;

    const requestSummaries = await getAdoptionRequests({
      ...filterSelections,
      status: [RequestStatus.PENDING],
    });
    const animalSummaries = await getAnimals({
      ...filterSelections,
      status: [AnimalStatus.REQUESTED],
    });

    displayedRequests = requestSummaries
      .map((request) => {
        const animal = animalSummaries.find((a) => a.id === request.animal_id);
        return animal ? { animal, request } : null;
      })
      .filter(Boolean) as { animal: Animal; request: AdoptionRequestSummary }[];
  }

  /**
   * Handles viewing animal and adopter details.
   *
   * @param animalSummary - The summary of the animal.
   * @param requestSummary - The summary of the adoption request.
   */
  async function handleViewRequest(animalSummary: AnimalSummary, requestSummary: AdoptionRequestSummary): Promise<void> {
    selectedAnimal = await getAnimalById(animalSummary.id);
    selectedAdopter = await getAdoptionRequestById(requestSummary.id);
    isViewModalOpen = true;
  }

  /**
   * Handles the main action for the request (approve, reject, etc.).
   *
   * @param animalSummary - The summary of the animal.
   * @param requestSummary - The summary of the adoption request.
   */
  function handleManageRequest(animalSummary: AnimalSummary, requestSummary: AdoptionRequestSummary): void {
    info("Handle request for: " + animalSummary.id);
    // Navigate to request management or open action modal
  }

  /**
   * Handles approving a request.
   *
   * @param animalSummary - The summary of the animal.
   * @param requestSummary - The summary of the adoption request to approve.
   */
  function handleApprove(animalSummary: AnimalSummary, requestSummary: AdoptionRequestSummary): void {
    info("Approve request: " + requestSummary.id);
  }

  /**
   * Handles rejecting a request.
   *
   * @param animalSummary - The summary of the animal.
   * @param requestSummary - The summary of the adoption request to reject.
   */
  function handleReject(animalSummary: AnimalSummary, requestSummary: AdoptionRequestSummary): void {
    info("Reject request: " + requestSummary.id);
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
  let filteredRequests = $derived(
    (displayedRequests || []).filter(({ animal, request }) => {
      if (!searchQuery) return true;

      const query = searchQuery.toLowerCase();

      return (
        animal.name.toLowerCase().includes(query) ||
        animal.id.toLowerCase().includes(query) ||
        animal.breed.toLowerCase().includes(query) ||
        animal.specie.toLowerCase().includes(query) ||
        request.name.toLowerCase().includes(query) ||
        request.email.toLowerCase().includes(query)
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
      <h1 class="page-title">Adoption Requests</h1>
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

    <div class="requests-list">
      {#each filteredRequests as { animal, request } (animal.id)}
        <AnimalAdoptionInfoRow
          animalSummary={animal}
          adoptionRequest={request}
        >
          {#snippet actions()}
            <button
              class="action-button view-button"
              onclick={() => handleViewRequest(animal, request)}
            >
              <Eye size={16} />
              <span>View</span>
            </button>
            <button
              class="action-button handle-button"
              onclick={() => handleManageRequest(animal, request)}
            >
              <UserCheck size={16} />
              <span>Handle</span>
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
