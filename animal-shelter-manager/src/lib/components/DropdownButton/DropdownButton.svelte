<!-- 
lib/components/DropdownButton/DropdownButton.svelte

This file defines a reusable DropdownButton component with a dropdown menu.
-->
<script lang="ts">
  import "./style.scss";

  // Label for the dropdown button
  export let label: string = "Handle";

  // Options for the dropdown menu
  export let options: Array<{ label: string; icon?: string; onclick?: () => void }> = [];

  // Icon for the dropdown button
  export let icon: string = "M12 2L2 7L12 12L22 7L12 2Z";

  // Title for the dropdown button
  export let title: string = "Dropdown Button";

  // State to track whether the dropdown is open
  let isOpen: boolean = false;

  /**
   * Toggles the dropdown menu's visibility.
   */
  function toggleDropdown(): void {
    isOpen = !isOpen;
  }

  /**
   * Handles the click event for a dropdown option.
   * @param option - The selected dropdown option.
   */
  function handleOptionClick(option: typeof options[0]): void {
    if (option.onclick) {
      option.onclick();
    }
    isOpen = false;
  }

  /**
   * Handles clicks outside the dropdown to close it.
   * @param event - The mouse event triggered by the click.
   */
  function handleClickOutside(event: MouseEvent): void {
    const target = event.target as HTMLElement;
    if (!target.closest(".dropdown-button-container")) {
      isOpen = false;
    }
  }
</script>

<!-- Listen for clicks on the window to close the dropdown if clicked outside -->
<svelte:window on:click={handleClickOutside} />

<div class="dropdown-button-container">
  <!-- Dropdown button -->
  <button
    class="dropdown-button"
    class:open={isOpen}
    on:click|stopPropagation={toggleDropdown}
    type="button"
    title={title}
  >
    <!-- Button icon -->
    <svg
      class="button-icon"
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
    >
      <path
        d={icon}
        fill="currentColor"
      />
    </svg>
    <!-- Button label -->
    <span class="button-label">{label}</span>
    <!-- Chevron icon -->
    <svg
      class="chevron"
      xmlns="http://www.w3.org/2000/svg"
      width="16"
      height="16"
      viewBox="0 0 24 24"
      class:rotated={isOpen}
    >
      <path
        d="M6 9L12 15L18 9"
        fill="currentColor"
      />
    </svg>
  </button>

  <!-- Dropdown menu -->
  {#if isOpen}
    <div class="dropdown-menu">
      <!-- Divider at the top of the first option -->
      <div class="dropdown-divider"></div>
      {#each options as option, index}
        {#if index > 0}
          <!-- Divider between options -->
          <div class="dropdown-divider"></div>
        {/if}
        <button
          class="dropdown-option"
          on:click|stopPropagation={() => handleOptionClick(option)}
          type="button"
        >
          <!-- Option icon -->
          {#if option.icon}
            <svg
              class="option-icon"
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
            >
              <path
                d={option.icon}
                fill="currentColor"
              />
            </svg>
          {/if}
          <!-- Option label -->
          <span>{option.label}</span>
        </button>
      {/each}
    </div>
  {/if}
</div>