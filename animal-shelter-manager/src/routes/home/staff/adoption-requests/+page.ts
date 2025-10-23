/*
 * routes/home/staff/adoption-requests/+page.ts
 *
 * Data loading functions for staff/adoption-requests page authentication and routing.
 */

import { goto } from "$app/navigation";
import {
  getCurrentUser,
  type CurrentUser,
} from "$lib/utils/authentication-utils";
import type { PageLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";
import {
  getAdoptionRequests,
  RequestStatus,
  AnimalStatus,
  getAnimals,
  type Animal,
  type AdoptionRequestSummary,
} from "$lib/utils/animal-utils";

/**
 * Adoption requests page data loader.
 */
export const load: PageLoad = async () => {
  try {
    // Check if user is authenticated
    let currentUser: CurrentUser | null = await getCurrentUser();

    if (!currentUser) {
      // Redirect to authentication if not logged in
      error("User not authenticated");
      goto("/authentication");
      return; // prevent returning data
    }

    const requestSummaries = await getAdoptionRequests({
      status: [RequestStatus.PENDING],
    });

    const animalSummaries = await getAnimals({
      status: [AnimalStatus.REQUESTED],
    });

    const adoptionRequestsData = requestSummaries.map((request) => {
      const animal = animalSummaries.find((a) => a.id === request.animal_id);
      return animal ? { animal, request } : null;
    });

    const pendingRequests = adoptionRequestsData.filter(
      (data) => data && data.animal && data.request,
    ) as { animal: Animal; request: AdoptionRequestSummary }[];

    return {
      currentUser,
      adoptionRequests: pendingRequests,
    };
  } catch (e) {
    // Authentication check failed, redirect to authentication
    error(`Error during page load: ${e}`);
    goto("/");
  }
};
