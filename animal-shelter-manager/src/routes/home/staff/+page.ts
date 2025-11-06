/**
 * src/routes/home/staff/+page.ts
 *
 * This page route handles requests to the base /home/staff URL
 * and redirects them to the default /home/staff/all-animals page
 * using client-side navigation.
 */
import { goto } from "$app/navigation";
import type { PageLoad } from "./$types";

export const load: PageLoad = () => {
  goto("/home/staff/all-animals", { replaceState: true });
};
