/**
 * routes/home/customer/my-adoption-requests/my-adoption-requests-utils.ts
 *
 * This file contains utility functions for the my adoption requests page.
 * It includes functions to fetch my adoption requests, which combine animal data
 * with their corresponding adoption requests submitted by a specific user.
 */

import {
  Animal,
  getAdoptionRequestsByUsername,
  getAnimalById,
  type AdoptionRequest,
  type AnimalSummary,
} from "$lib/utils/data-utils";

/** Structure for a user's adoption request, combining animal summary and the adoption request. */
export type MyAdoptionRequest = {
  /** The summary data for the animal associated with the adoption request. */
  animal: AnimalSummary;
  /** The adoption request submitted by the user. */
  request: AdoptionRequest;
};

/**
 * Fetches adoption requests submitted by a specific user.
 * Each request includes the animal's summary and the request.
 *
 * @returns A promise that resolves to an array of MyAdoptionRequest objects.
 *          Returns an empty array if there's an error.
 */
export async function getMyAdoptionRequests(
  username: string,
): Promise<MyAdoptionRequest[]> {
  // Fetch adoption requests submitted by the user
  const requests = await getAdoptionRequestsByUsername(username);

  // Combine each request with its associated animal summary
  const myAdoptionRequests: MyAdoptionRequest[] = [];
  for (const request of requests) {
    // Fetch the animal summary for the request
    const animal: Animal | null = await getAnimalById(request.animalId);

    // If the animal doesn't exist, skip this request
    if (!animal) {
      continue;
    }

    // Construct the animal summary and add to the result array
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

    myAdoptionRequests.push({ animal: animalSummary, request });
  }

  return myAdoptionRequests;
}
