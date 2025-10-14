<script lang="ts">

  import "./style.scss";

  // Define props/arguments for the component
  const { options, placeholder, width, label } = $props<{
    options: string[],
    placeholder: string,
    width: string,
    label: string
  }>();

  // Reactive state to track selected option
  let selection = $state();

  // Initialize selected option to placeholder
  selection = placeholder;

  // Reactive state to track dropdown active status
  let isActive = $state(false);

  // Toggles dropdown between active/inactive state
  function toggle() {
    isActive = !isActive;
  }

  // Handles option selection from dropdown
  function selectOption(option: string) {
    selection = option;
    isActive = false; // Close dropdown after selection
  }

  // Returns true if an option other than the placeholder has been chosen
  function optionChosen() {
    return selection !== placeholder;
  }

</script>

<!-- Dropdown label -->
<h1 class=dropdown-label>
  {label}
</h1>

<!-- Dropdown button -->
<button 
  class="dropdown-button"
  class:active={isActive}
  onclick={toggle}
  style="width: {width};"
  class:chosen={optionChosen()}
  >
  {selection}
</button>

<!-- Dropdown options, shown only if dropdown is active -->
{#if isActive}
    {#each options as option, i (option)}
        <button
            class="dropdown-option"
            class:last={i === options.length - 1}  onclick={() => selectOption(option)}
            style="width: {width};"
        >
            {option}
        </button>
    {/each}
{/if}


