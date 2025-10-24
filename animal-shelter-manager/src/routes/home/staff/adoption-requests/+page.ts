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
  AnimalAdoptionRequests,
  get_adoption_requests,
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

    /** Get animal ID from URL parameters */
    const animal_id = url.searchParams.get("animal_id");

    // Fetch adoption requests without filters initially
    const adoptionRequests: AnimalAdoptionRequests[] =
      await get_adoption_requests({}, animal_id);

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
