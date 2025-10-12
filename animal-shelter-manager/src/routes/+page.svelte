<!--
  +page.svelte

  Demo page to showcase the AnimalInfoRow component with sample animal data.
-->

<script lang="ts">
  import AnimalInfoRow from "$lib/components/AnimalInfoRow/AnimalInfoRow.svelte";
  import type { AnimalSummary } from "$lib/utils/animal-utils";
  import { AnimalStatus } from "$lib/utils/animal-utils";
  import { Eye, X } from "@lucide/svelte";

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
      admission_timestamp: 1722556800, // 1/8/2024 equivalent
      image_path: "/path/to/nonexistent/image.jpg", // Invalid path to test error handling
      status: AnimalStatus.AVAILABLE,
    },
    {
      id: "C99887",
      name: "Whiskers",
      specie: "Cat",
      breed: "Siamese",
      sex: "Female",
      admission_timestamp: 1704067200, // 1/1/2024 equivalent
      image_path: undefined, // No image to test placeholder
      status: AnimalStatus.ADOPTED,
    },
  ];

  /**
   * Handles View button click for an animal.
   *
   * @param animalId - ID of the animal being viewed
   */
  function handleView(animalId: string): void {
    console.log(`View clicked for animal ${animalId}`);
  }

  /**
   * Handles Revoke button click for an animal.
   *
   * @param animalId - ID of the animal being revoked
   */
  function handleRevoke(animalId: string): void {
    console.log(`Revoke clicked for animal ${animalId}`);
  }
</script>

<svelte:head>
  <title>Animal Info Row Demo</title>
</svelte:head>

<main class="demo-page">
  <header class="page-header">
    <h1>AnimalInfoRow Component Demo</h1>
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
</main>

<style lang="scss">
  @use "../lib/styles/colors.scss" as colors;
  @use "sass:color";

  .demo-page {
    /* Main page container */
    max-width: 800px;
    margin: 0 auto;
    padding: 24px;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto,
      sans-serif;
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

  /* Responsive design for smaller screens */
  @media (max-width: 600px) {
    .demo-page {
      padding: 16px;
    }

    .page-header h1 {
      font-size: 24px;
    }

    .animals-list {
      gap: 12px;
    }
  }
</style>
