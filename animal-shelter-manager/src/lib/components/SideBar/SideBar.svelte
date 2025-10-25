<!-- 
SideBar.svelte

This file defines a reusable SideBar component.
-->

<script lang="ts">
  import { User } from "@lucide/svelte";

  // Props
  interface Props {
    /** The username to display */
    username?: string;
    /** The user role to display */
    role?: string;
    /** The navigation items to display in the sidebar. */
    navItems: string[];
    /** Optional callback function to handle sign out events */
    onSignOut?: (() => void) | undefined;
    /** Optional callback function to handle navigation events */
    onNavigate?: ((item: string) => void) | undefined;
  }

  const {
    username = "Username",
    role = "Staff",
    navItems,
    onSignOut = undefined,
    onNavigate = undefined,
  }: Props = $props();

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
      <User size="58" class="user-icon" color="white" />
    </div>
    <div class="user-info">
      <h2>{username}</h2>
      <p>as {role}</p>
    </div>
  </div>

  <nav class="nav-menu">
    {#each navItems as item}
      <div
        class="nav-item"
        onclick={() => handleNavClick(item)}
        onkeydown={(e) => e.key === "Enter" && handleNavClick(item)}
        role="button"
        tabindex="0"
      >
        {item}
      </div>
    {/each}
  </nav>

  <button class="sign-out-btn" onclick={handleSignOut} type="button">
    Sign Out
  </button>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
