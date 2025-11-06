/**
 * src/routes/home/customer/my-adoption-requests/status-utils.ts
 *
 * This file contains utility functions for the my-adoption-requests page, specifically for handling status-related logic.
 */

/**
 * Gets the color for a given request status.
 * @param status The status of the request.
 * @returns The color for the status.
 */
export function getRequestStatusColor(status: string): string {
  switch (status) {
    case "pending":
      return "#ffc107";
    case "approved":
      return "#007bff";
    case "rejected":
      return "#ea4444";
    default:
      return "#6c757d";
  }
}
