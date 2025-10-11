<!--
  FilterModal.svelte

  Modal component that presents available filtering criteria to users.
  Allows users to select and configure different filter options for
  data filtering throughout the application.
-->

<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import ClosePopupButton from "../ClosePopupButton/ClosePopupButton.svelte";
  import {
    FilterCriteria,
    getFilterDisplayName,
    createEmptyFilterSelections,
    type FilterSelections,
  } from "./filter-utils";
  import "./style.scss";

  // Props
  interface Props {
    /** Whether the modal is currently visible */
    isVisible?: boolean;
    /** List of filter criteria to show in the modal */
    criteriaList?: FilterCriteria[];
    /** Current filter selections (for pre-populating the modal) */
    currentSelections?: FilterSelections | null;
  }

  const { 
    isVisible = false, 
    criteriaList = [], 
    currentSelections = null 
  }: Props = $props();

  // Event dispatcher for communicating with parent components
  const dispatch = createEventDispatcher<{
    close: FilterSelections;
  }>();

  // Currently selected criteria for showing in the content area
  let selectedCriteria: FilterCriteria | null = $state(null);
  
  // Filter selections being built in the modal
  let workingSelections: FilterSelections = $state(
    currentSelections || createEmptyFilterSelections(criteriaList)
  );

  /**
   * Handles clicking on a criteria menu item.
   *
   * @param criteria - The criteria that was clicked
   */
  function handleCriteriaClick(criteria: FilterCriteria): void {
    selectedCriteria = criteria;
  }

  /**
   * Handles closing the modal and returning the current selections.
   */
  function handleClose(): void {
    dispatch("close", workingSelections);
    resetModal();
  }

  /**
   * Resets the modal state when closing.
   */
  function resetModal(): void {
    selectedCriteria = null;
  }

  /**
   * Handles backdrop clicks to close the modal.
   *
   * @param event - Mouse event from the backdrop
   */
  function handleBackdropClick(event: MouseEvent): void {
    // Only close if clicking directly on the backdrop, not child elements
    if (event.target === event.currentTarget) {
      handleClose();
    }
  }

  /**
   * Handles keyboard events for modal accessibility.
   *
   * @param event - Keyboard event
   */
  function handleKeydown(event: KeyboardEvent): void {
    if (event.key === "Escape") {
      handleClose();
    }
  }

  // Update working selections when currentSelections prop changes
  $effect(() => {
    workingSelections = currentSelections || createEmptyFilterSelections(criteriaList);
  });
</script>

<!-- Modal backdrop and container -->
{#if isVisible}
  <!-- svelte-ignore a11y-no-static-element-interactions -->
  <div 
    class="modal-backdrop"
    onclick={handleBackdropClick}
    onkeydown={handleKeydown}
    role="dialog"
    aria-modal="true"
    aria-labelledby="filter-modal-title"
  >
    <div class="modal-container">
      <!-- Modal header with title and close button -->
      <div class="modal-header">
        <h2 id="filter-modal-title" class="modal-title">Filters</h2>
        <ClosePopupButton onclick={handleClose} />
      </div>

      <!-- Modal body with criteria menu and content area -->
      <div class="modal-body">
        <!-- Left side: Criteria menu -->
        <div class="criteria-menu">
          {#each criteriaList as criteria (criteria)}
            <button
              type="button"
              class="criteria-item {selectedCriteria === criteria ? 'active' : ''}"
              onclick={() => handleCriteriaClick(criteria)}
            >
              {getFilterDisplayName(criteria)}
            </button>
          {/each}
        </div>

        <!-- Right side: Filter content area -->
        <div class="filter-content">
          {#if selectedCriteria}
            <div class="filter-content-placeholder">
              <p>Filter options for {getFilterDisplayName(selectedCriteria)} will appear here</p>
            </div>
          {:else}
            <div class="filter-content-placeholder">
              <p>Select a filter criteria from the left menu</p>
            </div>
          {/if}
        </div>
      </div>
    </div>
  </div>
{/if}
