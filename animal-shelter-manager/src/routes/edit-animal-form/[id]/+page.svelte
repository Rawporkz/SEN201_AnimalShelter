<!-- 
routes/edit-animal-form/[id]/+page.svelte

This file defines the edit animal form page of the application.
Allows staff to edit existing animals in the shelter system.
-->

<script lang="ts">
  import { onMount } from "svelte";
  import { X, ImagePlus, Save, Trash2 } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import { convertFileSrc } from "@tauri-apps/api/core";
  import type { PageData } from "./$types";
  import FormTextField from "$lib/components/FormTextField/FormTextField.svelte";
  import FormDropdownButton from "$lib/components/FormDropdownButton/FormDropdownButton.svelte";
  import GenericButton from "$lib/components/GenericButton/GenericButton.svelte";
  import ConfirmationModal from "$lib/components/ConfirmationModal/ConfirmationModal.svelte";
  import {
    ANIMAL_SPECIES_OPTIONS,
    ANIMAL_SEX_OPTIONS,
    NEUTERED_STATUS_OPTIONS,
    getBreedsForSpecies,
  } from "$lib/config/animal-options";
  import {
    uploadAnimalImage,
    updateAnimal,
    deleteAnimal,
    type Animal,
    AnimalStatus,
    getAdoptionRequestsByAnimalId,
    RequestStatus,
    updateAdoptionRequest,
  } from "$lib/utils/data-utils";
  import { info, error } from "@tauri-apps/plugin-log";

  const { data }: { data: PageData } = $props();
  const animal = data.animal;

  /** Animal name entered by user */
  let animalName: string = $state("");

  /** Selected birth month (1-12) */
  let selectedMonth: string = $state("");

  /** Selected birth year */
  let selectedYear: string = $state("");

  /** Selected animal species */
  let selectedSpecies: string = $state("");

  /** Selected animal breed */
  let selectedBreed: string = $state("");

  /** Selected animal sex */
  let selectedSex: string = $state("");

  /** Selected neutered status */
  let selectedNeuteredStatus: string = $state("");

  /** Appearance description of the animal */
  let animalAppearance: string = $state("");

  /** Bio and characteristics of the animal */
  let animalBio: string = $state("");

  /** Path to uploaded animal image */
  let imagePath: string | null = $state(null);

  /** URL for the uploaded image, derived from imagePath */
  let imageUrl: string | null = $derived(
    imagePath ? convertFileSrc(imagePath) : null,
  );

  /** Flag to indicate if form submission is in progress */
  let isSaving: boolean = $state(false);

  /** Flag to indicate if image upload is in progress */
  let isUploadingImage: boolean = $state(false);

  /** Validity states for form fields */
  let isAnimalNameInvalid: boolean = $state(false);
  let isSelectedMonthInvalid: boolean = $state(false);
  let isSelectedYearInvalid: boolean = $state(false);
  let isSelectedSpeciesInvalid: boolean = $state(false);
  let isSelectedBreedInvalid: boolean = $state(false);
  let isSelectedSexInvalid: boolean = $state(false);
  let isSelectedNeuteredStatusInvalid: boolean = $state(false);
  let isAnimalAppearanceInvalid: boolean = $state(false);
  let isAnimalBioInvalid: boolean = $state(false);

  /** Flag to indicate if the user has attempted to save the form */
  let hasAttemptedSave: boolean = $state(false);

  /** Error message to display */
  let errorMessage: string = $state("");

  /** Whether the delete confirmation modal is open. */
  let isDeleteModalOpen: boolean = $state(false);

  onMount(() => {
    if (animal) {
      animalName = animal.name;
      selectedMonth = monthOptions[animal.birthMonth || 0];
      selectedYear = animal.birthYear?.toString() || "Unknown";
      selectedSpecies = animal.specie;
      selectedBreed = animal.breed;
      selectedSex = animal.sex;
      selectedNeuteredStatus = animal.neutered ? "Yes" : "No";
      animalAppearance = animal.appearance;
      animalBio = animal.bio;
      imagePath = animal.imagePath || null;
    }
  });

  /** Generate month options (1-12) */
  const monthOptions: string[] = [
    "Unknown",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
  ];

  /** Generate year options (current year back to 1990) */
  const yearOptions: string[] = (() => {
    const currentYear = new Date().getFullYear();
    const years: string[] = ["Unknown"];
    for (let year = currentYear; year >= 1990; year--) {
      years.push(year.toString());
    }
    return years;
  })();

  /** Get species options as strings */
  const speciesOptions: string[] = ANIMAL_SPECIES_OPTIONS.map(
    (option) => option.label,
  );

  /** Get sex options as strings */
  const sexOptions: string[] = ANIMAL_SEX_OPTIONS.map((option) => option.label);

  /** Get neutered status options as strings */
  const neuteredStatusOptions: string[] = NEUTERED_STATUS_OPTIONS.map(
    (option) => option.label,
  );

  /** Get breed options based on selected species */
  function getBreedOptions(): string[] {
    if (!selectedSpecies) return [];

    // Find the species value from the selected label
    const speciesOption = ANIMAL_SPECIES_OPTIONS.find(
      (option) => option.label === selectedSpecies,
    );
    if (!speciesOption) return [];

    return getBreedsForSpecies(speciesOption.value);
  }

  /**
   * Clears the error message.
   */
  function clearError(): void {
    errorMessage = "";
  }

  /**
   * Handles input changes to clear the error message.
   */
  function handleInputChange(): void {
    clearError();
  }

  /**
   * Sets an error message.
   *
   * @param message - The error message to display
   */
  function setError(message: string): void {
    errorMessage = message;
  }

  /**
   * Handles the image upload action.
   */
  async function handleImageUpload(): Promise<void> {
    isUploadingImage = true;
    clearError();

    try {
      const uploadedPath = await uploadAnimalImage();
      if (uploadedPath) {
        imagePath = uploadedPath;
        info(`Image uploaded successfully: ${uploadedPath}`);
      }
    } catch (e) {
      error(`Failed to upload image: ${e}`);
      setError("Failed to upload image. Please try again.");
    } finally {
      isUploadingImage = false;
    }
  }

  /**
   * Validates the form data.
   *
   * @returns boolean - True if form is valid, false otherwise
   */
  function validateForm(): boolean {
    let isValid = true;

    // Reset all invalid flags
    isAnimalNameInvalid = false;
    isSelectedMonthInvalid = false;
    isSelectedYearInvalid = false;
    isSelectedSpeciesInvalid = false;
    isSelectedBreedInvalid = false;
    isSelectedSexInvalid = false;
    isSelectedNeuteredStatusInvalid = false;
    isAnimalAppearanceInvalid = false;
    isAnimalBioInvalid = false;

    if (!animalName.trim()) {
      isAnimalNameInvalid = true;
      isValid = false;
    }

    if (!selectedMonth) {
      isSelectedMonthInvalid = true;
      isValid = false;
    }

    if (!selectedYear) {
      isSelectedYearInvalid = true;
      isValid = false;
    }

    if (!selectedSpecies) {
      isSelectedSpeciesInvalid = true;
      isValid = false;
    }

    if (!selectedBreed) {
      isSelectedBreedInvalid = true;
      isValid = false;
    }

    if (!selectedSex) {
      isSelectedSexInvalid = true;
      isValid = false;
    }

    if (!selectedNeuteredStatus) {
      isSelectedNeuteredStatusInvalid = true;
      isValid = false;
    }

    if (!animalAppearance.trim()) {
      isAnimalAppearanceInvalid = true;
      isValid = false;
    }

    if (!animalBio.trim()) {
      isAnimalBioInvalid = true;
      isValid = false;
    }

    if (!isValid) {
      setError("Please fill in all required fields correctly.");
    }

    return isValid;
  }

  /**
   * Handles the cancel action.
   */
  async function handleCancel(): Promise<void> {
    goto("/");
  }

  let deleteReason: string = $state("passed-away");
  let otherReasonText: string = $state("");

  /**
   * Opens the delete confirmation modal.
   */
  function handleDelete(): void {
    isDeleteModalOpen = true;
  }

  /**
   * Confirms and executes the delete or update operation based on the selected reason.
   */
  async function confirmDelete(): Promise<void> {
    if (deleteReason === "passed-away") {
      // Update the animal status to PASSED_AWAY
      animal.status = AnimalStatus.PASSED_AWAY;
      await updateAnimal(animal);
    } else {
      // Delete the animal record
      await deleteAnimal(animal);
    }

    // Reject all requests associated with the animal
    let requests = await getAdoptionRequestsByAnimalId(animal.id);
    for (const request of requests) {
      request.status = RequestStatus.REJECTED;
      await updateAdoptionRequest(request);
    }

    // Navigate back
    goto("/");
  }

  /**
   * Handles the save action.
   */
  async function handleSave(): Promise<void> {
    clearError();
    hasAttemptedSave = true;

    if (!validateForm()) {
      return;
    }

    isSaving = true;

    try {
      // Convert selected values back to backend format
      const speciesValue = ANIMAL_SPECIES_OPTIONS.find(
        (option) => option.label === selectedSpecies,
      )?.value;
      const sexValue = ANIMAL_SEX_OPTIONS.find(
        (option) => option.label === selectedSex,
      )?.value;
      const neuteredValue = NEUTERED_STATUS_OPTIONS.find(
        (option) => option.label === selectedNeuteredStatus,
      )?.value;

      if (!speciesValue || !sexValue || !neuteredValue) {
        setError("Invalid form data. Please check your selections.");
        return;
      }

      const monthNumber =
        selectedMonth === "Unknown"
          ? null
          : monthOptions.indexOf(selectedMonth);
      const yearNumber =
        selectedYear === "Unknown" ? null : parseInt(selectedYear);

      const animalToUpdate: Animal = {
        ...animal,
        name: animalName.trim(),
        specie: speciesValue,
        breed: selectedBreed,
        sex: sexValue,
        birthMonth: monthNumber,
        birthYear: yearNumber,
        neutered: neuteredValue === "yes",
        imagePath: imagePath!,
        appearance: animalAppearance.trim(),
        bio: animalBio.trim(),
      };
      info(`Updating animal: ${JSON.stringify(animalToUpdate)}`);
      await updateAnimal(animalToUpdate);
      goto("/");
    } catch (e) {
      error(`Failed to admit animal: ${e}`);
      setError("Failed to admit animal. Please try again.");
    } finally {
      isSaving = false;
    }
  }
</script>

<div class="container">
  <h1 class="page-title">Edit Animal</h1>

  <div class="form-content">
    <div class="left-section">
      <div class="image-upload-area">
        <button
          type="button"
          class="image-upload-button"
          onclick={handleImageUpload}
          disabled={isUploadingImage}
        >
          {#if imagePath}
            <img src={imageUrl} alt="Animal" class="uploaded-image" />
          {:else}
            <div class="image-placeholder">
              <ImagePlus size={48} />
              <span class="upload-text">
                {isUploadingImage ? "Uploading..." : "Upload Image"}
              </span>
            </div>
          {/if}
        </button>
      </div>
    </div>

    <div class="right-section">
      <div class="form-fields">
        <!-- Name and Birthdate Row -->
        <div class="form-row">
          <div class="form-field-left">
            <FormTextField
              label="Name"
              placeholder="Enter Name"
              bind:value={animalName}
              boxWidth="100%"
              rows={1}
              isInvalid={hasAttemptedSave && isAnimalNameInvalid}
              oninput={handleInputChange}
            />
          </div>
          <div class="form-field-right">
            <div class="birthdate-fields">
              <FormDropdownButton
                options={monthOptions}
                placeholder="Month"
                width="100%"
                label="Birthdate"
                bind:value={selectedMonth}
                isInvalid={hasAttemptedSave && isSelectedMonthInvalid}
                onchange={handleInputChange}
              />
              <FormDropdownButton
                options={yearOptions}
                placeholder="Year"
                width="100%"
                label="‎"
                bind:value={selectedYear}
                isInvalid={hasAttemptedSave && isSelectedYearInvalid}
                onchange={handleInputChange}
              />
            </div>
          </div>
        </div>

        <!-- Species and Breed Row -->
        <div class="form-row">
          <div class="form-field-left">
            <FormDropdownButton
              options={speciesOptions}
              placeholder="Pick a species"
              width="100%"
              label="Species"
              bind:value={selectedSpecies}
              isInvalid={hasAttemptedSave && isSelectedSpeciesInvalid}
              onchange={handleInputChange}
            />
          </div>
          <div class="form-field-right">
            <FormDropdownButton
              options={getBreedOptions()}
              placeholder="Pick breeds"
              width="100%"
              label="Breed"
              bind:value={selectedBreed}
              disabled={!selectedSpecies}
              resetOn={selectedSpecies}
              isInvalid={hasAttemptedSave && isSelectedBreedInvalid}
              onchange={handleInputChange}
            />
          </div>
        </div>

        <!-- Sex and Neutered Status Row -->
        <div class="form-row">
          <div class="form-field-left">
            <FormDropdownButton
              options={sexOptions}
              placeholder="Pick a sex"
              width="100%"
              label="Sex"
              bind:value={selectedSex}
              isInvalid={hasAttemptedSave && isSelectedSexInvalid}
              onchange={handleInputChange}
            />
          </div>
          <div class="form-field-right">
            <FormDropdownButton
              options={neuteredStatusOptions}
              placeholder="Pick a neutured status"
              width="100%"
              label="Neutured Status"
              bind:value={selectedNeuteredStatus}
              isInvalid={hasAttemptedSave && isSelectedNeuteredStatusInvalid}
              onchange={handleInputChange}
            />
          </div>
        </div>

        <!-- Appearance Field -->
        <div class="form-row full-width">
          <FormTextField
            label="Appearance"
            placeholder="Type Here..."
            bind:value={animalAppearance}
            boxWidth="100%"
            rows={4}
            isInvalid={hasAttemptedSave && isAnimalAppearanceInvalid}
            oninput={handleInputChange}
          />
        </div>

        <!-- Bio and Characteristics Field -->
        <div class="form-row full-width">
          <FormTextField
            label="Bio and Characteristics"
            placeholder="Type Here..."
            bind:value={animalBio}
            boxWidth="100%"
            rows={4}
            isInvalid={hasAttemptedSave && isAnimalBioInvalid}
            oninput={handleInputChange}
          />
        </div>
      </div>
    </div>
  </div>

  <div class="action-buttons">
    <div>
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
    </div>
    <div class="save-cancel-buttons">
      <GenericButton
        color="#ea4444"
        textColor="white"
        text="Delete"
        icon={Trash2}
        showIcon={true}
        iconPosition="left"
        width="120px"
        disabled={isSaving}
        onclick={handleDelete}
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
  </div>

  <!-- Error Message -->
  {#if errorMessage}
    <div class="error-message">
      {errorMessage}
    </div>
  {/if}

  <ConfirmationModal
    bind:open={isDeleteModalOpen}
    title="Deleting an Animal"
    message={`Please provide a reason for the deletion of ${animal.name}.`}
    width="500px"
    contentWidth="400px"
    confirmText={deleteReason === "passed-away" ? "Change Status" : "Confirm"}
    cancelText="Cancel"
    destructive={deleteReason !== "passed-away"}
    confirmColor={deleteReason === "passed-away" ? "black" : "#ea4444"}
    onconfirm={confirmDelete}
  >
    {#snippet extra()}
      <div class="delete-reason-options">
        <label>
          <input type="radio" bind:group={deleteReason} value="passed-away" />
          The animal has passed away
        </label>
        <label>
          <input type="radio" bind:group={deleteReason} value="mistake" />
          The animal was added by mistake
        </label>
        <label>
          <input type="radio" bind:group={deleteReason} value="other" />
          Other
        </label>
        {#if deleteReason === "other"}
          <FormTextField
            label=""
            placeholder="Type Here..."
            bind:value={otherReasonText}
            boxWidth="100%"
          />
        {/if}
      </div>
    {/snippet}
  </ConfirmationModal>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
