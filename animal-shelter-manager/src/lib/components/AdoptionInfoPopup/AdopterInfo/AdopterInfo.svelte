<!-- 
lib/components/AdopterInfo.svelte

This file defines a reusable AdopterInfo component.
-->
<script lang="ts">
  import "./style.scss";
  import { User } from "@lucide/svelte";
  import { emit } from "@tauri-apps/api/event";

  // Fields
  export let name: string = "Adopter Name";
  export let occupation: string = "Construction Worker";
  export let annualIncome: string = "150k-200k THB";
  export let email: string = "example@gmail.com";
  export let phoneNumber: string = "081-234-5678";
  export let streetAddress: string =
    "1 Chalong Krung 1 Alley, Khwaeng Lat Krabang, Lat Krabang, Bangkok 10520";
  export let country: string = "Thailand";
  export let numOfHousehold: number = 4;
  export let numOfChildren: number = 1;

  const closeModal = () => {
    emit("close-adopter-info");
  };
</script>

<div class="adopter-info-overlay">
  <div
    class="adopter-info-modal"
    role="dialog"
    aria-modal="true"
    tabindex="0"
    on:keydown={(e) => {
      if (e.key === "Escape") closeModal();
    }}
  >
    <div class="adopter-info-header">
      <h1>Adopter Information</h1>
    </div>
    <div class="adopter-info-content">
      <div class="adopter-info-row">
        <div class="adopter-info-item">
          <div class="label">Name</div>
          <div class="value">{name}</div>
        </div>
        <div class="adopter-info-item">
          <div class="label">Occupation</div>
          <div class="value">{occupation}</div>
        </div>
        <div class="adopter-info-item">
          <div class="label">Annual Income</div>
          <div class="value">{annualIncome}</div>
        </div>
      </div>
      <div class="adopter-info-row">
        <div class="adopter-info-item">
          <div class="label">Email address</div>
          <div class="value">{email}</div>
        </div>
        <div class="adopter-info-item">
          <div class="label">Telephone Number</div>
          <div class="value">{phoneNumber}</div>
        </div>
      </div>
      <div class="divider"></div>
      <div class="adopter-info-item full-width">
        <div class="label">Street Address</div>
        <div class="value">{streetAddress}</div>
      </div>
      <div class="adopter-info-item full-width">
        <div class="label">Country</div>
        <div class="value">{country}</div>
      </div>
      <div class="divider"></div>
      <div class="adopter-info-item full-width">
        <div class="label">How many people do you have in your household?</div>
        <div class="value">Answer: {numOfHousehold}</div>
      </div>
      <div class="adopter-info-item full-width">
        <div class="label">
          Of which, how many are children under 15 years old?
        </div>
        <div class="value">Answer: {numOfChildren}</div>
      </div>
    </div>
  </div>
</div>
