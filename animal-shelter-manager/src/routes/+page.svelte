<!-- 
routes/+page.svelte

This file is the root page of the frontend application.
It decides whether to show the authentication page, the staff's home page,
or the customer's home page based on the user's authentication status and role.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { error } from "@tauri-apps/plugin-log";
  import { getCurrentUser, UserRole } from "$lib/utils/authentication-utils";
  import "./style.scss";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";
  import { User } from "@lucide/svelte";

  /** Current authenticated user object with role, null if not logged in */
  let currentUser: { username: string; role: UserRole } | null = $state(null);

  onMount(async () => {
    try {
      // Check if user is authenticated
      currentUser = (await getCurrentUser()) as {
        username: string;
        role: UserRole;
      } | null;

      if (currentUser === null) {
        // Redirect to authentication page if not logged in
        goto("/authentication");
        currentUser = { username: "aaa", role: UserRole.STAFF };
      } else {
        // Handle authenticated user - redirect to appropriate dashboard based on role
        if (currentUser.role === "staff") {
          goto("/home/staff");
        } else if (currentUser.role === "customer") {
          // TODO: Create customer home page
          goto("/home/customer");
        } else {
          // Unknown role, redirect to authentication
          goto("/authentication");
        }
      }
    } catch (err) {
      error(`Authentication check failed: ${err}`);
      goto("/authentication");
    }
  });
</script>

<SideBar username="Jira" role="Staff"></SideBar>
