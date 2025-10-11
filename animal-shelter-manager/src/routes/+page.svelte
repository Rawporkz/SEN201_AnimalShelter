<!-- 
routes/+page.svelte

This file is the root page of the frontend application.
It decides whether to show the authentication page, the staff's home page,
or the customer's home page based on the user's authentication status and role.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import FilterModal from "$lib/components/FilterModal/FilterModal.svelte";
  import {
    FilterCriteria,
    type FilterSelections,
  } from "$lib/components/FilterModal/filter-utils";
  import "./style.scss";

  // Current authenticated user object, null if not logged in
  let currentUser = $state(null);

  // Filter modal state for testing
  let isFilterModalVisible: boolean = $state(false);

  // Current filter selections
  let currentFilterSelections: FilterSelections | null = $state(null);

  // List of filter criteria to show in the modal
  const testCriteriaList: FilterCriteria[] = [
    FilterCriteria.STATUS,
    FilterCriteria.SEX,
    FilterCriteria.SPECIES_AND_BREEDS,
    FilterCriteria.ADMISSION_DATE,
    FilterCriteria.ADOPTION_DATE,
  ];

  // Commented out for testing FilterModal
  // onMount(() => {
  //   // Check if user is authenticated
  //   if (currentUser === null) {
  //     // Redirect to authentication page if not logged in
  //     goto("/authentication");
  //   } else {
  //     // TODO: Handle authenticated user - show appropriate dashboard
  //   }
  // });

  /**
   * Handles opening the filter modal.
   */
  function handleOpenFilterModal(): void {
    isFilterModalVisible = true;
  }

  /**
   * Handles closing the filter modal and receiving the selections.
   *
   * @param selections - The filter selections from the modal
   */
  function handleFilterModalClose(selections: FilterSelections): void {
    currentFilterSelections = selections;
    isFilterModalVisible = false;
  }
</script>

<div class="root-page">
  <div class="test-section">
    <button type="button" onclick={handleOpenFilterModal}>
      Open Filter Modal
    </button>

    {#if currentFilterSelections}
      <div class="filter-results">
        <pre>{JSON.stringify(currentFilterSelections, null, 2)}</pre>
      </div>
    {/if}
  </div>
</div>

<FilterModal
  isVisible={isFilterModalVisible}
  criteriaList={testCriteriaList}
  currentSelections={currentFilterSelections}
  onclose={handleFilterModalClose}
/>
