<!-- 
routes/home/customer/my-adoption-requests/+page.svelte

This page displays the user's adoption requests.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { error } from "@tauri-apps/plugin-log";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";
  import { logoutUser } from "$lib/utils/authentication-utils";
  import type { PageData } from "./$types";
  import SearchBar from "$lib/components/SearchBar/SearchBar.svelte";
  import AnimalInfoRow from "$lib/components/AnimalInfoRow/AnimalInfoRow.svelte";
  import ViewAnimalModal from "$lib/components/ViewAnimalPopup/ViewAnimalModal.svelte";
  import ConfirmationModal from "$lib/components/ConfirmationModal/ConfirmationModal.svelte";
  import {
    type AnimalSummary,
    type Animal,
    type AdoptionRequest,
    getAnimalById,
    deleteAdoptionRequest,
  } from "$lib/utils/data-utils";
  import { Eye, X } from "@lucide/svelte";
  import ActionButton from "$lib/components/ActionButton/ActionButton.svelte";
  import NothingToShowIcon from "$lib/components/NothingToShowIcon/NothingToShowIcon.svelte";
  import { navigationMap } from "../navigation-utils";
  import { type MyAdoptionRequest } from "./my-adoption-request-utils";
  import ExpandableStatus from "$lib/components/ExpandableStatus/ExpandableStatus.svelte";
  import { getRequestStatusDisplayText } from "$lib/utils/data-utils";
  import { getRequestStatusColor } from "./status-utils";

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
  /** Whether the view animal modal is open. */
  let isViewModalOpen: boolean = $state(false);
  /** The animal selected for viewing. */
  let selectedAnimal: Animal | null = $state(null);
  /** Whether the sign-out confirmation modal is open. */
  let isSignOutModalOpen = $state(false);
  /** Whether the revoke confirmation modal is open. */
  let isRevokeModalOpen = $state(false);
  /** The adoption request to be revoked. */
  let requestToRevoke: AdoptionRequest | null = $state(null);

  /**
   * Handles the viewing of an animal's details.
   * @param animalSummary - The summary of the animal to view.
   */
  async function handleViewAnimal(animalSummary: AnimalSummary): Promise<void> {
    const animal = await getAnimalById(animalSummary.id);
    selectedAnimal = animal;
    isViewModalOpen = true;
  }

  /**
   * Opens the revoke confirmation modal for the given request.
   * @param request - The adoption request to revoke.
   */
  function handleRevokeRequest(request: AdoptionRequest): void {
    requestToRevoke = request;
    isRevokeModalOpen = true;
  }

  /**
   * Confirms and executes the revocation of an adoption request.
   */
  async function confirmRevokeRequest(): Promise<void> {
    if (!requestToRevoke) return;

    try {
      const deleted = await deleteAdoptionRequest(requestToRevoke.id);
      if (deleted) {
        displayedRequests = displayedRequests.filter(
          (r) => r.request.id !== requestToRevoke!.id,
        );
      }
    } catch (e) {
      error(`Failed to revoke request: ${e}`);
    } finally {
      requestToRevoke = null;
      isRevokeModalOpen = false;
    }
  }

  /**
   * Closes the view animal modal.
   */
  function handleCloseViewModal(): void {
    isViewModalOpen = false;
    selectedAnimal = null;
  }

  /** The list of adoption requests to be displayed. */
  let displayedRequests: MyAdoptionRequest[] = $state(
    data.myAdoptionRequests || [],
  );

  /** The derived list of adoption requests filtered by the search query. */
  let filteredRequests = $derived(
    displayedRequests.filter(({ animal }) => {
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
      <h1 class="page-title">My Adoption Requests</h1>
    </div>
    <div class="controls-bar">
      <SearchBar
        bind:value={searchQuery}
        placeholder="Search for names, IDs, breeds, and more..."
      />
    </div>

    <div class="animals-list">
      {#if filteredRequests.length > 0}
        {#each filteredRequests as { animal, request } (request.id)}
          <AnimalInfoRow animalSummary={animal}>
            {#snippet status()}
              <ExpandableStatus
                text={getRequestStatusDisplayText(request.status)}
                color={getRequestStatusColor(request.status)}
              />
            {/snippet}
            {#snippet actions()}
              <ActionButton
                label="View"
                icon={Eye}
                width="155px"
                onclick={() => handleViewAnimal(animal)}
              />
              {#if request.status === "pending"}
                <ActionButton
                  label="Revoke"
                  icon={X}
                  width="155px"
                  onclick={() => handleRevokeRequest(request)}
                />
              {/if}
            {/snippet}
          </AnimalInfoRow>
        {/each}
      {:else}
        <NothingToShowIcon />
      {/if}
    </div>
  </main>
</div>

{#if selectedAnimal}
  <ViewAnimalModal
    animal={selectedAnimal}
    adopter={null}
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
  bind:open={isRevokeModalOpen}
  title="Confirm Revoke"
  message="Are you sure you want to revoke this adoption request?"
  confirmText="Revoke"
  cancelText="Cancel"
  destructive={true}
  onconfirm={confirmRevokeRequest}
/>

<style lang="scss">
  @use "./style.scss";
</style>
