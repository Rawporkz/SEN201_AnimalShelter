/**
 * routes/send-adoption-request-form/[id]/send-request-utils.ts
 *
 * This file contains utility functions for sending adoption requests.
 */

import { error } from "@tauri-apps/plugin-log";
import {
  AnimalStatus,
  createAdoptionRequest,
  getAnimalById,
  updateAnimal,
  type AdoptionRequest,
} from "$lib/utils/data-utils";

/**
 * Sends an adoption request for a specific animal.
 * Updates the animal status to "REQUESTED".
 *
 * @param adoptionRequest - The adoption request data to be sent.
 */
export async function sendAdoptionRequest(
  adoptionRequest: AdoptionRequest,
): Promise<void> {
  try {
    // Retrieve the animal by ID
    let animal = await getAnimalById(adoptionRequest.animalId);
    if (!animal) {
      error(`Animal with ID ${adoptionRequest.animalId} not found.`);
      return;
    }

    // Update the animal status
    animal.status = AnimalStatus.REQUESTED;
    const updateStatus = await updateAnimal(animal);
    if (!updateStatus) {
      error(`Failed to update status for animal ID ${animal.id}.`);
      return;
    }

    // Create a new adoption request in the database
    await createAdoptionRequest(adoptionRequest);
  } catch (e) {
    error(`Error sending adoption request: ${e}`);
  }
}
