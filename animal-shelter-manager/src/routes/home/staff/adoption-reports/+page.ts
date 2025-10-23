/*
 * routes/home/staff/adoption-reports/+page.ts
 *
 * Data loading functions for staff/adoption-reports page authentication and routing.
 */

import { goto } from "$app/navigation";
import { getCurrentUser, type CurrentUser } from "$lib/utils/authentication-utils";
import type { PageLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";
import {
  getAllAdoptionRequests,
  getAdoptionRequestById,
  getAnimalById,
  RequestStatus,
} from "$lib/utils/animal-utils";

/** adoption-reports page data loader */
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

    // TODO: Add role-based access control
    // if (currentUser.role !== "staff") {
    //   goto('/authentication');
    // }

    const requestSummaries = await getAllAdoptionRequests();

    const adoptionRequestsData = await Promise.all(
      requestSummaries.map(async (summary) => {
        const [animal, request] = await Promise.all([
          getAnimalById(summary.animal_id),
          getAdoptionRequestById(summary.id),
        ]);
        return { animal, request };
      }),
    );

    const completedRequests = adoptionRequestsData.filter(
      (data) =>
        data.animal &&
        data.request &&
        data.request.status !== RequestStatus.PENDING,
    );

    return {
      currentUser,
      adoptionRequests: completedRequests,
    };
  } catch (e) {
    // Authentication check failed, redirect to authentication
    error(`Error during page load: ${e}`);
    goto("/authentication");
  }
};