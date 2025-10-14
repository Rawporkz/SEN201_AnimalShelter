<!-- 
routes/+page.svelte

This file is the root page of the frontend application.
It decides whether to show the authentication page, the staff's home page,
or the customer's home page based on the user's authentication status and role.
-->

<script lang="ts">
  import "./style.scss";
  import FormDropdownButton from "$lib/components/FormDropdownButton/FormDropdownButton.svelte";
  import FormTextField from "$lib/components/FormTextField/FormTextField.svelte";

  /** Selected value from dropdown */
  let selectedValue: string = "";

  /** Text field value */
  let textFieldValue: string = "";

  /**
   * Handles dropdown selection change
   * @param value - The selected value from dropdown
   */
  function handleDropdownChange(value: string): void {
    selectedValue = value;
  }
</script>

<div class="test-container">
  <div class="form-row">
    <FormDropdownButton
      options={[
        "Dog",
        "Cat",
        "Bird",
        "Rabbit",
        "Fish",
        "Horse",
        "Cow",
        "Pig",
        "Sheep",
        "Goat",
        "Chicken",
        "Duck",
        "Turkey",
        "Hamster",
        "Guinea Pig",
      ]}
      placeholder="Select an animal"
      width="300px"
      label="Choose an animal type:"
      maxOptions={4}
      onSelect={handleDropdownChange}
    />

    <FormTextField
      label="Animal description:"
      placeholder="Enter animal description..."
      bind:value={textFieldValue}
      boxWidth="300px"
      rows={1}
    />
  </div>

  <div class="text-below">
    <p>This text is positioned below the dropdown.</p>
    <p>When the dropdown opens, observe how this text behaves.</p>
    <p>Does it get pushed down or does the dropdown overlay on top of it?</p>
    {#if selectedValue}
      <p><strong>Selected: {selectedValue}</strong></p>
    {/if}
    {#if textFieldValue}
      <p><strong>Description: {textFieldValue}</strong></p>
    {/if}
  </div>
</div>

<style lang="scss">
  .test-container {
    display: flex;
    flex-direction: column;
    gap: 20px;
    padding: 40px;
    max-width: 800px;
  }

  .form-row {
    display: flex;
    gap: 20px;
    align-items: flex-start;
  }

  .text-below {
    background-color: #f0f0f0;
    padding: 16px;
    border-radius: 8px;
    border: 2px solid #ddd;

    p {
      margin: 8px 0;
      line-height: 1.5;

      &:first-child {
        margin-top: 0;
      }

      &:last-child {
        margin-bottom: 0;
      }
    }
  }
</style>
