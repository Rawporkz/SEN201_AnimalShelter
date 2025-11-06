/**
 * routes/home/staff/adoption-requests/adoption-requests-utils.ts
 *
 * This file contains utility functions for the adoption requests page.
 */

import {
  getAnimals,
  type AnimalSummary,
  type AdoptionRequest,
  RequestStatus,
  AnimalStatus,
  getAdoptionRequestsByAnimalId,
  getAnimalById,
  updateAdoptionRequest,
  Animal,
  updateAnimal,
} from "$lib/utils/data-utils";
import { info, error } from "@tauri-apps/plugin-log";
import { FilterSelections } from "$lib/utils/filter-utils";

/** Structure for adoption requests, combining animal summary and the pending adoption requests. */
export type AnimalAdoptionRequests = {
  /** The summary data for the adopted animal. */
  animal: AnimalSummary;
  /** The pending adoption request associated with the animal. */
  request: AdoptionRequest;
};

/**
 * Fetches adoption reports for animals with the "REQUESTED" status.
 * Each report includes the animal's summary and the pending adoption requests.
 *
 * @param filterSeclections - The filter selections to apply when fetching adopted animals.
 * @param animalId - (Optional) Specific animal ID to fetch adoption requests for.
 * @returns A promise that resolves to an array of AnimalAdoptionReport objects.
 *          Returns an empty array if there's an error.
 */
export async function getAdoptionRequests(
  filterSeclections: FilterSelections,
  animalId?: string | null,
): Promise<AnimalAdoptionRequests[]> {
  try {
    // Fetch adopted animals based on whether an animalId is provided
    let adoptedAnimals: AnimalSummary[];
    if (animalId) {
      // Fetch specific animal by ID
      const animal: Animal | null = await getAnimalById(animalId);

      // If animal not found, return empty array
      if (!animal) {
        return [];
      }

      // Construct AnimalSummary for the specific animal
      const animalSummary: AnimalSummary = {
        id: animal.id,
        name: animal.name,
        specie: animal.specie,
        breed: animal.breed,
        sex: animal.sex,
        admissionTimestamp: animal.admissionTimestamp,
        imagePath: animal.imagePath,
        status: animal.status,
      };
      adoptedAnimals = [animalSummary];
    } else {
      // Fetch all adopted animals with status REQUESTED, applying filters if any
      adoptedAnimals = await getAnimals({
        ...filterSeclections,
        status: AnimalStatus.REQUESTED,
      });
    }

    // For each requested animal, get the associated adoption requests
    let animalAdoptionRequests: AnimalAdoptionRequests[] = [];
    for (const animal of adoptedAnimals) {
      // Get adoption requests for the animal
      const adoptionRequest: AdoptionRequest[] =
        await getAdoptionRequestsByAnimalId(animal.id);

      // Find the pending adoption request
      const pendingRequests = adoptionRequest.filter(
        (request) => request.status === RequestStatus.PENDING,
      );

      // Add each pending request to the result list
      for (const request of pendingRequests) {
        animalAdoptionRequests.push({
          animal: animal,
          request: request,
        });
      }
    }

    info(`${JSON.stringify(animalAdoptionRequests)}`);
    return animalAdoptionRequests;
  } catch (e) {
    error(`Error fetching adoption requests: ${e}`);
    return [];
  }
}

/**
 * Approves a pending adoption request.
 *
 * @param request - The adoption request to approve.
 */
export async function approveRequest(request: AdoptionRequest): Promise<void> {
  try {
    // Update the animal's status
    let animal = await getAnimalById(request.animalId);
    if (!animal) {
      error(`Animal with ID ${request.animalId} not found.`);
      return;
    }
    animal.status = AnimalStatus.ADOPTED;
    await updateAnimal(animal);

    // Update the status of the approved request
    request.status = RequestStatus.APPROVED;
    request.adoptionTimestamp = Math.floor(Date.now() / 1000);
    await updateAdoptionRequest(request);

    // Retrieve all other adoption requests for the same animal
    const otherPendingRequests = await getAdoptionRequestsByAnimalId(
      request.animalId,
    ).then((requests) =>
      requests.filter(
        (r) => r.id !== request.id && r.status === RequestStatus.PENDING,
      ),
    );

    // Update status of the other requests
    for (const other of otherPendingRequests) {
      other.status = RequestStatus.REJECTED;
      await updateAdoptionRequest(other);
    }
  } catch (e) {
    error(`Error approving adoption request: ${e}`);
  }
}

/**
 * Rejects a pending adoption request.
 *
 * @param request - The adoption request to reject.
 */
export async function rejectRequest(request: AdoptionRequest): Promise<void> {
  try {
    // Update the status of the rejected request
    request.status = RequestStatus.REJECTED;
    await updateAdoptionRequest(request);

    // Retrieve all other adoption requests for the same animal
    const otherPendingRequests = await getAdoptionRequestsByAnimalId(
      request.animalId,
    ).then((requests) =>
      requests.filter(
        (r) => r.id !== request.id && r.status === RequestStatus.PENDING,
      ),
    );

    // If there are no other pending requests, update the animal status back to AVAILABLE
    if (otherPendingRequests.length === 0) {
      let animal = await getAnimalById(request.animalId);
      if (!animal) {
        error(`Animal with ID ${request.animalId} not found.`);
      } else {
        animal.status = AnimalStatus.AVAILABLE;
        await updateAnimal(animal);
      }
    }
  } catch (e) {
    error(`Error rejecting adoption request: ${e}`);
  }
}
