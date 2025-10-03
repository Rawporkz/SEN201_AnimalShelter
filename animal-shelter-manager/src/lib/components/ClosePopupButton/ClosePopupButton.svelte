<!-- 
lib/components/ClosePopupButton.svelte

This file defines a reusable ClosePopupButton component.
-->

<script lang="ts">
  import "./style.scss";

  // Optional callback function to handle button click events
  export let onclick: (() => void) | undefined = undefined;

  /**
   * Handles the click event for the close button.
   * Calls the provided onclick callback function if it exists.
   */
  function handleClick() {
    if (onclick) {
      onclick();
    }
  }
</script>

<button
  class="close-popup-button"
  on:click={handleClick}
  type="button"
  title="Close"
>
  <svg
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
  >
    <path
      d="M18 6L6 18M6 6L18 18"
      stroke="currentColor"
      stroke-width="2.5"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </svg>
</button>
