<!--
  SearchBar.svelte

  Reusable search search bar component with a search icon and placeholder text.
  Provides a bindable value that parent components can use to handle search queries.
-->

<script lang="ts">
  import { Search } from "@lucide/svelte";

  // Props
  interface Props {
    /** The current search value - bindable by parent components */
    value?: string;
    /** Placeholder text to display in the search input */
    placeholder?: string;
    /** Whether the search input is disabled */
    disabled?: boolean;
  }

  // Get all props
  let {
    value = $bindable(""),
    placeholder = "Search for names, IDs, breeds, and more...",
    disabled = false,
  }: Props = $props();

  /**
   * Handles keydown events for potential future enhancements (e.g., Enter key handling).
   *
   * @param event - The keyboard event from the search field
   */
  function handleKeydown(event: KeyboardEvent): void {
    // Future enhancement: Handle Enter key for immediate search
    if (event.key === "Enter") {
      // Could dispatch a custom search event here if needed
    }
  }
</script>

<div class="search-bar">
  <div class="search-icon">
    <Search size={16} />
  </div>
  <input
    type="text"
    class="search-input"
    {placeholder}
    {disabled}
    bind:value
    onkeydown={handleKeydown}
  />
</div>

<style lang="scss">
  @use "./style.scss";
</style>
