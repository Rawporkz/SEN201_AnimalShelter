<!--
  FilterModal/ChooseMuliFilter.svelte

  Filter component for criteria that allow multiple selections.
  Presents options as checkboxes where users can select one or
  more values for filtering purposes.
-->

<script lang="ts">
  // Props
  interface Props {
    /** The title to display for this filter */
    title: string;
    /** List of options to display */
    options: Array<{ value: string; label: string }>;
    /** Currently selected values */
    selectedValues: string[];
    /** Callback when selections change */
    onSelect: (selectedOptions: string[]) => void;
  }

  const { title, options, selectedValues, onSelect }: Props = $props();

  /**
   * Handles individual checkbox changes.
   *
   * @param optionValue - The value of the option being toggled
   * @param isChecked - Whether the checkbox is now checked
   */
  function handleOptionChange(optionValue: string, isChecked: boolean): void {
    let newSelections: string[];

    if (isChecked) {
      newSelections = [...selectedValues, optionValue];
    } else {
      newSelections = selectedValues.filter((value) => value !== optionValue);
    }

    onSelect(newSelections);
  }

  /**
   * Handles select all functionality.
   */
  function handleSelectAll(): void {
    onSelect(options.map((option) => option.value));
  }

  /**
   * Handles deselect all functionality.
   */
  function handleDeselectAll(): void {
    onSelect([]);
  }
</script>

<div class="choose-multi-filter">
  <div class="filter-header">
    <h3 class="filter-title">{title}</h3>
    <div class="filter-actions">
      <button type="button" class="action-btn" onclick={handleSelectAll}>
        Select All
      </button>
      <span class="separator">/</span>
      <button type="button" class="action-btn" onclick={handleDeselectAll}>
        Deselect All
      </button>
    </div>
  </div>

  <div class="options-list">
    {#each options as option (option.value)}
      <label class="option-item">
        <input
          type="checkbox"
          class="option-checkbox"
          value={option.value}
          checked={selectedValues.includes(option.value)}
          onchange={(e) =>
            handleOptionChange(option.value, e.currentTarget.checked)}
        />
        <span class="option-label">{option.label}</span>
      </label>
    {/each}
  </div>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
