<!-- 
routes/admit-animal-form/+page.svelte

This file defines the admit animal form page of the application.
Allows staff to admit new animals to the shelter system.
-->

<script lang="ts">
  import { X, ImagePlus, Save } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import FormTextField from "$lib/components/FormTextField/FormTextField.svelte";
  import FormDropdownButton from "$lib/components/FormDropdownButton/FormDropdownButton.svelte";
  import GenericButton from "$lib/components/GenericButton/GenericButton.svelte";
  import {
    ANIMAL_SPECIES_OPTIONS,
    ANIMAL_SEX_OPTIONS,
    NEUTERED_STATUS_OPTIONS,
    getBreedsForSpecies,
  } from "$lib/config/animal-options";
  import {
    uploadAnimalImage,
    createAnimal,
    type Animal,
    AnimalStatus,
  } from "$lib/utils/animal-utils";
  import { info, error } from "@tauri-apps/plugin-log";

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

  /** Flag to indicate if form submission is in progress */
  let isSaving: boolean = $state(false);

  /** Flag to indicate if image upload is in progress */
  let isUploadingImage: boolean = $state(false);

  /** Error message to display */
  let errorMessage: string = $state("");

  /** Generate month options (1-12) */
  const monthOptions: string[] = [
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
    const years: string[] = [];
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
   * Handles species selection and resets breed selection.
   *
   * @param species - The selected species label
   */
  function handleSpeciesSelect(species: string): void {
    selectedSpecies = species;
    selectedBreed = ""; // Reset breed when species changes
    clearError();
  }

  /**
   * Handles breed selection.
   *
   * @param breed - The selected breed
   */
  function handleBreedSelect(breed: string): void {
    selectedBreed = breed;
    clearError();
  }

  /**
   * Handles sex selection.
   *
   * @param sex - The selected sex label
   */
  function handleSexSelect(sex: string): void {
    selectedSex = sex;
    clearError();
  }

  /**
   * Handles neutered status selection.
   *
   * @param status - The selected neutered status label
   */
  function handleNeuteredStatusSelect(status: string): void {
    selectedNeuteredStatus = status;
    clearError();
  }

  /**
   * Handles month selection.
   *
   * @param month - The selected month name
   */
  function handleMonthSelect(month: string): void {
    selectedMonth = month;
    clearError();
  }

  /**
   * Handles year selection.
   *
   * @param year - The selected year
   */
  function handleYearSelect(year: string): void {
    selectedYear = year;
    clearError();
  }

  /**
   * Clears the error message.
   */
  function clearError(): void {
    errorMessage = "";
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
    if (!animalName.trim()) {
      setError("Please enter the animal's name.");
      return false;
    }

    if (!selectedMonth) {
      setError("Please select a birth month.");
      return false;
    }

    if (!selectedYear) {
      setError("Please select a birth year.");
      return false;
    }

    if (!selectedSpecies) {
      setError("Please select a species.");
      return false;
    }

    if (!selectedBreed) {
      setError("Please select a breed.");
      return false;
    }

    if (!selectedSex) {
      setError("Please select a sex.");
      return false;
    }

    if (!selectedNeuteredStatus) {
      setError("Please select a neutered status.");
      return false;
    }

    if (!animalAppearance.trim()) {
      setError("Please describe the animal's appearance.");
      return false;
    }

    if (!animalBio.trim()) {
      setError("Please provide bio and characteristics.");
      return false;
    }

    return true;
  }

  /**
   * Handles the cancel action.
   */
  async function handleCancel(): Promise<void> {
    goto("/");
  }

  /**
   * Handles the save action.
   */
  async function handleSave(): Promise<void> {
    clearError();

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

      const monthNumber = monthOptions.indexOf(selectedMonth) + 1;
      const yearNumber = parseInt(selectedYear);

      // Create animal object
      const animal: Animal = {
        id: "", // Will be set by backend
        name: animalName.trim(),
        specie: speciesValue,
        breed: selectedBreed,
        sex: sexValue,
        birth_month: monthNumber,
        birth_year: yearNumber,
        neutered: neuteredValue === "yes",
        admission_timestamp: Math.floor(Date.now() / 1000), // Current timestamp in seconds
        status: AnimalStatus.AVAILABLE,
        image_path: imagePath!,
        appearance: animalAppearance.trim(),
        bio: animalBio.trim(),
      };
      info(`Creating animal: ${JSON.stringify(animal)}`);
      await createAnimal(animal);
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
  <h1 class="page-title">Admit Animal</h1>

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
            <img src={imagePath} alt="Animal" class="uploaded-image" />
          {:else}
            <div class="image-placeholder">
              <ImagePlus size={48} color="#999" />
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
            />
          </div>
          <div class="form-field-right">
            <div class="birthdate-fields">
              <FormDropdownButton
                options={monthOptions}
                placeholder="Month"
                width="100%"
                label="Birthdate"
                onSelect={handleMonthSelect}
              />
              <FormDropdownButton
                options={yearOptions}
                placeholder="Year"
                width="100%"
                label="‎"
                onSelect={handleYearSelect}
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
              onSelect={handleSpeciesSelect}
            />
          </div>
          <div class="form-field-right">
            <FormDropdownButton
              options={getBreedOptions()}
              placeholder="Pick breeds"
              width="100%"
              label="Breed"
              onSelect={handleBreedSelect}
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
              onSelect={handleSexSelect}
            />
          </div>
          <div class="form-field-right">
            <FormDropdownButton
              options={neuteredStatusOptions}
              placeholder="Pick a neutured status"
              width="100%"
              label="Neutured Status"
              onSelect={handleNeuteredStatusSelect}
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

  <!-- Error Message -->
  {#if errorMessage}
    <div class="error-message">
      {errorMessage}
    </div>
  {/if}
</div>

<style lang="scss">
  @use "./style.scss";
</style>
