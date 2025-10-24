<!--
  ActionDropdownButton.svelte

  Action-themed dropdown button component with fixed blue styling and customizable menu,
  showing button label, icon, and expandable options list for action-oriented interfaces.
-->

<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import type { Component } from "svelte";

  // Props interface
  interface Props {
    /** Label text displayed on the dropdown button */
    label: string;
    /** Icon component to display on the button */
    icon?: Component<any>;
    /** Array of dropdown menu options with labels, icons, and click handlers */
    options: Array<{
      label: string;
      icon?: Component<any>;
      onclick?: () => void;
    }>;
  }

  const { label, options, icon }: Props = $props();

  /** State to track whether the dropdown menu is currently open */
  let isOpen: boolean = $state(false);

  /**
   * Toggles the dropdown menu's visibility state.
   * @returns void
   */
  function toggleDropdown(): void {
    isOpen = !isOpen;
  }

  /**
   * Handles the click event for a dropdown option, executing its callback and closing the menu.
   * @param option - The selected dropdown option containing label, icon, and onclick handler
   * @returns void
   */
  function handleOptionClick(option: (typeof options)[0]): void {
    if (option.onclick) {
      option.onclick();
    }
    isOpen = false;
  }

  /**
   * Handles clicks outside the dropdown component to automatically close the menu.
   * @param event - The mouse event triggered by clicking outside the component
   * @returns void
   */
  function handleClickOutside(event: MouseEvent): void {
    const target = event.target as HTMLElement;
    if (!target.closest(".dropdown-button-container")) {
      isOpen = false;
    }
  }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="dropdown-button-container">
  <button
    class="dropdown-button"
    class:open={isOpen}
    onclick={toggleDropdown}
    type="button"
  >
    {#if icon}
      {@const IconComponent = icon}
      <div class="button-icon">
        <IconComponent size={18} />
      </div>
    {/if}
    <span class="button-label">{label}</span>
    <div class="chevron" class:rotated={isOpen}>
      <ChevronDown size={16} />
    </div>
  </button>

  {#if isOpen}
    <div class="dropdown-menu">
      <div class="dropdown-divider"></div>
      {#each options as option, index}
        {#if index > 0}
          <div class="dropdown-divider"></div>
        {/if}
        <button
          class="dropdown-option"
          onclick={() => handleOptionClick(option)}
          type="button"
        >
          {#if option.icon}
            {@const OptionIconComponent = option.icon}
            <div class="option-icon">
              <OptionIconComponent size={18} />
            </div>
          {/if}
          <span>{option.label}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "./style.scss";
</style>
