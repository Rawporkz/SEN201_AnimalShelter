<!--
FilterModal.svelte

Modal component that presents available filtering criteria to users.
Allows users to select and configure different filter options for
data filtering throughout the application.
-->

<script lang="ts">
  import {
    FilterCriteria,
    FilterType,
    getFilterDisplayName,
    getFilterType,
    createEmptyFilterSelections,
    type FilterSelections,
  } from "$lib/utils/filter-utils";
  import {
    ANIMAL_STATUS_OPTIONS,
    ANIMAL_SEX_OPTIONS,
    DATE_RANGE_FILTER_OPTIONS,
    ANIMAL_BREED_OPTIONS,
    ANIMAL_SPECIES_OPTIONS,
  } from "$lib/config/animal-options";
  import ChooseMultiFilter from "./ChooseMultiFilter/ChooseMuliFilter.svelte";
  import ChooseOneFilter from "./ChooseOneFilter/ChooseOneFilter.svelte";
  import NestedChooseManyFilter from "./NestedChooseManyFilter/NestedChooseManyFilter.svelte";
  import ClosePopupButton from "../ClosePopupButton/ClosePopupButton.svelte";

  // Props
  interface Props {
    /** Whether the modal is currently visible */
    isVisible?: boolean;
    /** List of filter criteria to show in the modal */
    criteriaList?: FilterCriteria[];
    /** Current filter selections (for pre-populating the modal) */
    currentSelections?: FilterSelections | null;
    /** Callback function called when modal closes with filter selections */
    onclose?: (selections: FilterSelections) => void;
  }

  const {
    isVisible = false,
    criteriaList = [],
    currentSelections = null,
    onclose,
  }: Props = $props();

  // Currently selected criteria for showing in the content area
  let selectedCriteria: FilterCriteria | null = $state(
    criteriaList.length > 0 ? criteriaList[0] : null,
  );

  // Filter selections being built in the modal
  let workingSelections: FilterSelections = $state(
    currentSelections || createEmptyFilterSelections(criteriaList),
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
    if (onclose) {
      onclose(workingSelections);
    }
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

  // Update working selections when currentSelections prop changes
  $effect(() => {
    workingSelections =
      currentSelections || createEmptyFilterSelections(criteriaList);
  });

  // Update selected criteria when criteriaList changes
  $effect(() => {
    if (criteriaList.length > 0 && selectedCriteria === null) {
      selectedCriteria = criteriaList[0];
    }
  });

  /**
   * Gets options for a specific filter criteria.
   *
   * @param criteria - The criteria to get options for
   * @returns Array of option objects with value and label
   */
  function getOptionsForCriteria(
    criteria: FilterCriteria,
  ): Array<{ value: string; label: string }> {
    switch (criteria) {
      case FilterCriteria.STATUS:
        return ANIMAL_STATUS_OPTIONS;
      case FilterCriteria.SEX:
        return ANIMAL_SEX_OPTIONS;
      case FilterCriteria.ADMISSION_DATE:
      case FilterCriteria.ADOPTION_DATE:
        return DATE_RANGE_FILTER_OPTIONS;
      default:
        return [];
    }
  }

  /**
   * Gets currently selected values for a criteria.
   *
   * @param criteria - The criteria to get selections for
   * @returns Array of selected values
   */
  function getSelectedValuesForCriteria(criteria: FilterCriteria): string[] {
    const value = workingSelections[criteria];
    if (Array.isArray(value)) {
      return value;
    }
    // If value is null, return all available options (everything selected)
    if (value === null) {
      return getOptionsForCriteria(criteria).map((option) => option.value);
    }
    return [];
  }

  /**
   * Gets currently selected value for single-selection criteria.
   *
   * @param criteria - The criteria to get selection for
   * @returns Selected value or null if none selected
   */
  function getSelectedValueForCriteria(
    criteria: FilterCriteria,
  ): string | null {
    const value = workingSelections[criteria];
    if (typeof value === "string") {
      return value;
    }
    // If value is null, return the first option (default selection)
    if (value === null) {
      const options = getOptionsForCriteria(criteria);
      return options.length > 0 ? options[0].value : null;
    }
    return null;
  }

  /**
   * Handles selection changes from multi-select filter components.
   *
   * @param criteria - The criteria being updated
   * @param selectedValues - The new selected values
   */
  function handleFilterSelectionChange(
    criteria: FilterCriteria,
    selectedValues: string[],
  ): void {
    workingSelections = {
      ...workingSelections,
      [criteria]: selectedValues,
    };
  }

  /**
   * Handles selection changes from single-select filter components.
   *
   * @param criteria - The criteria being updated
   * @param selectedValue - The new selected value
   */
  function handleSingleFilterSelectionChange(
    criteria: FilterCriteria,
    selectedValue: string | null,
  ): void {
    workingSelections = {
      ...workingSelections,
      [criteria]: selectedValue,
    };
  }

  /**
   * Gets nested options for hierarchical filter criteria.
   *
   * @param criteria - The criteria to get nested options for
   * @returns Record of categories with their labels and items
   */
  function getNestedOptionsForCriteria(
    criteria: FilterCriteria,
  ): Record<string, { label: string; items: string[] }> {
    if (criteria === FilterCriteria.SPECIES_AND_BREEDS) {
      const nestedOptions: Record<string, { label: string; items: string[] }> =
        {};

      for (const speciesOption of ANIMAL_SPECIES_OPTIONS) {
        nestedOptions[speciesOption.value] = {
          label: speciesOption.label,
          items: ANIMAL_BREED_OPTIONS[speciesOption.value] || [],
        };
      }

      return nestedOptions;
    }

    return {};
  }

  /**
   * Gets currently selected nested values for a criteria.
   *
   * @param criteria - The criteria to get selections for
   * @returns Record of selected values per category
   */
  function getSelectedNestedValuesForCriteria(
    criteria: FilterCriteria,
  ): Record<string, string[]> {
    const value = workingSelections[criteria];

    if (value && typeof value === "object" && !Array.isArray(value)) {
      return value as Record<string, string[]>;
    }

    // If value is null, return all options selected (everything selected)
    if (value === null) {
      const nestedOptions = getNestedOptionsForCriteria(criteria);
      const allSelected: Record<string, string[]> = {};

      for (const category in nestedOptions) {
        allSelected[category] = [...nestedOptions[category].items];
      }

      return allSelected;
    }

    // Return empty selections for each category
    const nestedOptions = getNestedOptionsForCriteria(criteria);
    const emptySelections: Record<string, string[]> = {};

    for (const category in nestedOptions) {
      emptySelections[category] = [];
    }

    return emptySelections;
  }

  /**
   * Handles selection changes from nested filter components.
   *
   * @param criteria - The criteria being updated
   * @param selectedValues - The new selected nested values
   */
  function handleNestedFilterSelectionChange(
    criteria: FilterCriteria,
    selectedValues: Record<string, string[]>,
  ): void {
    workingSelections = {
      ...workingSelections,
      [criteria]: selectedValues,
    };
  }
</script>

{#if isVisible}
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <div
    class="modal-backdrop"
    onclick={handleBackdropClick}
    role="dialog"
    aria-modal="true"
    aria-labelledby="filter-modal-title"
    tabindex="-1"
  >
    <div class="modal-container">
      <div class="close-button-wrapper">
        <ClosePopupButton onclick={handleClose} />
      </div>
      <div class="modal-header">
        <h2 class="modal-title">Filters</h2>
      </div>

      <div class="criteria-menu">
        {#each criteriaList as criteria (criteria)}
          <button
            type="button"
            class="criteria-item {selectedCriteria === criteria
              ? 'active'
              : ''}"
            onclick={() => handleCriteriaClick(criteria)}
          >
            {getFilterDisplayName(criteria)}
          </button>
        {/each}
      </div>

      <div class="filter-content">
        {#if selectedCriteria}
          {#if getFilterType(selectedCriteria) === FilterType.CHOOSE_MANY}
            <ChooseMultiFilter
              title={getFilterDisplayName(selectedCriteria)}
              options={getOptionsForCriteria(selectedCriteria)}
              selectedValues={getSelectedValuesForCriteria(selectedCriteria)}
              onSelect={(values) =>
                handleFilterSelectionChange(selectedCriteria!, values)}
            />
          {:else if getFilterType(selectedCriteria) === FilterType.CHOOSE_ONE}
            <ChooseOneFilter
              title={getFilterDisplayName(selectedCriteria)}
              options={getOptionsForCriteria(selectedCriteria)}
              selectedValue={getSelectedValueForCriteria(selectedCriteria)}
              onSelect={(value) =>
                handleSingleFilterSelectionChange(selectedCriteria!, value)}
            />
          {:else if getFilterType(selectedCriteria) === FilterType.NESTED_CHOOSE_MANY}
            <NestedChooseManyFilter
              title={getFilterDisplayName(selectedCriteria)}
              nestedOptions={getNestedOptionsForCriteria(selectedCriteria)}
              selectedValues={getSelectedNestedValuesForCriteria(
                selectedCriteria,
              )}
              onSelect={(values) =>
                handleNestedFilterSelectionChange(selectedCriteria!, values)}
            />
          {:else}
            <div class="filter-content-placeholder">
              <p>
                Filter options for {getFilterDisplayName(selectedCriteria)} will
                appear here
              </p>
            </div>
          {/if}
        {:else}
          <div class="filter-content-placeholder">
            <p>Select a filter criteria from the left menu</p>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style lang="scss">
  @use "./style.scss";
</style>
