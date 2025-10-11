<script>
  import { emit } from "@tauri-apps/api/event";

</script>
<!-- 
lib/components/AdopterInfo.svelte

This file defines a reusable AdopterInfo component.
-->
<script lang="ts">
  import "./style.scss";
  import { User } from "@lucide/svelte";

  // Fields
  export let name: string = "Adopter Name";
  export let occupation: string = "Construction Worker";
  export let annualIncome: string = "150k-200k THB";
  export let email: string = "example@gmail.com";
  export let phoneNumber: string = "081-234-5678";
  export let streetAddress: string = "1 Chalong Krung 1 Alley, Khwaeng Lat Krabang, Lat Krabang, Bangkok 10520";
  export let country: string = "Thailand";
  export let numOfHousehold: number = 4;
  export let numOfChildren: number = 1;

</script>
