/**
 * lib/utils/data-utils.ts
 *
 * This module contains TypeScript types and utility functions for animal-related
 * operations, including invoking Tauri backend functions for animal management.
 */

import { invoke } from "@tauri-apps/api/core";
import { error } from "@tauri-apps/plugin-log";

// ==================== ENUMS ====================

/** Status of an animal in the shelter system */
export enum AnimalStatus {
  /** Animal is available for adoption */
  AVAILABLE = "available",
  /** Animal has been requested for adoption but not yet approved */
  REQUESTED = "requested",
  /** Animal has been successfully adopted */
  ADOPTED = "adopted",
  /** Animal has passed away */
  PASSED_AWAY = "passed-away",
}

/** Status of an adoption request in the system */
export enum RequestStatus {
  /** Request is pending review */
  PENDING = "pending",
  /** Request has been rejected */
  REJECTED = "rejected",
  /** Request has been approved */
  APPROVED = "approved",
}

// ==================== INTERFACES ====================

/** Represents an animal in the shelter system */
export interface Animal {
  /** Unique identifier for the animal */
  id: string;
  /** Name of the animal */
  name: string;
  /** Species of the animal (e.g., "Dog", "Cat") */
  specie: string;
  /** Breed of the animal */
  breed: string;
  /** Sex of the animal (e.g., "Male", "Female") */
  sex: string;
  /** Birth month of the animal (1-12) */
  birthMonth: number | null;
  /** Birth year of the animal */
  birthYear: number | null;
  /** Whether the animal has been neutered */
  neutered: boolean;
  /** Timestamp when the animal was admitted to the shelter */
  admissionTimestamp: number;
  /** Current status of the animal */
  status: AnimalStatus;
  /** Path to the animal's image file */
  imagePath?: string;
  /** Appearance description of the animal */
  appearance: string;
  /** Bio & Characteristics of the animal */
  bio: string;
}

/** Simplified animal information for listing views */
export interface AnimalSummary {
  /** Unique identifier for the animal */
  id: string;
  /** Name of the animal */
  name: string;
  /** Species of the animal */
  specie: string;
  /** Breed of the animal */
  breed: string;
  /** Sex of the animal */
  sex: string;
  /** Timestamp when the animal was admitted to the shelter */
  admissionTimestamp: number;
  /** Path to the animal's image file */
  imagePath?: string;
  /** Current status of the animal */
  status: AnimalStatus;
}

/** Represents an adoption request in the system */
export interface AdoptionRequest {
  /** Unique identifier for the adoption request */
  id: string;
  /** ID of the animal being requested for adoption */
  animalId: string;
  /** Username of the user who made the request */
  username: string;
  /** Full name of the person requesting adoption */
  name: string;
  /** Email address of the requester */
  email: string;
  /** Telephone number of the requester */
  telNumber: string;
  /** Address of the requester */
  address: string;
  /** Occupation of the requester */
  occupation: string;
  /** Annual income of the requester */
  annualIncome: string;
  /** Number of people in the household */
  numPeople: number;
  /** Number of children in the household */
  numChildren: number;
  /** Timestamp when the request was submitted */
  requestTimestamp: number;
  /** Timestamp when the adoption was completed (0 if not completed) */
  adoptionTimestamp: number;
  /** Current status of the request */
  status: RequestStatus;
  /** Country of the requester */
  country: string;
}

import type { FilterSelections } from "$lib/utils/filter-utils";

// ==================== ANIMAL FUNCTIONS ====================

/**
 * Retrieves animals from the database, with optional filtering.
 *
 * @param filters - Optional map of filter criteria and values
 * @returns Promise<AnimalSummary[]> - List of animal summaries. Returns an empty array if the operation fails.
 */
export async function getAnimals(
  filters: FilterSelections | null,
): Promise<AnimalSummary[]> {
  try {
    return await invoke<AnimalSummary[]>("get_animals", { filters });
  } catch (e) {
    error(`Failed to get animals: ${e}`);
    return [];
  }
}

/**
 * Retrieves a specific animal by ID.
 *
 * @param animalId - The ID of the animal to retrieve
 * @returns Promise<Animal | null> - The animal data if found, null if not found. Returns null if the operation fails.
 */
export async function getAnimalById(animalId: string): Promise<Animal | null> {
  try {
    return await invoke<Animal | null>("get_animal_by_id", { animalId });
  } catch (e) {
    error(`Failed to get animal by ID ${animalId}: ${e}`);
    return null;
  }
}

/**
 * Retrieves an animal and its associated accepted adoption in parallel.
 *
 * @param animalId - The ID of the animal.
 * @param animalStatus - The status of the animal, to determine if a request should be fetched.
 * @returns A promise that resolves to an object containing the animal and optional adopter details. Returns `{ animal: null, adopter: null }` if the operation fails.
 */
export async function getAnimalWithAcceptedAdoption(
  animalId: string,
  animalStatus: AnimalStatus,
): Promise<{ animal: Animal | null; adopter: AdoptionRequest | null }> {
  try {
    const animalPromise = getAnimalById(animalId);
    let adopterPromise: Promise<AdoptionRequest | null> = Promise.resolve(null);

    if (animalStatus === AnimalStatus.ADOPTED) {
      adopterPromise = getAdoptionRequestsByAnimalId(animalId).then(
        (requests) => {
          // Find the request with the APPROVED status
          const foundApprovedRequest = requests.find(
            (request) => request && request.status === RequestStatus.APPROVED,
          );
          return foundApprovedRequest || null;
        },
      );
    }

    const [animal, adopter] = await Promise.all([
      animalPromise,
      adopterPromise,
    ]);
    return { animal, adopter };
  } catch (e) {
    error(`Failed to get animal ${animalId} with adoption request: ${e}`);
    return { animal: null, adopter: null };
  }
}

/**
 * Creates a new animal in the database.
 *
 * @param animal - The animal data to create
 * @returns Promise<void> - A promise that resolves when the operation is complete. Logs an error if the operation fails.
 */
export async function createAnimal(animal: Animal): Promise<void> {
  try {
    await invoke("create_animal", { animal });
  } catch (e) {
    error(`Failed to create animal: ${e}`);
  }
}

/**
 * Updates an existing animal in the database.
 *
 * @param animal - The updated animal data
 * @returns Promise<boolean> - True if animal was found and updated, false if not found. Returns false if the operation fails.
 */
export async function updateAnimal(animal: Animal): Promise<boolean> {
  try {
    return await invoke<boolean>("update_animal", { animal });
  } catch (e) {
    error(`Failed to update animal: ${e}`);
    return false;
  }
}

/**
 * Deletes an animal from the database.
 *
 * @param animal - The animal data to delete
 * @returns Promise<boolean> - True if animal was found and deleted, false if not found. Returns false if the operation fails.
 */
export async function deleteAnimal(
  animal: AnimalSummary | Animal,
): Promise<boolean> {
  try {
    const animalId = animal.id;

    // Delete animal from database
    const deleteStatus = await invoke<boolean>("delete_animal", { animalId });

    // Remove file associated with the animal if the deletion was successful
    if (deleteStatus) {
      await invoke<boolean>("delete_animal", { animalId });
      return true;
    }

    return false;
  } catch (e) {
    error(`Failed to delete animal with ID ${animal.id}: ${e}`);
    return false;
  }
}

// ==================== ADOPTION REQUEST FUNCTIONS ====================

/**
 * Retrieves a specific adoption request by ID.
 *
 * @param requestId - The ID of the adoption request to retrieve
 * @returns Promise<AdoptionRequest | null> - The adoption request data if found, null if not found. Returns null if the operation fails.
 */
export async function getAdoptionRequestById(
  requestId: string,
): Promise<AdoptionRequest | null> {
  try {
    return await invoke<AdoptionRequest | null>("get_adoption_request_by_id", {
      requestId,
    });
  } catch (e) {
    error(`Failed to get adoption request by ID ${requestId}: ${e}`);
    return null;
  }
}

/**
 * Retrieves all adoption requests for a specific animal ID.
 *
 * @param animalId - The ID of the animal to retrieve requests for
 * @returns Promise<AdoptionRequest[]> - List of adoption requests. Returns an empty array if the operation fails.
 */
export async function getAdoptionRequestsByAnimalId(
  animalId: string,
): Promise<AdoptionRequest[]> {
  try {
    return await invoke<AdoptionRequest[]>(
      "get_adoption_requests_by_animal_id",
      { animalId },
    );
  } catch (e) {
    error(`Failed to get adoption requests for animal ID ${animalId}: ${e}`);
    return [];
  }
}

/**
 * Retrieves all adoption requests for a specific username.
 *
 * @param username - The username to retrieve requests for
 * @returns Promise<AdoptionRequest[]> - List of adoption requests. Returns an empty array if the operation fails.
 */
export async function getAdoptionRequestsByUsername(
  username: string,
): Promise<AdoptionRequest[]> {
  try {
    return await invoke<AdoptionRequest[]>(
      "get_adoption_requests_by_username",
      { username },
    );
  } catch (e) {
    error(`Failed to get adoption requests for username ${username}: ${e}`);
    return [];
  }
}

/**
 * Creates a new adoption request in the database.
 *
 * @param request - The adoption request data to create
 * @returns Promise<void> - A promise that resolves when the operation is complete. Logs an error if the operation fails.
 */
export async function createAdoptionRequest(
  request: AdoptionRequest,
): Promise<void> {
  try {
    await invoke("create_adoption_request", { request });
  } catch (e) {
    error(`Failed to create adoption request: ${e}`);
  }
}

/**
 * Updates an existing adoption request in the database.
 *
 * @param request - The updated adoption request data
 * @returns Promise<boolean> - True if request was found and updated, false if not found. Returns false if the operation fails.
 */
export async function updateAdoptionRequest(
  request: AdoptionRequest,
): Promise<boolean> {
  try {
    return await invoke<boolean>("update_adoption_request", { request });
  } catch (e) {
    error(`Failed to update adoption request: ${e}`);
    return false;
  }
}

/**
 * Deletes an adoption request from the database.
 *
 * @param requestId - The ID of the adoption request to delete
 * @returns Promise<boolean> - True if request was found and deleted, false if not found. Returns false if the operation fails.
 */
export async function deleteAdoptionRequest(
  requestId: string,
): Promise<boolean> {
  try {
    return await invoke<boolean>("delete_adoption_request", { requestId });
  } catch (e) {
    error(`Failed to delete adoption request with ID ${requestId}: ${e}`);
    return false;
  }
}

// ==================== FILE FUNCTIONS ====================

/**
 * Uploads a file selected by the user for animal images.
 *
 * @returns Promise<string | null> - The path of the uploaded file if successful, null if canceled. Returns null if the upload fails.
 */
export async function uploadAnimalImage(): Promise<string | null> {
  try {
    return await invoke<string | null>("upload_file");
  } catch (e) {
    error(`Failed to upload animal image: ${e}`);
    return null;
  }
}

/**
 * Deletes a file from the specified path.
 *
 * @param filePath - The path of the file to be deleted
 * @returns Promise<void> - A promise that resolves when the operation is complete. Logs an error if the deletion fails.
 */
export async function deleteFile(filePath: string): Promise<void> {
  try {
    await invoke("delete_file", { filePath });
  } catch (e) {
    error(`Failed to delete file: ${e}`);
  }
}

// ==================== UTILITY FUNCTIONS ====================

/**
 * Formats a timestamp to a readable date string.
 *
 * @param timestamp - Unix timestamp in seconds
 * @returns string - Formatted date string (DD/MM/YYYY)
 */
export function formatTimestamp(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const day = date.getDate().toString().padStart(2, "0");
  const month = (date.getMonth() + 1).toString().padStart(2, "0");
  const year = date.getFullYear();
  return `${day}/${month}/${year}`;
}

/**
 * Calculates age from birth year and month.
 *
 * @param birthYear - Birth year of the animal
 * @param birthMonth - Birth month of the animal (1-12)
 * @returns string - Age in years and months
 */
export function calculateAge(
  birthYear: number | null,
  birthMonth: number | null,
): string | null {
  if (!birthYear || !birthMonth) {
    return null;
  }
  const now = new Date();
  const currentYear = now.getFullYear();
  const currentMonth = now.getMonth() + 1;

  let ageYears = currentYear - birthYear;
  let ageMonths = currentMonth - birthMonth;

  if (ageMonths < 0) {
    ageYears -= 1;
    ageMonths += 12;
  }

  if (ageYears === 0) {
    return `${ageMonths} m old`;
  } else if (ageMonths === 0) {
    return `${ageYears} y old`;
  } else {
    return `${ageYears} y ${ageMonths} m old`;
  }
}

/**
 * Gets the display text for an animal status.
 *
 * @param status - The animal status
 * @returns string - Human-readable status text
 */
export function getStatusDisplayText(status: AnimalStatus): string {
  switch (status) {
    case AnimalStatus.AVAILABLE:
      return "Available";
    case AnimalStatus.REQUESTED:
      return "Requested";
    case AnimalStatus.ADOPTED:
      return "Adopted";
    case AnimalStatus.PASSED_AWAY:
      return "Passed Away";
    default:
      return "Unknown";
  }
}

/**
 * Gets the display text for a request status.
 *
 * @param status - The request status
 * @returns string - Human-readable status text
 */
export function getRequestStatusDisplayText(status: RequestStatus): string {
  switch (status) {
    case RequestStatus.PENDING:
      return "Pending";
    case RequestStatus.REJECTED:
      return "Rejected";
    case RequestStatus.APPROVED:
      return "Approved";
    default:
      return "Unknown";
  }
}
