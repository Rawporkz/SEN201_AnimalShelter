/**
 * src/routes/home/customer/+page.ts
 *
 * This page route handles requests to the base /home/customer URL
 * and redirects them to the default /home/customer/available-animals page
 * using client-side navigation.
 */
import { goto } from "$app/navigation";
import type { PageLoad } from "./$types";

/**
 * Redirects the user to the available animals page when they navigate to /home/customer.
 */
export const load: PageLoad = () => {
  goto("/home/customer/available-animals", { replaceState: true });
};
