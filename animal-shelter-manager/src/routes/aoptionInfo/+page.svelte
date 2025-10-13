<script lang="ts">
  import AdoptionInfoPopup from "../../lib/components/AdoptionInfoPopup/AdoptionInfoPopup.svelte";
  import { AnimalStatus, RequestStatus } from "../../lib/utils/animal-utils";
  import type { Animal, AdoptionRequest } from "../../lib/utils/animal-utils";

  // Mock animal data
  const mockAnimal: Animal = {
    id: "animal-001",
    name: "Buddy",
    specie: "Dog",
    breed: "Golden Retriever",
    sex: "Male",
    birth_month: 3,
    birth_year: 2020,
    neutered: true,
    admission_timestamp: 1640995200,
    adoption_timestamp: 0,
    status: AnimalStatus.REQUESTED,
    image_path:
      "https://tse4.mm.bing.net/th/id/OIP.l19sly2QKflDHh4YJ0sSHQHaE7?cb=12&rs=1&pid=ImgDetMain&o=7&rm=3",
    appearance: "Golden fur, friendly eyes, medium build",
    bio: "Buddy is a loving and energetic dog who loves to play fetch and go on walks. He's great with children and other pets.",
  };

  // Mock adopter data
  const mockAdopter: AdoptionRequest = {
    id: "request-001",
    animal_id: "animal-001",
    name: "John Smith",
    email: "john.smith@example.com",
    tel_number: "+1-555-0123",
    address: "123 Main Street, Springfield",
    occupation: "Software Engineer",
    annual_income: "$80,000",
    num_people: 3,
    num_children: 1,
    request_timestamp: 1672531200,
    adoption_timestamp: 0,
    status: RequestStatus.PENDING,
    country: "United States",
  };

  // State for popup
  let isPopupOpen = $state(false);

  function openPopup() {
    isPopupOpen = true;
  }

  function closePopup() {
    isPopupOpen = false;
  }
</script>

<div class="test-page">
  <h1>Adoption Info Popup Test</h1>

  <div class="button-container">
    <button class="open-button" onclick={openPopup}>
      Open Adoption Info Popup
    </button>
  </div>

  <AdoptionInfoPopup
    animal={mockAnimal}
    adopter={mockAdopter}
    isOpen={isPopupOpen}
    onClose={closePopup}
  />
</div>

<style lang="scss">
  @use "./style.scss";
</style>
