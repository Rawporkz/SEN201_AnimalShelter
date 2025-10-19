<!--
  routes/send-adoption-request-form/[id]/+page.svelte

  This file defines the page for sending an adoption request for a specific animal.
-->

<script lang="ts">
  import { goto } from "$app/navigation";
  import { onMount } from "svelte";
  import { X, Save } from "@lucide/svelte";
  import type { PageData } from "./$types";
  import { info, error } from "@tauri-apps/plugin-log";
  import { convertFileSrc } from "@tauri-apps/api/core";

  import FormTextField from "$lib/components/FormTextField/FormTextField.svelte";
  import FormDropdownButton from "$lib/components/FormDropdownButton/FormDropdownButton.svelte";
  import GenericButton from "$lib/components/GenericButton/GenericButton.svelte";
  import { calculateAge } from "$lib/utils/animal-utils";
  import type { AdoptionRequest } from "$lib/utils/animal-utils";
  import {
    createAdoptionRequest,
    RequestStatus,
  } from "$lib/utils/animal-utils";
  import { COUNTRY_OPTIONS, INCOME_OPTIONS } from "./form-options-utils";

  /** Props passed from the load function */
  const { data }: { data: PageData } = $props();

  /** Animal data from the load function */
  const animal = data.animal;

  /** URL for the animal's image */
  let imageUrl: string | null = $state(null);

  /** Applicant's name */
  let applicantName: string = $state("");
  /** Applicant's occupation */
  let applicantOccupation: string = $state("");
  /** Applicant's annual income */
  let applicantAnnualIncome: string = $state("");
  /** Applicant's email address */
  let applicantEmail: string = $state("");
  /** Applicant's telephone number */
  let applicantTelNumber: string = $state("");
  /** Applicant's street address */
  let applicantAddress: string = $state("");
  /** Applicant's country */
  let applicantCountry: string = $state("");
  /** Applicant's state/province */
  let applicantState: string = $state("");
  /** Number of people in household */
  let numPeople: string = $state("");
  /** Number of children in household */
  let numChildren: string = $state("");

  /** Validity states for form fields */
  let isApplicantNameInvalid: boolean = $state(false);
  let isApplicantOccupationInvalid: boolean = $state(false);
  let isApplicantAnnualIncomeInvalid: boolean = $state(false);
  let isApplicantEmailInvalid: boolean = $state(false);
  let isApplicantTelNumberInvalid: boolean = $state(false);
  let isApplicantAddressInvalid: boolean = $state(false);
  let isApplicantCountryInvalid: boolean = $state(false);
  let isApplicantStateInvalid: boolean = $state(false);
  let isNumPeopleInvalid: boolean = $state(false);
  let isNumChildrenInvalid: boolean = $state(false);

  /** Flag to indicate if form submission is in progress */
  let isSaving: boolean = $state(false);

  /** Flag to indicate if the user has attempted to save the form */
  let hasAttemptedSave: boolean = $state(false);

  /** Error message to display */
  let errorMessage: string = $state("");

  onMount(() => {
    if (animal.image_path) {
      imageUrl = convertFileSrc(animal.image_path);
    }
  });

  /** Clears the error message */
  function clearError(): void {
    errorMessage = "";
  }

  /**
   * Sets an error message.
   * @param message - The error message to display
   */
  function setError(message: string): void {
    errorMessage = message;
  }

  /**
   * Validates the form data.
   * @returns boolean - True if form is valid, false otherwise
   */
  function validateForm(): boolean {
    let isValid = true;

    // Reset all invalid flags
    isApplicantNameInvalid = false;
    isApplicantOccupationInvalid = false;
    isApplicantAnnualIncomeInvalid = false;
    isApplicantEmailInvalid = false;
    isApplicantTelNumberInvalid = false;
    isApplicantAddressInvalid = false;
    isApplicantCountryInvalid = false;
    isApplicantStateInvalid = false;
    isNumPeopleInvalid = false;
    isNumChildrenInvalid = false;

    if (!applicantName.trim()) {
      isApplicantNameInvalid = true;
      isValid = false;
    }
    if (!applicantOccupation.trim()) {
      isApplicantOccupationInvalid = true;
      isValid = false;
    }
    if (!applicantAnnualIncome || applicantAnnualIncome === "Pick a range") {
      isApplicantAnnualIncomeInvalid = true;
      isValid = false;
    }
    if (
      !applicantEmail.trim() ||
      !/^[\w-\.]+@([\w-]+\.)+[\w-]{2,4}$/.test(applicantEmail)
    ) {
      isApplicantEmailInvalid = true;
      isValid = false;
    }
    if (!applicantTelNumber.trim()) {
      isApplicantTelNumberInvalid = true;
      isValid = false;
    }
    if (!applicantAddress.trim()) {
      isApplicantAddressInvalid = true;
      isValid = false;
    }
    if (!applicantCountry || applicantCountry === "Pick a country") {
      isApplicantCountryInvalid = true;
      isValid = false;
    }
    if (!applicantState.trim()) {
      isApplicantStateInvalid = true;
      isValid = false;
    }
    if (!numPeople.trim() || isNaN(parseInt(numPeople))) {
      isNumPeopleInvalid = true;
      isValid = false;
    }
    if (!numChildren.trim() || isNaN(parseInt(numChildren))) {
      isNumChildrenInvalid = true;
      isValid = false;
    }

    if (!isValid) {
      setError("Please fill in all required fields correctly.");
    }

    return isValid;
  }

  /** Handles the cancel action */
  function handleCancel(): void {
    goto("/");
  }

  /** Handles the save action */
  async function handleSave(): Promise<void> {
    clearError();
    hasAttemptedSave = true;

    if (!validateForm()) {
      return;
    }

    isSaving = true;

    try {
      const adoptionRequest: AdoptionRequest = {
        id: "", // Will be set by backend
        animal_id: animal.id,
        name: applicantName.trim(),
        email: applicantEmail.trim(),
        tel_number: applicantTelNumber.trim(),
        address: applicantAddress.trim(),
        occupation: applicantOccupation.trim(),
        annual_income: applicantAnnualIncome,
        num_people: parseInt(numPeople) || 0,
        num_children: parseInt(numChildren) || 0,
        country: applicantCountry,
        status: RequestStatus.PENDING,
        request_timestamp: Math.floor(Date.now() / 1000),
        adoption_timestamp: 0,
      };

      info(`Creating adoption request: ${JSON.stringify(adoptionRequest)}`);
      await createAdoptionRequest(adoptionRequest);
      goto("/");
    } catch (e) {
      error(`Failed to send adoption request: ${e}`);
      setError("Failed to send adoption request. Please try again.");
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="container">
  <h1 class="page-title">Send Adoption Request</h1>

  <div class="content-wrapper">
    <!-- Animal Info Section -->
    <div class="animal-info-section">
      <h2>{animal.name}</h2>
      <div class="animal-image">
        {#if imageUrl}
          <img src={imageUrl} alt="Photo of {animal.name}" />
        {/if}
      </div>
      <div class="animal-details">
        <div class="detail-row">
          <div class="detail-item">
            <span class="label">Specie</span>
            <span class="value">{animal.specie}</span>
          </div>
          <div class="detail-item">
            <span class="label">Breed</span>
            <span class="value">{animal.breed}</span>
          </div>
        </div>
        <div class="detail-row">
          <div class="detail-item">
            <span class="label">Sex</span>
            <span class="value">{animal.sex}</span>
          </div>
          <div class="detail-item">
            <span class="label">Birth Month/Year</span>
            <span class="value">
              {animal.birth_year && animal.birth_month
                ? `${animal.birth_month}/${animal.birth_year} (${calculateAge(
                    animal.birth_year,
                    animal.birth_month,
                  )})`
                : "Unknown"}
            </span>
          </div>
        </div>
        <div class="detail-item">
          <span class="label">Appearance</span>
          <span class="value">{animal.appearance}</span>
        </div>
      </div>
    </div>

    <!-- Form Section -->
    <div class="form-section">
      <div class="form-fields">
        <div class="form-row-3-col">
          <FormTextField
            label="Name"
            placeholder="Enter name"
            bind:value={applicantName}
            boxWidth="100%"
            rows={1}
            isInvalid={hasAttemptedSave && isApplicantNameInvalid}
          />
          <FormTextField
            label="Occupation"
            placeholder="Enter occupation"
            bind:value={applicantOccupation}
            boxWidth="100%"
            rows={1}
            isInvalid={hasAttemptedSave && isApplicantOccupationInvalid}
          />
          <FormDropdownButton
            label="Annual Income"
            placeholder="Pick a range"
            options={INCOME_OPTIONS}
            onSelect={(v) => (applicantAnnualIncome = v)}
            maxOptions={10}
            width="100%"
            isInvalid={hasAttemptedSave && isApplicantAnnualIncomeInvalid}
          />
        </div>
        <div class="form-row">
          <FormTextField
            label="Email Address"
            placeholder="Enter email address"
            bind:value={applicantEmail}
            boxWidth="100%"
            rows={1}
            isInvalid={hasAttemptedSave && isApplicantEmailInvalid}
          />
          <FormTextField
            label="Telephone Number"
            placeholder="Enter telephone number"
            bind:value={applicantTelNumber}
            boxWidth="100%"
            rows={1}
            isInvalid={hasAttemptedSave && isApplicantTelNumberInvalid}
          />
        </div>

        <div class="divider"></div>

        <div class="form-row full-width">
          <FormTextField
            label="Street Address"
            placeholder="Type Here..."
            bind:value={applicantAddress}
            boxWidth="100%"
            rows={4}
            isInvalid={hasAttemptedSave && isApplicantAddressInvalid}
          />
        </div>
        <div class="form-row">
          <FormDropdownButton
            label="Country"
            placeholder="Pick a country"
            options={COUNTRY_OPTIONS}
            onSelect={(v) => (applicantCountry = v)}
            width="100%"
            isInvalid={hasAttemptedSave && isApplicantCountryInvalid}
          />
          <FormTextField
            label="State/Province"
            placeholder="Enter a state/province"
            bind:value={applicantState}
            boxWidth="100%"
            rows={1}
            isInvalid={hasAttemptedSave && isApplicantStateInvalid}
          />
        </div>

        <div class="divider"></div>

        <div class="form-row">
          <FormTextField
            label="How many people do you have in your household?"
            placeholder="Answer the question"
            bind:value={numPeople}
            boxWidth="100%"
            rows={1}
            isInvalid={hasAttemptedSave && isNumPeopleInvalid}
          />
          <FormTextField
            label="Of which, how many are children under 15 years old?"
            placeholder="Answer the question"
            bind:value={numChildren}
            boxWidth="100%"
            rows={1}
            isInvalid={hasAttemptedSave && isNumChildrenInvalid}
          />
        </div>
      </div>
    </div>
  </div>

  <!-- Action Buttons -->
  <div class="action-buttons">
    <GenericButton
      color="#003a62"
      textColor="white"
      text="Cancel"
      icon={X}
      showIcon={true}
      iconPosition="left"
      width="120px"
      disabled={isSaving}
      onclick={handleCancel}
    />
    <GenericButton
      color="#00b047"
      textColor="white"
      text="Save"
      icon={Save}
      showIcon={true}
      iconPosition="left"
      width="120px"
      disabled={isSaving}
      onclick={handleSave}
    />
  </div>

  {#if errorMessage}
    <div class="error-message">
      {errorMessage}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "./style.scss";
</style>
