<!-- 
routes/+page.svelte

This file is the root page of the frontend application.
It decides whether to show the authentication page, the staff's home page,
or the customer's home page based on the user's authentication status and role.
-->

<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import "./style.scss";
  import ClosePopupButton from "$lib/components/ClosePopupButton/ClosePopupButton.svelte";
  import SideBar from "$lib/components/SideBar/SideBar.svelte";

  // Current authenticated user object, null if not logged in
  let currentUser = $state(null);

  // onMount(() => {
  //   // Check if user is authenticated
  //   if (currentUser === null) {
  //     // Redirect to authentication page if not logged in
  //     goto("/authentication");
  //   } else {
  //     // TODO: Handle authenticated user - show appropriate dashboard
  //   }
  // });
  //
</script>

<ClosePopupButton></ClosePopupButton>
<SideBar username="John Doe" role="as Staff"></SideBar>
