/**
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
  type AnimalAdoptionRequests,
  getAdoptionRequests,
} from "./adoption-requests-utils";

export const load: PageLoad = async ({ url }) => {
  try {
    // Check if user is authenticated
    let currentUser: CurrentUser | null = await getCurrentUser();

    if (!currentUser) {
      // Redirect to authentication if not logged in
      error("User not authenticated");
      goto("/authentication");
      return; // prevent returning data
    }

    // Get animal ID from URL parameters
    const animalId = url.searchParams.get("animalId");

    // Fetch adoption requests without filters initially
    const adoptionRequests: AnimalAdoptionRequests[] =
      await getAdoptionRequests({}, animalId);

    return {
      currentUser,
      adoptionRequests,
    };
  } catch (e) {
    // Authentication check failed, redirect to authentication
    error(`Error during page load: ${e}`);
    goto("/");
  }
};
