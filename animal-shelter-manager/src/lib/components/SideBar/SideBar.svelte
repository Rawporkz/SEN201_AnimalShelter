<!-- 
lib/components/SideBar.svelte

This file defines a reusable SideBar component.
-->

<script lang="ts">
  import "./style.scss";
  import { User } from "@lucide/svelte";

  // Fields
  export let username: string = "Username";
  export let role: string = "as Staff";
  export let onSignOut: (() => void) | undefined = undefined;
  export let onNavigate: ((item: string) => void) | undefined = undefined;

  // Array of navigation items
  const navItems = ["All Animals", "Adoption Requests", "Adoption Reports"];

  /*
   * Handles the sign-out button click
   * @param item
   */
  function handleSignOut() {
    if (onSignOut) {
      onSignOut();
    }
  }

  /*
   * Handles navigation item clicks
   * Calls the provided onNavigate callback function with the clicked item if it exists.
   */
  function handleNavClick(item: string) {
    if (onNavigate) {
      onNavigate(item);
    }
  }
</script>

<div class="sidebar">
  <div class="user-profile">
    <div class="user-icon">
      <User size="56" class="user-icon" />
    </div>
    <div class="user-info">
      <h2>{username}</h2>
      <p>{role}</p>
    </div>
  </div>

  <nav class="nav-menu">
    {#each navItems as item}
      <div
        class="nav-item"
        on:click={() => handleNavClick(item)}
        on:keydown={(e) => e.key === "Enter" && handleNavClick(item)}
        role="button"
        tabindex="0"
      >
        {item}
      </div>
    {/each}
  </nav>

  <button class="sign-out-btn" on:click={handleSignOut} type="button">
    Sign Out
  </button>
</div>
