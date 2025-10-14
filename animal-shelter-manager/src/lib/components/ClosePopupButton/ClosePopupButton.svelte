<!-- 
ClosePopupButton.svelte

This file defines a reusable ClosePopupButton component.
-->

<script lang="ts">
  import { X } from "@lucide/svelte";

  // Props
  interface Props {
    /** Optional callback function to handle button click events */
    onclick?: (() => void) | undefined;
  }

  const { onclick = undefined }: Props = $props();

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
  onclick={handleClick}
  type="button"
  title="Close"
>
  <X size={24} />
</button>

<style lang="scss">
  @use "./style.scss";
</style>
