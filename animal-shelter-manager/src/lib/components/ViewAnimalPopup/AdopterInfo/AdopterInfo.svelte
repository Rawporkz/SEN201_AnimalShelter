<!-- 
ViewAnimalModal/AdopterInfo.svelte

This file defines a reusable AdopterInfo component.
-->
<script lang="ts">
  import type { AdoptionRequest } from "../../../utils/animal-utils";
  import ClosePopupButton from "../../ClosePopupButton/ClosePopupButton.svelte";

  // Props
  interface Props {
    /** The adopter information (optional) */
    adopter?: AdoptionRequest | null;
    /** Callback function to close the popup */
    onclose?: () => void;
  }

  const { adopter, onclose }: Props = $props();
</script>

<div class="adopter-info-modal" role="dialog" aria-modal="true" tabindex="0">
  <div class="close-button-wrapper">
    <ClosePopupButton onclick={onclose} />
  </div>
  <div class="adopter-info-header">
    <h1>Adopter Information</h1>
  </div>
  <div class="adopter-info-content">
    <div class="adopter-info-row">
      <div class="adopter-info-item">
        <div class="label">Name</div>
        <div class="value">{adopter?.name ?? "Adopter Name"}</div>
      </div>
      <div class="adopter-info-item">
        <div class="label">Occupation</div>
        <div class="value">{adopter?.occupation ?? "Unknown"}</div>
      </div>
      <div class="adopter-info-item">
        <div class="label">Annual Income</div>
        <div class="value">{adopter?.annualIncome ?? "Unknown"}</div>
      </div>
    </div>
    <div class="adopter-info-row">
      <div class="adopter-info-item">
        <div class="label">Email address</div>
        <div class="value">{adopter?.email ?? "example@gmail.com"}</div>
      </div>
      <div class="adopter-info-item">
        <div class="label">Telephone Number</div>
        <div class="value">{adopter?.telNumber ?? "Unknown"}</div>
      </div>
    </div>
    <div class="divider"></div>
    <div class="adopter-info-item full-width">
      <div class="label">Street Address</div>
      <div class="value">{adopter?.address ?? "Unknown"}</div>
    </div>
    <div class="adopter-info-item full-width">
      <div class="label">Country</div>
      <div class="value">{adopter?.country ?? "Unknown"}</div>
    </div>
    <div class="divider"></div>
    <div class="adopter-info-item full-width">
      <div class="label">How many people do you have in your household?</div>
      <div class="value">Answer: {adopter?.numPeople ?? "Unknown"}</div>
    </div>
    <div class="adopter-info-item full-width">
      <div class="label">
        Of which, how many are children under 15 years old?
      </div>
      <div class="value">Answer: {adopter?.numChildren ?? "Unknown"}</div>
    </div>
  </div>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
