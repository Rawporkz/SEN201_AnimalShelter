<!--
FormDropdownButton.svelte

Form-themed dropdown button component for selecting options from a predefined list,
with customizable styling, placeholder text, and width for form interfaces.
-->

<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";

  // Props interface
  interface Props {
    /** Array of available options to select from */
    options: string[];
    /** Placeholder text shown when no option is selected */
    placeholder: string;
    /** CSS width value for the dropdown button */
    width: string;
    /** Label text displayed above the dropdown */
    label: string;
    /** Callback function called when an option is selected */
    onSelect?: (selectedValue: string) => void;
    /** Maximum number of options to show before enabling scroll */
    maxOptions?: number;
  }

  const {
    options,
    placeholder,
    width,
    label,
    onSelect,
    maxOptions = 5,
  }: Props = $props();

  /** Currently selected option, defaults to placeholder */
  let text: string = $state(placeholder);

  /** State to track whether the dropdown menu is currently open */
  let isOpen: boolean = $state(false);

  /**
   * Toggles the dropdown menu's visibility state.
   */
  function toggle(): void {
    isOpen = !isOpen;
  }

  /**
   * Handles the selection of a dropdown option, updating the text and closing the menu.
   * @param option - The selected option string
   */
  function selectOption(option: string): void {
    text = option;
    isOpen = false;
    if (onSelect) {
      onSelect(option);
    }
  }

  /**
   * Checks if a valid option (not placeholder) has been selected.
   * @returns boolean - True if an option other than placeholder is selected
   */
  function optionChosen(): boolean {
    return text !== placeholder;
  }

  /**
   * Checks if the dropdown should be scrollable based on number of options.
   * @returns boolean - True if options exceed maxOptions limit
   */
  function shouldScroll(): boolean {
    return options.length > maxOptions;
  }
</script>

<h1 class="dropdown-label">
  {label}
</h1>

<div class="dropdown" style="width: {width};">
  <button
    class="dropdown-button"
    class:active={isOpen}
    class:chosen={optionChosen()}
    onclick={toggle}
    style="width: {width};"
    type="button"
  >
    <span class="button-text">{text}</span>
    <div class="chevron" class:rotated={isOpen}>
      <ChevronDown size={16} />
    </div>
  </button>

  {#if isOpen}
    <div 
      class="dropdown-options-container" 
      class:scrollable={shouldScroll()}
      style="--max-height: {maxOptions * 44}px;"
    >
      {#each options as option, i (option)}
        <button
          class="dropdown-option"
          class:last={i === options.length - 1}
          onclick={() => selectOption(option)}
          style="width: {width};"
          type="button"
        >
          {option}
        </button>
        {#if i < options.length - 1}
          <div class="horizontal-divider"></div>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "./style.scss";
</style>
