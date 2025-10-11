<!-- 
routes/+page.svelte

This file is the root page of the frontend application.
It decides whether to show the authentication page, the staff's home page,
or the customer's home page based on the user's authentication status and role.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import "./style.scss";
  import NormalButton from "$lib/components/NormalButton/NormalButton.svelte";

  // Current authenticated user object, null if not logged in
  let currentUser = $state(null);

  // TEMP: Enable to quickly test the NormalButton on this page
  const TEST_BUTTON: boolean = true;

  // Test click handler
  function handleRejectClick(): void {
    console.info("Reject clicked");
  }

  onMount(() => {
    if (TEST_BUTTON) {
      // Skip redirect while testing the button on this page
      return;
    }

    // Check if user is authenticated
    if (currentUser === null) {
      // Redirect to authentication page if not logged in
      goto("/authentication");
    } else {
      // TODO: Handle authenticated user - show appropriate dashboard
    }
  });
</script>

{#if TEST_BUTTON}
  <div style="padding: 16px; display: inline-block;">
    <NormalButton
      color="#ea4444"
      textColor="#ffffff"
      text="Reject"
      width="106px"
      onClick={handleRejectClick}
    />
  </div>
{/if}