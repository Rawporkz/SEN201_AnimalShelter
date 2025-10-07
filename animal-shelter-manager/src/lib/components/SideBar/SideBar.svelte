<!-- 
lib/components/SideBar.svelte

This file defines a reusable SideBar component.
-->

<script lang="ts">
  import "./style.scss";

  // Fields
  export let username: string = "Username";
  export let role: string = "as Staff";
  export let onSignOut: (() => void) | undefined = undefined;
  export let onNavigate: ((item: string) => void) | undefined = undefined;

  // Navigation items
  const navItems = ["All Animals", "Adoption Requests", "Adoption Reports"];

  // Handles sign out button click
  function handleSignOut() {
    if (onSignOut) {
      onSignOut();
    }
  }

  // Handles navigation item clicks
  function handleNavClick(item: string) {
    if (onNavigate) {
      onNavigate(item);
    }
  }
</script>

<div class="sidebar">
  <div class="user-profile">
    <div class="user-icon">
      <svg
        width="30"
        height="30"
        viewBox="0 0 24 24"
        fill="currentColor"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M12 12C14.7614 12 17 9.76142 17 7C17 4.23858 14.7614 2 12 2C9.23858 2 7 4.23858 7 7C7 9.76142 9.23858 12 12 12Z"
        />
        <path
          d="M12 14C6.47715 14 2 16.4772 2 19.5V22H22V19.5C22 16.4772 17.5228 14 12 14Z"
        />
      </svg>
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
