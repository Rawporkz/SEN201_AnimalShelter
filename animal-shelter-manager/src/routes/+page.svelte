<!--
  +page.svelte

  Demo page to showcase the AnimalInfoRow component with sample animal data.
-->

<script lang="ts">
  import AnimalInfoRow from "$lib/components/AnimalInfoRow/AnimalInfoRow.svelte";
  import AnimalAdoptionInfoRow from "$lib/components/AnimalAdoptionInfoRow/AnimalAdoptionInfoRow.svelte";
  import type { AnimalSummary, AdoptionRequest } from "$lib/utils/animal-utils";
  import { AnimalStatus, RequestStatus } from "$lib/utils/animal-utils";
  import { Eye, X } from "@lucide/svelte";
  import { info } from "@tauri-apps/plugin-log";
  import "./style.scss";

  // Sample animal data for demonstration
  const sampleAnimals: AnimalSummary[] = [
    {
      id: "542312",
      name: "Animal Name 2",
      specie: "Cat",
      breed: "Foldex",
      sex: "Female",
      admission_timestamp: 1726185600, // 14/9/2025 equivalent
      image_path: undefined, // No image to test placeholder
      status: AnimalStatus.REQUESTED,
    },
    {
      id: "A00123",
      name: "Buddy",
      specie: "Dog",
      breed: "Golden Retriever",
      sex: "Male",
      admission_timestamp: 1722556800,
      image_path: "/path/to/nonexistent/image.jpg",
      status: AnimalStatus.AVAILABLE,
    },
    {
      id: "C99887",
      name: "Whiskers",
      specie: "Cat",
      breed: "Siamese",
      sex: "Female",
      admission_timestamp: 1704067200,
      image_path: undefined,
      status: AnimalStatus.ADOPTED,
    },
  ];

  // Sample adoption data for demonstration
  const sampleAdoptionRequests: AdoptionRequest[] = [
    {
      id: "REQ001",
      animal_id: "120312",
      name: "Adopter Name 1",
      email: "someone@gmail.com",
      tel_number: "+1234567890",
      address: "123 Main St, City, State",
      occupation: "Software Engineer",
      annual_income: "$75,000",
      num_people: 2,
      num_children: 0,
      request_timestamp: 1726185600,
      adoption_timestamp: 1729209600,
      status: RequestStatus.APPROVED,
    },
    {
      id: "REQ002",
      animal_id: "542312",
      name: "Adopter Name 2",
      email: "someone@gmail.com",
      tel_number: "+1234567891",
      address: "456 Oak Ave, City, State",
      occupation: "Teacher",
      annual_income: "$55,000",
      num_people: 4,
      num_children: 2,
      request_timestamp: 1726185600,
      adoption_timestamp: 1729209600,
      status: RequestStatus.APPROVED,
    },
    {
      id: "REQ003",
      animal_id: "123121",
      name: "Adopter Name 3",
      email: "someone@gmail.com",
      tel_number: "+1234567892",
      address: "789 Pine Rd, City, State",
      occupation: "Nurse",
      annual_income: "$65,000",
      num_people: 3,
      num_children: 1,
      request_timestamp: 1726185600,
      adoption_timestamp: 1729209600,
      status: RequestStatus.APPROVED,
    },
  ];

  // Sample animals for adoption reports (matching the adoption requests)
  const sampleAdoptionAnimals: AnimalSummary[] = [
    {
      id: "120312",
      name: "Animal Name 1",
      specie: "Dog",
      breed: "Golden Retriever",
      sex: "Male",
      admission_timestamp: 1715731200,
      image_path: undefined,
      status: AnimalStatus.ADOPTED,
    },
    {
      id: "542312",
      name: "Animal Name 2",
      specie: "Cat",
      breed: "Scottish Fold",
      sex: "Female",
      admission_timestamp: 1723852800,
      image_path: undefined,
      status: AnimalStatus.ADOPTED,
    },
    {
      id: "123121",
      name: "Animal Name 3",
      specie: "Dog",
      breed: "Labrador",
      sex: "Male",
      admission_timestamp: 1723852800,
      image_path: undefined,
      status: AnimalStatus.ADOPTED,
    },
  ];

  /**
   * Handles View button click for an animal.
   *
   * @param animalId - ID of the animal being viewed
   */
  function handleView(animalId: string): void {
    info(`View clicked for animal ${animalId}`);
  }

  /**
   * Handles Revoke button click for an animal.
   *
   * @param animalId - ID of the animal being revoked
   */
  function handleRevoke(animalId: string): void {
    info(`Revoke clicked for animal ${animalId}`);
  }
</script>

<svelte:head>
  <title>Animal Components Demo</title>
</svelte:head>

<main class="demo-page">
  <header class="page-header">
    <h1>Animal Components Demo</h1>
    <p>AnimalInfoRow and AnimalAdoptionInfoRow components</p>
  </header>

  <section class="animals-list">
    {#each sampleAnimals as animal (animal.id)}
      <AnimalInfoRow animalSummary={animal} showStatus={true}>
        <button
          class="action-btn view-btn"
          onclick={() => handleView(animal.id)}
          type="button"
          title="View {animal.name}"
        >
          <Eye size={16} />
          View
        </button>

        <button
          class="action-btn revoke-btn"
          onclick={() => handleRevoke(animal.id)}
          type="button"
          title="Revoke {animal.name}"
        >
          <X size={16} />
          Revoke
        </button>
      </AnimalInfoRow>
    {/each}
  </section>

  <section class="adoption-reports-section">
    <h2 class="section-title">Adoption Reports</h2>
    <div class="animals-list">
      {#each sampleAdoptionAnimals as animal, index (animal.id)}
        <AnimalAdoptionInfoRow
          animalSummary={animal}
          adoptionRequest={sampleAdoptionRequests[index]}
        >
          <button
            class="action-btn view-btn"
            onclick={() => handleView(animal.id)}
            type="button"
            title="View {animal.name} adoption report"
          >
            <Eye size={16} />
            View
          </button>
        </AnimalAdoptionInfoRow>
      {/each}
    </div>
  </section>
</main>

<style lang="scss">
  @use "../lib/styles/colors.scss" as colors;
  @use "sass:color";

  .demo-page {
    /* Main page container */
    max-width: 800px;
    margin: 0 auto;
    padding: 24px;
  }

  .page-header {
    /* Header section with title and description */
    text-align: center;
    margin-bottom: 32px;

    h1 {
      font-size: 28px;
      font-weight: 700;
      color: colors.$blue-main;
      margin: 0 0 8px 0;
    }

    p {
      font-size: 16px;
      color: colors.$grey-text;
      margin: 0;
    }
  }

  .adoption-reports-section {
    /* Container for adoption report section */
    margin-top: 48px;
  }

  .section-title {
    /* Section title styling */
    font-size: 24px;
    font-weight: 600;
    color: colors.$blue-main;
    margin: 0 0 24px 0;
    border-bottom: 2px solid colors.$grey-light;
    padding-bottom: 8px;
  }

  .animals-list {
    /* Container for animal info rows */
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .action-btn {
    /* Base styles for action buttons */
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border: none;
    border-radius: 8px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.2s ease-in-out;
    min-width: 80px;
    justify-content: center;

    &:hover {
      transform: translateY(-1px);
      box-shadow: 0 4px 8px rgba(0, 0, 0, 0.15);
    }

    &:active {
      transform: translateY(0);
    }
  }

  .view-btn {
    /* View button styling */
    background-color: colors.$blue-main;
    color: white;

    &:hover {
      background-color: color.adjust(colors.$blue-main, $lightness: -10%);
    }
  }

  .revoke-btn {
    /* Revoke button styling */
    background-color: colors.$red-vibrant;
    color: white;

    &:hover {
      background-color: color.adjust(colors.$red-vibrant, $lightness: -10%);
    }
  }
</style>
