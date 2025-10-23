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
   * @param item - The navigation item that was clicked
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

  // Mock data for adoption requests
  interface AdoptionRequestData {
    animal: AnimalSummary;
    request: AdoptionRequest;
  }

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
   * Handles viewing animal and adopter details
   */
  function handleViewRequest(requestData: AdoptionRequestData): void {
    selectedAnimal = requestData.animal as any;
    selectedAdopter = requestData.request;
    isViewModalOpen = true;
  }

  /**
   * Handles the main action for the request (approve, reject, etc.)
   */
  function handleManageRequest(requestData: AdoptionRequestData): void {
    info("Handle request for: " + requestData.animal.id);
    // Navigate to request management or open action modal
  }

  /**
   * Handles approving a request
   */
  function handleApprove(requestData: AdoptionRequestData): void {
    info("Approve request: " + requestData.request.id);
  }

  /**
   * Handles rejecting a request
   */
  function handleReject(requestData: AdoptionRequestData): void {
    info("Reject request: " + requestData.request.id);
  }

  /**
   * Closes the view modal
   */
  function handleCloseViewModal(): void {
    isViewModalOpen = false;
    selectedAnimal = null;
    selectedAdopter = null;
  }

  // Filtered requests based on search query
  let filteredRequests = $derived(
    (data.adoptionRequests || []).filter((requestData) => {
      if (!searchQuery) return true;

      const query = searchQuery.toLowerCase();
      const animal = requestData.animal;
      const request = requestData.request;

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
      {#each filteredRequests as requestData (requestData.animal.id)}
        <AnimalAdoptionInfoRow
          animalSummary={requestData.animal}
          adoptionRequest={requestData.request}
        >
          {#snippet actions()}
            <button
              class="action-button view-button"
              onclick={() => handleViewRequest(requestData)}
            >
              <Eye size={16} />
              <span>View</span>
            </button>
            <button
              class="action-button handle-button"
              onclick={() => handleManageRequest(requestData)}
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
