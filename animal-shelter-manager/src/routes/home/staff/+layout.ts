/*
 * routes/home/staff/+layout.ts
 *
 * Data loading functions for staff layout authentication and routing.
 */

import { redirect } from "@sveltejs/kit";
import { getCurrentUser } from "$lib/utils/authentication-utils";
import type { LayoutLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";

/** Staff layout data loader */
export const load: LayoutLoad = async ({ url }) => {
  try {
    // Check if user is authenticated
    const currentUser = await getCurrentUser();
    error(`Current User: ${JSON.stringify(currentUser)}`);

    if (!currentUser) {
      // Redirect to authentication if not logged in
      error("User not authenticated");
      redirect(302, "/authentication");
    }

    // TODO: Add role-based access control
    // if (currentUser.role !== "staff") {
    //   redirect(302, '/authentication');
    // }

    // If we're at the base staff route, redirect to all-animals by default
    if (url.pathname === "/home/staff" || url.pathname === "/home/staff/") {
      redirect(302, "/home/staff/all-animals");
    }

    return {
      currentUser,
    };
  } catch (error) {
    // Authentication check failed, redirect to authentication
    error("Error during authentication check");
    redirect(302, "/authentication");
  }
};
