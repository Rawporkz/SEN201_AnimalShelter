/**
 * routes/home/staff/adoption-reports/adoption-reports-utils.ts
 *
 * This file contains utility functions for the adoption reports page.
 * It includes functions to fetch adoption reports, which combine animal data
 * with their corresponding approved adoption requests.
 */

import {
  getAnimals,
  type AnimalSummary,
  type AdoptionRequest,
  RequestStatus,
  AnimalStatus,
  getAdoptionRequestsByAnimalId,
} from "$lib/utils/animal-utils";
import { error } from "@tauri-apps/plugin-log";
import { FilterSelections } from "$lib/utils/filter-utils";

/** Structure for an adoption report, combining animal summary and the accepted adoption request. */
export type AnimalAdoptionReport = {
  /** The summary data for the adopted animal. */
  animal: AnimalSummary;
  /** The approved adoption request associated with the animal. */
  adoption: AdoptionRequest;
};

/**
 * Fetches adoption reports for animals with the "ADOPTED" status.
 * Each report includes the animal's summary and the approved adoption request.
 *
 * @param filterSeclections - The filter selections to apply when fetching adopted animals.
 * @returns A promise that resolves to an array of AnimalAdoptionReport objects.
 *          Returns an empty array if there's an error.
 */
export async function getAdoptionReports(
  filterSeclections: FilterSelections,
): Promise<AnimalAdoptionReport[]> {
  try {
    // Fetch all adopted animals with status ADOPTED, applying filters if any
    const adoptedAnimals = await getAnimals({
      ...filterSeclections,
      status: AnimalStatus.ADOPTED,
    });

    // For each adopted animal, get the associated adoption request
    let animalAdoptionRequests: AnimalAdoptionReport[] = [];
    for (const animal of adoptedAnimals) {
      // Get adoption requests for the animal
      const adoptionRequest: AdoptionRequest[] =
        await getAdoptionRequestsByAnimalId(animal.id);

      // Find the approved adoption request
      const approvedRequests = adoptionRequest.find(
        (request) => request.status === RequestStatus.APPROVED,
      );

      if (approvedRequests) {
        animalAdoptionRequests.push({
          animal: animal,
          adoption: approvedRequests,
        });
      }
    }

    return animalAdoptionRequests;
  } catch (e) {
    error(`Error fetching adoption reports: ${e}`);
    return [];
  }
}
