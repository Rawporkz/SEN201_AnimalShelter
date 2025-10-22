<!-- 
adoption-requests/+page.svelte

This page displays all adoption requests for staff members to review and manage.
-->

<script lang="ts">
  import SearchBar from "$lib/components/SearchBar/SearchBar.svelte";
  import AnimalAdoptionInfoRow from "$lib/components/AnimalAdoptionInfoRow/AnimalAdoptionInfoRow.svelte";
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
    RequestStatus,
  } from "$lib/utils/animal-utils";
  import { SlidersHorizontal, Eye, UserCheck } from "@lucide/svelte";

  // Mock data for adoption requests
  interface AdoptionRequestData {
    animal: AnimalSummary;
    request: AdoptionRequest;
  }

  let adoptionRequests: AdoptionRequestData[] = $state([
    {
      animal: {
        id: "120312",
        name: "Bob",
        specie: "Dog",
        breed: "Golden Retriever",
        sex: "Male",
        admission_timestamp: 1728432000,
        status: AnimalStatus.REQUESTED,
        image_path:
          "https://th.bing.com/th/id/R.9003028d7ec6ff58a70a2be15a05ffed?rik=9cLEFO6s9oiGeg&riu=http%3a%2f%2fwww.publicdomainpictures.net%2fpictures%2f40000%2fvelka%2fgolden-retriever-dog-1364426710r9x.jpg&ehk=Z8ZK9mRUJe0rT61EYByfWPUGg1BEToYpGPK3bCz1aTU%3d&risl=&pid=ImgRaw&r=0",
      },
      request: {
        id: "req-001",
        animal_id: "120392",
        name: "Non",
        email: "non@gmail.com",
        adoption_timestamp: 1736380800,
        status: RequestStatus.PENDING,
      } as AdoptionRequest,
    },
    {
      animal: {
        id: "542312",
        name: "Mr.Butters",
        specie: "Cat",
        breed: "Foldex",
        sex: "Female",
        admission_timestamp: 1736380800,
        status: AnimalStatus.REQUESTED,
        image_path:
          "https://tse1.mm.bing.net/th/id/OIP.YurrNbVEnySArvYoqFUdXgHaE5?cb=12&rs=1&pid=ImgDetMain&o=7&rm=3",
      },
      request: {
        id: "req-002",
        animal_id: "542312",
        name: "Jira",
        email: "amarin@gmail.com",
        adoption_timestamp: 1736380800,
        status: RequestStatus.PENDING,
      } as AdoptionRequest,
    },
    {
      animal: {
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
      request: {
        id: "req-003",
        animal_id: "123121",
        name: "Win",
        email: "someone@gmail.com",
        adoption_timestamp: 1736380800,
        status: RequestStatus.APPROVED,
      } as AdoptionRequest,
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
    console.log("Filter selections:", filterSelections);
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
    console.log("Handle request for:", requestData.animal.id);
    // Navigate to request management or open action modal
  }

  /**
   * Handles approving a request
   */
  function handleApprove(requestData: AdoptionRequestData): void {
    console.log("Approve request:", requestData.request.id);
  }

  /**
   * Handles rejecting a request
   */
  function handleReject(requestData: AdoptionRequestData): void {
    console.log("Reject request:", requestData.request.id);
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
    adoptionRequests.filter((requestData) => {
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
    })
  );
</script>

<div class="adoption-requests-page">
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
