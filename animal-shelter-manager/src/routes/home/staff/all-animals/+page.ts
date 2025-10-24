/**
 * routes/home/staff/all-animals/+page.ts
 *
 * Data loading functions for staff/all-animals page authentication and routing.
 */

import { goto } from "$app/navigation";
import {
  getCurrentUser,
  type CurrentUser,
} from "$lib/utils/authentication-utils";
import type { PageLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";
import { getAnimals } from "$lib/utils/animal-utils";

export const load: PageLoad = async () => {
  try {
    // Check if user is authenticated
    let currentUser: CurrentUser | null = await getCurrentUser();

    if (!currentUser) {
      // Redirect to authentication if not logged in
      error("User not authenticated");
      goto("/authentication");
      return;
    }

    return {
      currentUser,
      animals: await getAnimals(null),
    };
  } catch (e) {
    // Authentication check failed, redirect to authentication
    error(`Error during page load: ${e}`);
    goto("/");
  }
};
