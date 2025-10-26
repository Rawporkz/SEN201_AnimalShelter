<!-- 
routes/home/staff/adoption-requests/+page.svelte

This page displays all adoption requests for staff members to review and manage.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { error, info } from "@tauri-apps/plugin-log";
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
    Funnel,
    Eye,
    FileCheck,
    CheckIcon,
    X,
    TriangleAlert,
  } from "@lucide/svelte";
  import ActionButton from "$lib/components/ActionButton/ActionButton.svelte";
  import { navigationMap } from "../navigation-utils";
  import {
    type AnimalAdoptionRequests,
    approveRequest,
    getAdoptionRequests,
    rejectRequest,
  } from "./adoption-requests-utils";
  import ActionDropdownButton from "$lib/components/ActionDropdownButton/ActionDropdownButton.svelte";
  import NothingToShowIcon from "$lib/components/NothingToShowIcon/NothingToShowIcon.svelte";

  // Props
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
  /** Controls the visibility of the approve confirmation modal. */
  let isApproveModalOpen = $state(false);
  /** Controls the visibility of the reject confirmation modal. */
  let isRejectModalOpen = $state(false);
  /** The adoption request to be handled (approved or rejected). */
  let requestToHandle: AdoptionRequest | null = $state(null);

  /** List of filter criteria to display in the filter modal. */
  const filterCriteria = [
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

  /** Store of adoption requests to be displayed. */
  let displayedRequests: AnimalAdoptionRequests[] = $state(
    data.adoptionRequests || [],
  );

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

    // Fetch adoption reports with the new filters applied
    displayedRequests = await getAdoptionRequests(filterSelections);
  }

  /**
   * Handles viewing animal and adopter details.
   *
   * @param animalSummary - The summary of the animal.
   * @param request - The summary of the adoption request.
   */
  async function handleViewRequest(
    animalSummary: AnimalSummary,
    request: AdoptionRequest,
  ): Promise<void> {
    const animal: Animal | null = await getAnimalById(animalSummary.id);
    if (!animal) {
      error(`Animal with ID ${animalSummary.id} not found.`);
      return;
    }
    selectedAnimal = animal;
    selectedAdopter = request;
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

  /**
   * Opens the approve confirmation modal for the given request.
   * @param request - The adoption request to approve.
   */
  function handleApproveRequest(request: AdoptionRequest): void {
    requestToHandle = request;
    isApproveModalOpen = true;
  }

  /**
   * Opens the reject confirmation modal for the given request.
   * @param request - The adoption request to reject.
   */
  function handleRejectRequest(request: AdoptionRequest): void {
    requestToHandle = request;
    isRejectModalOpen = true;
  }

  /**
   * Confirms and executes the approval of an adoption request.
   */
  async function confirmApproveRequest(): Promise<void> {
    if (!requestToHandle) return;
    const animalId = requestToHandle.animalId;
    try {
      info(`Approving request ID: ${requestToHandle.id}`);
      await approveRequest(requestToHandle);

      // Remove the approved request and all other requests for the same animal from the displayed list
      displayedRequests = displayedRequests.filter(
        (r) => r.animal.id !== animalId,
      );
    } catch (e) {
      error(`Failed to approve request: ${e}`);
    } finally {
      requestToHandle = null;
      isApproveModalOpen = false;
    }
  }

  /**
   * Confirms and executes the rejection of an adoption request.
   */
  async function confirmRejectRequest(): Promise<void> {
    if (!requestToHandle) return;
    const requestId = requestToHandle.id;
    try {
      info(`Rejecting request ID: ${requestToHandle.id}`);
      await rejectRequest(requestToHandle);

      // Remove the rejected request from the displayed list
      displayedRequests = displayedRequests.filter(
        (r) => r.request.id !== requestId,
      );
    } catch (e) {
      error(`Failed to reject request: ${e}`);
    } finally {
      requestToHandle = null;
      isRejectModalOpen = false;
    }
  }

  /** Derived store of adoption requests filtered based on search query. */
  let filteredRequests = $derived(
    displayedRequests.filter(({ animal, request }) => {
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
      navItems={Object.keys(navigationMap)}
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
      <ActionButton
        label="Filters"
        icon={Funnel}
        width="110px"
        onclick={handleFilterClick}
      />
    </div>
    <div class="requests-list">
      {#if filteredRequests.length > 0}
        {#each filteredRequests as { animal, request } (animal.id)}
          <AnimalAdoptionInfoRow
            animalSummary={animal}
            adoptionRequest={request}
          >
            {#snippet actions()}
              <ActionButton
                label="View"
                icon={Eye}
                width="155px"
                onclick={() => handleViewRequest(animal, request)}
              />
              <ActionDropdownButton
                label="Handle"
                width="155px"
                icon={FileCheck}
                options={[
                  {
                    label: "Approve",
                    icon: CheckIcon,
                    onclick: () => handleApproveRequest(request),
                  },
                  {
                    label: "Reject",
                    icon: X,
                    onclick: () => handleRejectRequest(request),
                  },
                ]}
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

<ConfirmationModal
  bind:open={isApproveModalOpen}
  title="Approving a Request"
  message={`Are you sure you want to approve this adoption request by ${requestToHandle?.name}?`}
  confirmText="Approve"
  cancelText="Cancel"
  onconfirm={confirmApproveRequest}
>
  {#snippet extra()}
    <div class="warning-box">
      <TriangleAlert size={50} />
      <span>This will reject all other requests for this animal.</span>
    </div>
  {/snippet}
</ConfirmationModal>

<ConfirmationModal
  bind:open={isRejectModalOpen}
  title="Confirm Rejection"
  message="Are you sure you want to reject this adoption request?"
  confirmText="Reject"
  cancelText="Cancel"
  destructive={true}
  onconfirm={confirmRejectRequest}
/>

<style lang="scss">
  @use "./style.scss";
</style>
