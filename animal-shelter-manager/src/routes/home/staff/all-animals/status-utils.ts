/**
 * src/routes/home/staff/all-animals/status-utils.ts
 *
 * This file contains utility functions for the all-animals page, specifically for handling status-related logic.
 */

/**
 * Gets the color for a given status.
 * @param status The status of the animal.
 * @returns The color for the status.
 */
export function getStatusColor(status: string): string {
  switch (status) {
    case "available":
      return "#00b047";
    case "requested":
      return "#ffc107";
    case "adopted":
      return "#007bff";
    case "passed-away":
      return "#6c757d";
    default:
      return "#6c757d";
  }
}
