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

  /** Current authenticated user object, null if not logged in */
  let currentUser = $state(null);

  // Test click handler
  function handleRejectClick(): void {
    console.info("Reject clicked");
  }

  // Navigate to the modal test route
  function openModalTest(): void {
    goto("/modal-test");
  }

  // onMount(() => {
  // =======
  //   onMount(() => {
  //     // Check if user is authenticated
  //     if (currentUser === null) {
  //       // Redirect to authentication page if not logged in
  //       goto("/authentication");
  //     } else {
  //       // TODO: Handle authenticated user - show appropriate dashboard
  //     }
  //   });
</script>

<div style="padding: 16px; display: inline-block;">
  <NormalButton
    color="#ea4444"
    textColor="#ffffff"
    text="Reject"
    width="106px"
    onClick={handleRejectClick}
  />
</div>
<div style="padding: 16px; display: inline-block;">
  <NormalButton
    color="#004abb"
    textColor="#ffffff"
    text="Open Modal Test"
    width="180px"
    onClick={openModalTest}
  />
</div>
