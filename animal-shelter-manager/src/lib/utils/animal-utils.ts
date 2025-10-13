/**
 * animal-utils.ts
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
  birth_month: number;
  /** Birth year of the animal */
  birth_year: number;
  /** Whether the animal has been neutered */
  neutered: boolean;
  /** Timestamp when the animal was admitted to the shelter */
  admission_timestamp: number;
  /** Current status of the animal */
  status: AnimalStatus;
  /** Path to the animal's image file */
  image_path?: string;
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
  admission_timestamp: number;
  /** Path to the animal's image file */
  image_path?: string;
  /** Current status of the animal (optional for display) */
  status?: AnimalStatus;
}

/** Represents an adoption request in the system */
export interface AdoptionRequest {
  /** Unique identifier for the adoption request */
  id: string;
  /** ID of the animal being requested for adoption */
  animal_id: string;
  /** Full name of the person requesting adoption */
  name: string;
  /** Email address of the requester */
  email: string;
  /** Telephone number of the requester */
  tel_number: string;
  /** Address of the requester */
  address: string;
  /** Occupation of the requester */
  occupation: string;
  /** Annual income of the requester */
  annual_income: string;
  /** Number of people in the household */
  num_people: number;
  /** Number of children in the household */
  num_children: number;
  /** Timestamp when the request was submitted */
  request_timestamp: number;
  /** Timestamp when the adoption was completed (0 if not completed) */
  adoption_timestamp: number;
  /** Current status of the request */
  status: RequestStatus;
  /** Country of the requester */
  country: string;
}

/** Simplified adoption request information for listing views */
export interface AdoptionRequestSummary {
  /** Unique identifier for the adoption request */
  id: string;
  /** ID of the animal being requested for adoption */
  animal_id: string;
  /** Full name of the person requesting adoption */
  name: string;
  /** Email address of the requester */
  email: string;
  /** Timestamp when the request was submitted */
  request_timestamp: number;
}

// ==================== ANIMAL FUNCTIONS ====================

/**
 * Retrieves all animals from the database.
 *
 * @returns Promise<AnimalSummary[]> - List of animal summaries
 * @throws Error if the operation fails
 */
export async function getAllAnimals(): Promise<AnimalSummary[]> {
  try {
    return await invoke<AnimalSummary[]>("get_all_animals");
  } catch (e) {
    error(`Failed to get all animals: ${e}`);
    return [];
  }
}

/**
 * Retrieves a specific animal by ID.
 *
 * @param animalId - The ID of the animal to retrieve
 * @returns Promise<Animal | null> - The animal data if found, null if not found
 * @throws Error if the operation fails
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
 * Creates a new animal in the database.
 *
 * @param animal - The animal data to create
 * @throws Error if the operation fails
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
 * @returns Promise<boolean> - True if animal was found and updated, false if not found
 * @throws Error if the operation fails
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
 * @param animalId - The ID of the animal to delete
 * @returns Promise<boolean> - True if animal was found and deleted, false if not found
 * @throws Error if the operation fails
 */
export async function deleteAnimal(animalId: string): Promise<boolean> {
  try {
    return await invoke<boolean>("delete_animal", { animalId });
  } catch (e) {
    error(`Failed to delete animal with ID ${animalId}: ${e}`);
    return false;
  }
}

// ==================== ADOPTION REQUEST FUNCTIONS ====================

/**
 * Retrieves all adoption requests from the database.
 *
 * @returns Promise<AdoptionRequestSummary[]> - List of adoption request summaries
 * @throws Error if the operation fails
 */
export async function getAllAdoptionRequests(): Promise<
  AdoptionRequestSummary[]
> {
  try {
    return await invoke<AdoptionRequestSummary[]>("get_all_adoption_requests");
  } catch (e) {
    error(`Failed to get all adoption requests: ${e}`);
    return [];
  }
}

/**
 * Retrieves a specific adoption request by ID.
 *
 * @param requestId - The ID of the adoption request to retrieve
 * @returns Promise<AdoptionRequest | null> - The adoption request data if found, null if not found
 * @throws Error if the operation fails
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
 * Creates a new adoption request in the database.
 *
 * @param request - The adoption request data to create
 * @throws Error if the operation fails
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
 * @returns Promise<boolean> - True if request was found and updated, false if not found
 * @throws Error if the operation fails
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
 * @returns Promise<boolean> - True if request was found and deleted, false if not found
 * @throws Error if the operation fails
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
 * @returns Promise<string | null> - The path of the uploaded file if successful, null if canceled
 * @throws Error if the upload fails
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
 * @throws Error if the deletion fails
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
export function calculateAge(birthYear: number, birthMonth: number): string {
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
    return `${ageMonths} month${ageMonths !== 1 ? "s" : ""} old`;
  } else if (ageMonths === 0) {
    return `${ageYears} year${ageYears !== 1 ? "s" : ""} old`;
  } else {
    return `${ageYears} year${ageYears !== 1 ? "s" : ""}, ${ageMonths} month${ageMonths !== 1 ? "s" : ""} old`;
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
