<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import "./style.scss";
  import AdopterInfo from "../lib/components/AdopterInfo/AdopterInfo.svelte";

  // Mock user for testing
  let currentUser = $state({ id: 1, role: "customer" });

  let showAdopterInfo = true;

  onMount(() => {
    if (currentUser === null) {
      goto("/authentication");
    } else {
      showAdopterInfo = true;
    }
  });
</script>

{#if showAdopterInfo}
  <AdopterInfo
    name="John Doe"
    occupation="Software Engineer"
    annualIncome="100k-150k THB"
    email="john.doe@example.com"
    phoneNumber="080-123-4567"
    streetAddress="123 Main St, Bangkok, Thailand"
    country="Thailand"
    numOfHousehold={3}
    numOfChildren={1}
  />
{/if}
