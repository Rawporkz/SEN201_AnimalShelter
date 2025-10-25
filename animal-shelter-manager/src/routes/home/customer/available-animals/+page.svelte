<!-- 
routes/home/customer/available-animals/+page.svelte

This page displays available animals to customers.
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
  import ConfirmationModal from "$lib/components/ConfirmationModal/ConfirmationModal.svelte";
  import {
    FilterCriteria,
    type FilterSelections,
  } from "$lib/utils/filter-utils";
  import {
    type AnimalSummary,
    type Animal,
    type AdoptionRequest,
    getAnimals,
    getAnimalWithAcceptedAdoption,
  } from "$lib/utils/data-utils";
  import { Eye, Send, Funnel } from "@lucide/svelte";
  import ActionButton from "$lib/components/ActionButton/ActionButton.svelte";
  import NothingToShowIcon from "$lib/components/NothingToShowIcon/NothingToShowIcon.svelte";
  import { navigationMap } from "../navigation-utils";

  // Props
  interface Props {
    /** The data passed to the page from the load function. */
    data: PageData;
  }

  const { data }: Props = $props();

  /**
   * Handles navigation when a sidebar item is clicked.
   * @param item - The navigation item that was clicked.
   */
  function handleNavigation(item: string): void {
    const route: string | undefined = navigationMap[item];
    if (route) {
      goto(route);
    }
  }

  /**
   * Opens the sign-out confirmation modal.
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

  /** The current search query. */
  let searchQuery: string = $state("");
  /** Whether the filter modal is open. */
  let isFilterModalOpen: boolean = $state(false);
  /** The current filter selections. */
  let filterSelections: FilterSelections | null = $state(null);
  /** Whether the view animal modal is open. */
  let isViewModalOpen: boolean = $state(false);
  /** The animal selected for viewing. */
  let selectedAnimal: Animal | null = $state(null);
  /** The adopter information for the selected animal. */
  let selectedAdopter: AdoptionRequest | null = $state(null);
  /** Whether the sign-out confirmation modal is open. */
  let isSignOutModalOpen = $state(false);

  /** The filter criteria to be displayed in the filter modal. */
  const filterCriteria = [
    FilterCriteria.SEX,
    FilterCriteria.SPECIES_AND_BREEDS,
  ];

  /**
   * Opens the filter modal.
   */
  function handleFilterClick(): void {
    isFilterModalOpen = true;
  }

  /**
   * Handles the closing of the filter modal and applies the selected filters.
   * @param selections - The filter selections from the modal.
   */
  async function handleFilterClose(
    selections: FilterSelections,
  ): Promise<void> {
    filterSelections = selections;
    info("Filter selections:" + JSON.stringify(filterSelections));
    isFilterModalOpen = false;
    displayedAnimals = await getAnimals({
      ...filterSelections,
      status: ["available", "requested"],
    });
  }

  /**
   * Handles the viewing of an animal's details.
   * @param animalSummary - The summary of the animal to view.
   */
  async function handleViewAnimal(animalSummary: AnimalSummary): Promise<void> {
    const { animal, adopter } = await getAnimalWithAcceptedAdoption(
      animalSummary.id,
      animalSummary.status,
    );
    selectedAnimal = animal;
    selectedAdopter = adopter;
    isViewModalOpen = true;
  }

  /**
   * Navigates to the send adoption request form for the given animal.
   * @param animalId - The ID of the animal to send a request for.
   */
  function handleSendRequest(animalId: string): void {
    goto(`/send-adoption-request-form/${animalId}`);
  }

  /**
   * Closes the view animal modal.
   */
  function handleCloseViewModal(): void {
    isViewModalOpen = false;
    selectedAnimal = null;
    selectedAdopter = null;
  }

  /** The list of animals to be displayed. */
  let displayedAnimals: AnimalSummary[] = $state(data.animals || []);

  /** The derived list of animals filtered by the search query. */
  let filteredAnimals = $derived(
    displayedAnimals.filter((animal) => {
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
      username={data.currentUser?.username ?? "Customer User"}
      role="Customer"
      navItems={Object.keys(navigationMap)}
      onNavigate={handleNavigation}
      onSignOut={handleSignOut}
    />
  </div>

  <main class="main-content">
    <div class="page-header">
      <h1 class="page-title">Available Animals</h1>
    </div>
    <div class="controls-bar">
      <SearchBar
        bind:value={searchQuery}
        placeholder="Search for names, IDs, breeds, and more..."
      />
      <ActionButton
        label="Filter"
        icon={Funnel}
        width="110px"
        onclick={handleFilterClick}
      ></ActionButton>
    </div>

    <div class="animals-list">
      {#if filteredAnimals.length > 0}
        {#each filteredAnimals as animal (animal.id)}
          <AnimalInfoRow animalSummary={animal} showStatus={false}>
            {#snippet actions()}
              <ActionButton
                label="View"
                icon={Eye}
                width="155px"
                onclick={() => handleViewAnimal(animal)}
              />
              <ActionButton
                label="Send Request"
                icon={Send}
                width="155px"
                onclick={() => handleSendRequest(animal.id)}
              />
            {/snippet}
          </AnimalInfoRow>
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
