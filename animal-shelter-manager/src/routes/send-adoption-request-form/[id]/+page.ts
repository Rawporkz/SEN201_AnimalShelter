/**
 * routes/send-adoption-request-form/[id]/+page.ts
 *
 * This file contains the load function for the send adoption request page.
 * It fetches the animal data based on the ID from the URL.
 */

import { error } from "@sveltejs/kit";
import { getAnimalById } from "$lib/utils/animal-utils";
import type { PageLoad } from "./$types";

/**
 * Load function to fetch animal data for the page.
 *
 * @param params - The page parameters, containing the animal ID.
 * @returns The animal data.
 * @throws 404 error if the animal is not found.
 */
export const load: PageLoad = async ({ params }) => {
  const animal = await getAnimalById(params.id);

  if (animal) {
    return {
      animal,
    };
  }

  error(404, "Animal not found");
};
