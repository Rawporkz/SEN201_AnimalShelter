/*
 * routes/home/staff/+layout.ts
 *
 * Data loading functions for staff layout authentication and routing.
 */

import { goto } from "$app/navigation";
import { getCurrentUser, CurrentUser } from "$lib/utils/authentication-utils";
import type { LayoutLoad } from "./$types";
import { error } from "@tauri-apps/plugin-log";

/** Staff layout data loader */
export const load: LayoutLoad = async ({ url }) => {
  try {
    // Check if user is authenticated
    let currentUser: CurrentUser | null = await getCurrentUser();

    if (!currentUser) {
      // Redirect to authentication if not logged in
      error("User not authenticated");
      goto("/authentication");
    }

    // TODO: Add role-based access control
    // if (currentUser.role !== "staff") {
    //   goto('/authentication');
    // }

    // If we're at the base staff route, redirect to all-animals by default
    if (url.pathname === "/home/staff" || url.pathname === "/home/staff/") {
      goto("/home/staff/all-animals");
    }

    return {
      currentUser,
    };
  } catch (e) {
    // Authentication check failed, redirect to authentication
    error(`Error during authentication check: ${e}`);
    goto("/authentication");
  }
};
