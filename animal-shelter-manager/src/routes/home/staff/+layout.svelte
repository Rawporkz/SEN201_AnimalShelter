<!--
 +layout.svelte 

 This file defines the layout for the staff home page.
 It includes a sidebar navigation and a main content area that changes
 based on the selected navigation item.
 -->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { error } from "@tauri-apps/plugin-log";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";
  import { logoutUser } from "$lib/utils/authentication-utils";
  import type { LayoutData } from "./$types";

  interface Props {
    data: LayoutData;
  }

  const { data }: Props = $props();

  /**
   * Maps sidebar navigation items to their corresponding routes.
   * This mapping allows the sidebar to navigate to different sections.
   */
  const navigationMap: Record<string, string> = {
    "All Animals": "/home/staff/all-animals",
    "Adoption Requests": "/home/staff/adoption-requests",
    "Adoption Reports": "/home/staff/adoption-reports",
  };

  /**
   * Handles navigation when a sidebar item is clicked.
   * Navigates to the corresponding route based on the navigation mapping.
   *
   * @param item - The navigation item that was clicked
   */
  function handleNavigation(item: string): void {
    const route: string | undefined = navigationMap[item];
    if (route) {
      goto(route);
    }
  }

  /**
   * Handles user sign out process.
   * Logs out the user and redirects to the authentication page.
   */
  async function handleSignOut(): Promise<void> {
    try {
      const logoutSuccess: boolean = await logoutUser();
      if (logoutSuccess) {
        goto("/authentication");
      }
    } catch (err) {
      error(`Sign out failed: ${err}`);
    }
  }
</script>

<div class="staff-layout">
  <div class="sidebar-container">
    <SideBar
      username={data.currentUser?.username ?? "Staff User"}
      role="Staff"
      onNavigate={handleNavigation}
      onSignOut={handleSignOut}
    />
  </div>

  <main class="main-content">
    <!-- svelte-ignore slot_element_deprecated -->
    <slot />
  </main>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
