<!-- 
lib/components/ClosePopupButton.svelte

This file defines a reusable ClosePopupButton component.
-->

<script lang="ts">
  import "./style.scss";
  import { X } from "@lucide/svelte";

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
  <X class="close-icon"></X>
</button>
