/**
 * routes/home/customer/my-adoption-requests/+page.ts
 *
 * Data loading functions for customer/my-adoption-requests page authentication and routing.
 */

import { goto } from "$app/navigation";
import {
  getCurrentUser,
  type CurrentUser,
} from "$lib/utils/authentication-utils";
import type { PageLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";
import {
  getMyAdoptionRequests,
  MyAdoptionRequest,
} from "./my-adoption-request-utils";

/**
 * Loads the necessary data for the my adoption requests page.
 * @returns A promise that resolves with the current user and a list of their adoption requests.
 */
export const load: PageLoad = async () => {
  try {
    // Check if user is authenticated
    const currentUser: CurrentUser | null = await getCurrentUser();

    if (!currentUser) {
      error("User not authenticated");
      goto("/authentication");
      return;
    }

    // Fetch adoption requests submitted by the current user
    const myAdoptionRequests: MyAdoptionRequest[] = await getMyAdoptionRequests(
      currentUser.username,
    );

    return {
      currentUser,
      myAdoptionRequests,
    };
  } catch (e) {
    error(`Error during page load: ${e}`);
    goto("/");
    return { currentUser: null, myAdoptionRequests: [] };
  }
};
