/**
 * lib/utils/filter-utils.ts
 *
 * This module contains utility types, enums, and functions for the FilterModal
 * component system, including filter criteria definitions and selection handling.
 */

/** Available filter criteria that can be shown in the modal */
export enum FilterCriteria {
  STATUS = "status",
  SEX = "sex",
  SPECIES_AND_BREEDS = "species_and_breeds",
  ADMISSION_DATE = "admission_date",
  ADOPTION_DATE = "adoption_date",
}

/** Types of filter components available */
export enum FilterType {
  CHOOSE_ONE = "choose_one",
  CHOOSE_MANY = "choose_many",
  NESTED_CHOOSE_MANY = "nested_choose_many",
}

/** Configuration for each filter criteria */
export interface FilterCriteriaConfig {
  criteria: FilterCriteria;
  type: FilterType;
  displayName: string;
}

/** Selected values for different filter types */
export type FilterValue = string | string[] | Record<string, string[]>;

/** Complete filter selections mapped by criteria */
export type FilterSelections = Partial<
  Record<FilterCriteria, FilterValue | null>
>;

/** Configuration mapping for each criteria */
export const FILTER_CRITERIA_CONFIGS: Record<
  FilterCriteria,
  FilterCriteriaConfig
> = {
  [FilterCriteria.STATUS]: {
    criteria: FilterCriteria.STATUS,
    type: FilterType.CHOOSE_MANY,
    displayName: "Status",
  },
  [FilterCriteria.SEX]: {
    criteria: FilterCriteria.SEX,
    type: FilterType.CHOOSE_MANY,
    displayName: "Sex",
  },
  [FilterCriteria.SPECIES_AND_BREEDS]: {
    criteria: FilterCriteria.SPECIES_AND_BREEDS,
    type: FilterType.NESTED_CHOOSE_MANY,
    displayName: "Species & Breeds",
  },
  [FilterCriteria.ADMISSION_DATE]: {
    criteria: FilterCriteria.ADMISSION_DATE,
    type: FilterType.CHOOSE_ONE,
    displayName: "Admission Date",
  },
  [FilterCriteria.ADOPTION_DATE]: {
    criteria: FilterCriteria.ADOPTION_DATE,
    type: FilterType.CHOOSE_ONE,
    displayName: "Adoption Date",
  },
};

/**
 * Gets the configuration for a specific filter criteria.
 *
 * @param criteria - The filter criteria to get configuration for
 * @returns FilterCriteriaConfig - Configuration object for the criteria
 */
export function getFilterConfig(
  criteria: FilterCriteria,
): FilterCriteriaConfig {
  return FILTER_CRITERIA_CONFIGS[criteria];
}

/**
 * Gets the display name for a filter criteria.
 *
 * @param criteria - The filter criteria to get display name for
 * @returns string - Human-readable display name
 */
export function getFilterDisplayName(criteria: FilterCriteria): string {
  return FILTER_CRITERIA_CONFIGS[criteria].displayName;
}

/**
 * Gets the filter type for a specific criteria.
 *
 * @param criteria - The filter criteria to get type for
 * @returns FilterType - The type of filter component to use
 */
export function getFilterType(criteria: FilterCriteria): FilterType {
  return FILTER_CRITERIA_CONFIGS[criteria].type;
}

/**
 * Creates an empty filter selections object with null values.
 *
 * @param criteriaList - List of criteria to initialize
 * @returns FilterSelections - Empty filter selections object
 */
export function createEmptyFilterSelections(
  criteriaList: FilterCriteria[],
): FilterSelections {
  const selections: FilterSelections = {} as FilterSelections;

  for (const criteria of criteriaList) {
    selections[criteria] = null;
  }

  return selections;
}

/**
 * Validates if a criteria list contains only valid FilterCriteria values.
 *
 * @param criteriaList - Array of criteria to validate
 * @returns boolean - True if all criteria are valid, false otherwise
 */
export function validateCriteriaList(
  criteriaList: unknown[],
): criteriaList is FilterCriteria[] {
  return criteriaList.every((criteria) =>
    Object.values(FilterCriteria).includes(criteria as FilterCriteria),
  );
}
