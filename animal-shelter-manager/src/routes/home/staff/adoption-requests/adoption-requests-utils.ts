/**
 * routes/home/staff/adoption-requests/adoption-requests-utils.ts
 *
 * This file contains utility functions for the adoption requests page.
 * It includes functions to fetch adoption requests, which combine animal data
 * with their corresponding pending adoption requests.
 */

import {
  getAnimals,
  type AnimalSummary,
  type AdoptionRequest,
  RequestStatus,
  AnimalStatus,
  getAdoptionRequestsByAnimalId,
  getAnimalById,
} from "$lib/utils/animal-utils";
import { error } from "@tauri-apps/plugin-log";
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
export async function get_adoption_requests(
  filterSeclections: FilterSelections,
  animalId?: string | null,
): Promise<AnimalAdoptionRequests[]> {
  try {
    // Fetch adopted animals based on whether an animalId is provided
    let adoptedAnimals: AnimalSummary[];
    if (animalId) {
      // Fetch specific animal by ID
      const animal = (await getAnimalById(animalId)) as AnimalSummary;
      adoptedAnimals = [animal];
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
      const adoptionRequest: AdoptionRequest[] | null =
        await getAdoptionRequestsByAnimalId(animal.id);

      // Find the pending adoption request
      const pendingRequests = adoptionRequest?.filter(
        (request) => request.status === RequestStatus.PENDING,
      );

      // Add each pending request to the result list
      for (const request of pendingRequests || []) {
        animalAdoptionRequests.push({
          animal: animal,
          request: request,
        });
      }
    }

    return animalAdoptionRequests;
  } catch (e) {
    error(`Error fetching adoption requests: ${e}`);
    return [];
  }
}
