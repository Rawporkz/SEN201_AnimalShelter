/**
 * routes/send-adoption-request-form/[id]/form-options-utils.ts
 *
 * This module contains utility data for form options specific to the
 * send adoption request page, such as country lists and income ranges.
 */

/** List of countries for dropdown selection */
export const COUNTRY_OPTIONS: string[] = [
  "Thailand",
  "Myanmar",
  "Malaysia",
  "Singapore",
  "Vietnam",
  "Laos",
  "China",
];

/** Options for the annual income dropdown */
export const INCOME_OPTIONS: string[] = [
  "<USD 25,000",
  "USD 25,000 - USD 49,999",
  "USD 50,000 - USD 99,999",
  "USD 100,000 - USD 149,999",
  ">USD 150,000",
];
