/**
 * routes/home/customer/available-animals/+page.ts
 *
 * Data loading functions for customer/available-animals page authentication and routing.
 */

import { goto } from "$app/navigation";
import {
  getCurrentUser,
  type CurrentUser,
} from "$lib/utils/authentication-utils";
import type { PageLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";
import {
  getAnimals,
  AnimalStatus,
  getAdoptionRequestsByUsername,
} from "$lib/utils/data-utils";

/**
 * Loads the necessary data for the available animals page.
 * @returns A promise that resolves with the current user and a list of available animals.
 */
export const load: PageLoad = async () => {
  try {
    /** The currently authenticated user. */
    const currentUser: CurrentUser | null = await getCurrentUser();

    if (!currentUser) {
      // Redirect to authentication if not logged in
      error("User not authenticated");
      goto("/authentication");
      return;
    }

    // Fetch animals that are either AVAILABLE or REQUESTED
    let animals = await getAnimals({
      status: [AnimalStatus.AVAILABLE, AnimalStatus.REQUESTED],
    });

    // Fetch animals already requested by the current user to filter them out
    const requestedAnimals = await getAdoptionRequestsByUsername(
      currentUser.username,
    ).then((requests) => requests.map((req) => req.animalId));
    animals = animals.filter((animal) => !requestedAnimals.includes(animal.id));

    return {
      currentUser,
      animals,
    };
  } catch (e) {
    // Authentication check failed, redirect to authentication
    error(`Error during page load: ${e}`);
    goto("/");
    return { currentUser: null, animals: [] };
  }
};
