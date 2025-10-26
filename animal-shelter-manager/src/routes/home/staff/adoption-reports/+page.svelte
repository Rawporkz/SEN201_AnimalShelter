<!-- 
routes/home/staff/adoption-reports/+page.svelte

This page displays all adoption reports for staff members to review.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { error } from "@tauri-apps/plugin-log";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";
  import { logoutUser } from "$lib/utils/authentication-utils";
  import type { PageData } from "./$types";
  import SearchBar from "$lib/components/SearchBar/SearchBar.svelte";
  import AnimalAdoptionInfoRow from "$lib/components/AnimalAdoptionInfoRow/AnimalAdoptionInfoRow.svelte";
  import FilterModal from "$lib/components/FilterModal/FilterModal.svelte";
  import ViewAnimalModal from "$lib/components/ViewAnimalPopup/ViewAnimalModal.svelte";
  import ConfirmationModal from "$lib/components/ConfirmationModal/ConfirmationModal.svelte";
  import {
    FilterCriteria,
    type FilterSelections,
  } from "$lib/utils/filter-utils";
  import {
    type AnimalSummary,
    type Animal,
    type AdoptionRequest,
    getAnimalById,
  } from "$lib/utils/data-utils";
  import {
    type AnimalAdoptionReport,
    getAdoptionReports,
  } from "./adoption-reports-utils";
  import { Funnel, Eye } from "@lucide/svelte";
  import ActionButton from "$lib/components/ActionButton/ActionButton.svelte";
  import NothingToShowIcon from "$lib/components/NothingToShowIcon/NothingToShowIcon.svelte";
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
  /** Controls the visibility of the sign-out confirmation modal. */
  let isSignOutModalOpen = $state(false);

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
  function handleSignOut(): void {
    isSignOutModalOpen = true;
  }

  /**
   * Confirms and executes the sign-out process.
   */
  async function confirmSignOut(): Promise<void> {
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
    filterSelections = filterSelections;
    isFilterModalOpen = false;

    // Fetch adoption reports without filters initially
    const adoptionRequests: AnimalAdoptionReport[] =
      await getAdoptionReports(filterSelections);

    displayedRequests = adoptionRequests;
  }

  /**
   * Handles viewing animal and adopter details.
   *
   * @param animalSummary - The summary of the animal.
   * @param adoption - The summary of the adoption request.
   */
  async function handleViewRequest(
    animalSummary: AnimalSummary,
    adoption: AdoptionRequest,
  ): Promise<void> {
    const animal: Animal | null = await getAnimalById(animalSummary.id);
    if (!animal) {
      error(`Animal with ID ${animalSummary.id} not found.`);
      return;
    }
    selectedAnimal = animal;
    selectedAdopter = adoption;
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
    displayedRequests.filter(({ animal, adoption }) => {
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
      navItems={Object.keys(navigationMap)}
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
      <ActionButton
        label="Filters"
        icon={Funnel}
        width="110px"
        onclick={handleFilterClick}
      />
    </div>
    <div class="report-list">
      {#if searchedRequests.length > 0}
        {#each searchedRequests as { animal, adoption } (animal.id)}
          <AnimalAdoptionInfoRow
            animalSummary={animal}
            adoptionRequest={adoption}
          >
            {#snippet actions()}
              <ActionButton
                label="View"
                icon={Eye}
                width="155px"
                onclick={() => handleViewRequest(animal, adoption)}
              />
            {/snippet}
          </AnimalAdoptionInfoRow>
        {/each}
      {:else}
        <NothingToShowIcon />
      {/if}
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

<ConfirmationModal
  bind:open={isSignOutModalOpen}
  title="Confirm Sign Out"
  message="Are you sure you want to sign out?"
  confirmText="Sign Out"
  cancelText="Cancel"
  destructive={true}
  onconfirm={confirmSignOut}
/>

<style lang="scss">
  @use "./style.scss";
</style>
