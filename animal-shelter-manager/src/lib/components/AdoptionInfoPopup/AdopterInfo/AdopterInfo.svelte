<!-- 
AdoptionInfoPopup/AdopterInfo.svelte

This file defines a reusable AdopterInfo component.
-->
<script lang="ts">
  // Props
  interface Props {
    /** Adopter's full name */
    name?: string;
    /** Adopter's occupation */
    occupation?: string;
    /** Adopter's annual income range */
    annualIncome?: string;
    /** Adopter's email address */
    email?: string;
    /** Adopter's phone number */
    phoneNumber?: string;
    /** Adopter's street address */
    streetAddress?: string;
    /** Adopter's country */
    country?: string;
    /** Number of people in household */
    numOfHousehold?: number;
    /** Number of children under 15 in household */
    numOfChildren?: number;
  }

  const {
    name = "Adopter Name",
    occupation = "Construction Worker",
    annualIncome = "150k-200k THB",
    email = "example@gmail.com",
    phoneNumber = "081-234-5678",
    streetAddress = "1 Chalong Krung 1 Alley, Khwaeng Lat Krabang, Lat Krabang, Bangkok 10520",
    country = "Thailand",
    numOfHousehold = 4,
    numOfChildren = 1,
  }: Props = $props();
</script>

<div class="adopter-info-modal" role="dialog" aria-modal="true" tabindex="0">
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

<style lang="scss">
  @use "./style.scss";
</style>
