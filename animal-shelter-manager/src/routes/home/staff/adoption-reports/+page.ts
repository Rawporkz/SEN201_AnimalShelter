/**
 * routes/home/staff/adoption-reports/+page.ts
 *
 * Data loading functions for staff/adoption-reports page authentication and routing.
 */

import { goto } from "$app/navigation";
import {
  getCurrentUser,
  type CurrentUser,
} from "$lib/utils/authentication-utils";
import type { PageLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";
import {
  type AnimalAdoptionReport,
  get_adoption_reports,
} from "./adoption-reports-utils";

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

    // Fetch adoption reports without filters initially
    const adoptionRequests: AnimalAdoptionReport[] = await get_adoption_reports(
      {},
    );

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
