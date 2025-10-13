# Code review

for `animal-shelter-manager/src/lib/utils/animal-utils.ts`

## Enum


### AnimalStatus
Why did we use enum? 
it's better than putting status as a string. Enum type makes sure that we can only put one of these 4 values as an animal's status.

### RequestStatus
No problem with this enum. The 3 options are sufficient for all adoption requests.

## Interfaces


### Animal 
Non suggested to put in 3 more fields: adoption_timestamp, appearance, and bio, since we are missing that from the Animal's status. He needs it for the `AdopterInfoModal`.

Jira agrees with adding bio and appearance. But for adoption_timestamp, he suggests that we can just use the adoption_timestamp field in the `AdoptionRequest` interface instead. We can retrieve the associated adoption by matching the animal's ID and the adoption's animal_id field. I will add both the appearance and bio since

### AnimalSummary
Animal status should not be null-able

### AdoptionRequest
Wants a country field for usage in AdopterInfoModal

### AdoptionRequestSummary
This is already good, no changes needed

## Animal Function

### getAllAnimals
This function is called by the UI Components
We shouldn't rely on UI on catching error. We should return an empty array if there is an error, handle the error internally by the function.

### getAnimalById
Since this function already returns null when an animal is not found, we can also return null when an error occurs, so we don't have to throw errors

### createAnimal
Log instead of throwing errors, because if UI call this function outside the try block, it will crash the application.

### updateAnimal & deleteAnimal
Since the function already returns false, when the animal is not found, we should also return false if an error occurs, to avoid throwing errors.

## Adoption Request Function
Implement updates that will be made in the animal function into the Adoption Request Function as well.

## File Function


### uploadAnimalImage
Since this function already returns null when an animal is not found, we can also return null when an error occurs, so we don't have to throw errors

### deleteFile
Log instead of throwing errors, because if UI call this function outside the try block, it will crash the application.

## Utility Function


### formatTimestamp
No problem with this function

### calculateAge
No problem with this function

### getStatusDisplayText
Default value of `"Unknown"` is a nice touch, in case we add more options to the status enum later and forget to update this function, it will return an `"Unknown"` instead of crashing.
