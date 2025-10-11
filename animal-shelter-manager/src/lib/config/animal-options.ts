/**
 * animal-options.ts
 *
 * This module contains all configuration options for animal-related dropdowns
 * used throughout the animal shelter management system.
 */

// Animal status options for tracking adoption lifecycle
export type AnimalStatus =
  | "available"
  | "requested"
  | "adopted"
  | "passed away";

// Animal sex options for biological classification
export type AnimalSex = "male" | "female";

// Animal species supported by the shelter system
export type AnimalSpecies =
  | "dog"
  | "cat"
  | "rabbit"
  | "bird"
  | "hamster"
  | "guinea pig"
  | "ferret"
  | "reptile"
  | "fish"
  | "horse"
  | "goat"
  | "pig";

// Neutered status options for spay/neuter tracking
export type NeuteredStatus = "yes" | "no";

// Available animal status options with display labels
export const ANIMAL_STATUS_OPTIONS: Array<{
  value: AnimalStatus;
  label: string;
}> = [
  { value: "available", label: "Available" },
  { value: "requested", label: "Requested" },
  { value: "adopted", label: "Adopted" },
  { value: "passed away", label: "Passed Away" },
];

// Available animal sex options with display labels
export const ANIMAL_SEX_OPTIONS: Array<{
  value: AnimalSex;
  label: string;
}> = [
  { value: "male", label: "Male" },
  { value: "female", label: "Female" },
];

// Available animal species options with display labels
export const ANIMAL_SPECIES_OPTIONS: Array<{
  value: AnimalSpecies;
  label: string;
}> = [
  { value: "dog", label: "Dog" },
  { value: "cat", label: "Cat" },
  { value: "rabbit", label: "Rabbit" },
  { value: "bird", label: "Bird" },
  { value: "hamster", label: "Hamster" },
  { value: "guinea pig", label: "Guinea Pig" },
  { value: "ferret", label: "Ferret" },
  { value: "reptile", label: "Reptile" },
  { value: "fish", label: "Fish" },
  { value: "horse", label: "Horse" },
  { value: "goat", label: "Goat" },
  { value: "pig", label: "Pig" },
];

// Available neutered status options with display labels
export const NEUTERED_STATUS_OPTIONS: Array<{
  value: NeuteredStatus;
  label: string;
}> = [
  { value: "yes", label: "Yes" },
  { value: "no", label: "No" },
];

// Comprehensive breed options organized by species
export const ANIMAL_BREED_OPTIONS: Record<AnimalSpecies, string[]> = {
  dog: [
    "Labrador Retriever",
    "Golden Retriever",
    "German Shepherd",
    "Bulldog",
    "Poodle",
    "Beagle",
    "Rottweiler",
    "Yorkshire Terrier",
    "Dachshund",
    "Siberian Husky",
  ],
  cat: [
    "Domestic Shorthair",
    "Domestic Longhair",
    "Persian",
    "Maine Coon",
    "Siamese",
    "Ragdoll",
    "British Shorthair",
    "Abyssinian",
    "Russian Blue",
    "Bengal",
  ],
  rabbit: [
    "Holland Lop",
    "Mini Lop",
    "Netherland Dwarf",
    "Lionhead",
    "Flemish Giant",
    "Rex",
    "Angora",
    "Dutch",
  ],
  bird: [
    "Budgerigar",
    "Cockatiel",
    "Canary",
    "Lovebird",
    "Conure",
    "Cockatoo",
    "African Grey",
    "Macaw",
    "Finch",
  ],
  hamster: [
    "Syrian",
    "Dwarf Campbell's",
    "Dwarf Winter White",
    "Roborovski",
    "Chinese",
    "European",
  ],
  "guinea pig": [
    "American",
    "Peruvian",
    "Abyssinian",
    "Silkie",
    "Teddy",
    "Coronet",
    "Skinny Pig",
  ],
  ferret: [
    "Domestic Ferret",
    "Albino",
    "Sable",
    "Silver",
    "Cinnamon",
    "Chocolate",
  ],
  reptile: [
    "Ball Python",
    "Bearded Dragon",
    "Leopard Gecko",
    "Corn Snake",
    "Red-Eared Slider",
    "Blue-Tongued Skink",
    "Iguana",
    "Chameleon",
  ],
  fish: [
    "Goldfish",
    "Betta",
    "Guppy",
    "Angelfish",
    "Tetra",
    "Molly",
    "Platy",
    "Barb",
    "Cichlid",
  ],
  horse: [
    "Quarter Horse",
    "Thoroughbred",
    "Arabian",
    "Paint Horse",
    "Appaloosa",
    "Tennessee Walking Horse",
    "Mustang",
    "Clydesdale",
  ],
  goat: [
    "Nigerian Dwarf",
    "Boer",
    "Nubian",
    "Pygmy",
    "Alpine",
    "Saanen",
    "LaMancha",
  ],
  pig: [
    "Pot-bellied Pig",
    "Mini Pig",
    "Kunekune",
    "Vietnamese Pot-bellied",
    "Juliana",
    "American Mini Pig",
  ],
};

/**
 * Gets the breed options for a specific animal species.
 *
 * @param species - The animal species to get breeds for
 * @returns string[] - Array of breed names for the specified species
 */
export function getBreedsForSpecies(species: AnimalSpecies): string[] {
  return ANIMAL_BREED_OPTIONS[species] || [];
}

/**
 * Gets all available animal species as an array of strings.
 *
 * @returns AnimalSpecies[] - Array of all available species
 */
export function getAllSpecies(): AnimalSpecies[] {
  return Object.keys(ANIMAL_BREED_OPTIONS) as AnimalSpecies[];
}
