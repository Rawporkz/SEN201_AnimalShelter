<!--
FormDropdownButton.svelte

Form-themed dropdown button component for selecting options from a predefined list,
with customizable styling, placeholder text, and width for form interfaces.
-->

<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import { onMount, onDestroy } from "svelte";

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
    /** Disable the dropdown (non-interactive, muted styles) */
    disabled?: boolean;
    /** When this value changes, the selection resets to placeholder */
    resetOn?: unknown;
    /** Flag to indicate if the input is invalid */
    isInvalid?: boolean;
  }

  const {
    options,
    placeholder,
    width,
    label,
    onSelect,
    maxOptions = 5,
    disabled = false,
    resetOn,
    isInvalid = false,
  }: Props = $props();

  /** Currently selected option, defaults to placeholder */
  let text: string = $state(placeholder);

  /** State to track whether the dropdown menu is currently open */
  let isOpen: boolean = $state(false);

  /** Root element reference for outside-click detection */
  let rootEl: HTMLDivElement | null = null;

  /**
   * Close the menu when disabled is turned on.
   */
  $effect(() => {
    if (disabled && isOpen) {
      isOpen = false;
    }
  });

  /**
   * Reset selection when resetOn changes (e.g., parent dependency changed).
   * @returns void
   */
  let lastResetToken: unknown = undefined;
  $effect(() => {
    if (resetOn !== lastResetToken) {
      lastResetToken = resetOn;
      text = placeholder;
      isOpen = false;
    }
  });

  /**
   * Toggles the dropdown menu's visibility state.
   */
  function toggle(): void {
    if (disabled) return;
    isOpen = !isOpen;
  }

  /**
   * Handles the selection of a dropdown option, updating the text and closing the menu.
   * @param option - The selected option string
   */
  function selectOption(option: string): void {
    if (disabled) return;
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

  /*
   * Handles clicks outside of this component to close the dropdown.
   * @param event - The pointer event fired on the document
   */
  function handleDocumentPointerDown(event: PointerEvent): void {
    if (!isOpen) return;
    if (!rootEl) return;
    const target = event.target as Node | null;
    if (target && rootEl.contains(target)) return;
    isOpen = false;
  }

  /*
   * Handles the Escape key to close the dropdown when open.
   * @param event - The keyboard event fired on the document
   */
  function handleDocumentKeyDown(event: KeyboardEvent): void {
    if (!isOpen) return;
    if (event.key === "Escape") {
      isOpen = false;
    }
  }

  onMount(() => {
    document.addEventListener("pointerdown", handleDocumentPointerDown, true);
    document.addEventListener("keydown", handleDocumentKeyDown, true);
  });

  onDestroy(() => {
    document.removeEventListener(
      "pointerdown",
      handleDocumentPointerDown,
      true,
    );
    document.removeEventListener("keydown", handleDocumentKeyDown, true);
  });
</script>

<div class="dropdown-container" style="width: {width};" bind:this={rootEl}>
  {#if label}
    <label class="dropdown-label" for="dropdown-label">
      {label}
    </label>
  {/if}

  <div class="dropdown" class:disabled>
    <button
      class="dropdown-button"
      class:active={isOpen}
      class:chosen={optionChosen()}
      class:error-border={isInvalid}
      onclick={toggle}
      style="width: {width};"
      type="button"
      {disabled}
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
</div>

<style lang="scss">
  @use "./style.scss";
</style>
