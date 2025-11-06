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
  import {
    type CurrentUser,
    getCurrentUser,
  } from "$lib/utils/authentication-utils";
  import "./style.scss";

  onMount(async () => {
    try {
      // Check if user is authenticated
      const currentUser: CurrentUser | null = await getCurrentUser();

      if (currentUser === null) {
        // Redirect to authentication page if not logged in
        goto("/authentication");
      } else {
        // Handle authenticated user - redirect to appropriate home based on role
        if (currentUser.role === "staff") {
          goto("/home/staff");
        } else if (currentUser.role === "customer") {
          goto("/home/customer");
        } else {
          // Unknown role, redirect to authentication
          error(`Unknown user role: ${currentUser.role}`);
          goto("/authentication");
        }
      }
    } catch (err) {
      error(`Authentication check failed: ${err}`);
      goto("/authentication");
    }
  });
</script>
