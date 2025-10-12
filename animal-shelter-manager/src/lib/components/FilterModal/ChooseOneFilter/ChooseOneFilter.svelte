<!--
  FilterModal/ChooseOneFilter.svelte

  Filter component for criteria that allow single selection only.
  Presents options where users can select exactly one value
  for filtering purposes.
-->

<script lang="ts">
  import "./style.scss";

  // Props
  interface Props {
    /** The title to display for this filter */
    title: string;
    /** List of options to display */
    options: Array<{ value: string; label: string }>;
    /** Currently selected value */
    selectedValue: string | null;
    /** Callback when selection changes */
    onSelect: (selectedOption: string | null) => void;
  }

  const { title, options, selectedValue, onSelect }: Props = $props();

  /**
   * Handles radio button selection changes.
   *
   * @param optionValue - The value of the option being selected
   */
  function handleOptionChange(optionValue: string): void {
    onSelect(optionValue);
  }
</script>

<div class="choose-one-filter">
  <div class="filter-header">
    <h3 class="filter-title">{title}</h3>
  </div>

  <div class="options-list">
    {#each options as option (option.value)}
      <label class="option-item">
        <input
          type="radio"
          class="option-radio"
          name="{title}-options"
          value={option.value}
          checked={selectedValue === option.value}
          onchange={() => handleOptionChange(option.value)}
        />
        <span class="option-label">{option.label}</span>
      </label>
    {/each}
  </div>
</div>
