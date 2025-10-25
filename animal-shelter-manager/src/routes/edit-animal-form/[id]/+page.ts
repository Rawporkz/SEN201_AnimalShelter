/**
 * src/routes/edit-animal-form/[id]/+page.ts
 *
 * This file contains the load function for the edit animal form page,
 * which is responsible for fetching the animal's data before the page is rendered.
 */
import { error } from "@sveltejs/kit";
import { getAnimalById } from "$lib/utils/data-utils";
import type { PageLoad } from "./$types";

/**
 * Loads the animal data for the edit form.
 * @param params - The page parameters, containing the animal ID.
 * @returns A promise that resolves with the animal data.
 */
export const load: PageLoad = async ({ params }) => {
  /** The animal data. */
  const animal = await getAnimalById(params.id);

  if (animal) {
    return {
      animal,
    };
  }

  error(404, "Animal not found");
};
