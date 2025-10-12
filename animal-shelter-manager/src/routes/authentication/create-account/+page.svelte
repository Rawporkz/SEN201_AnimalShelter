<!-- 
routes/authentication/create-account/+page.svelte

This file defines the create-account page of the application.
Displays a card for password confirmation during account creation.
-->

<script lang="ts">
  import { User } from "@lucide/svelte";
  import {
    createUserAccount,
    type UserCredentials,
    type UserRole,
  } from "../authentication-utils";
  import { info, error } from "@tauri-apps/plugin-log";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";

  /** URL search parameters from the current page */
  const params = page.url.searchParams;

  /** Username from URL parameters */
  const username = params.get("username");

  /** Password from URL parameters */
  const password = params.get("password");

  /** Role from URL parameters */
  const role = params.get("role");

  /** User credentials object constructed from URL parameters */
  const userCredentials: UserCredentials = {
    username: username || "",
    password: password || "",
    role: (role as UserRole) || "staff",
  };

  /** Password confirmation entered by the user */
  let passwordConfirmation = $state("");

  /** Error message to display when account creation fails */
  let errorMessage = $state("");

  /** Flag to indicate if password confirmation field should show error state (red border) */
  let hasPasswordConfirmationError = $state(false);

  /** Flag to indicate if account creation is in progress */
  let isCreatingAccount = $state(false);

  /**
   * Clears the error state and error message.
   */
  function clearError(): void {
    errorMessage = "";
    hasPasswordConfirmationError = false;
  }

  /**
   * Sets an error for the password confirmation field.
   *
   * @param message - The error message to display
   */
  function setPasswordConfirmationError(message: string): void {
    errorMessage = message;
    hasPasswordConfirmationError = true;
    passwordConfirmation = ""; // Clear password confirmation field
  }

  /**
   * Sets a general error without clearing fields.
   *
   * @param message - The error message to display
   */
  function setGeneralError(message: string): void {
    errorMessage = message;
    hasPasswordConfirmationError = false;
  }

  /**
   * Handles input changes to clear error state when user starts typing.
   */
  function handleInputChange(): void {
    if (hasPasswordConfirmationError || errorMessage) {
      clearError();
    }
  }

  /**
   * Handles the back button click.
   */
  async function handleBack(): Promise<void> {
    goto("/authentication");
  }

  /**
   * Handles the confirm button click for account creation.
   * Validates password confirmation and proceeds with account creation.
   */
  async function handleConfirm(): Promise<void> {
    // Clear any existing errors
    clearError();

    // Basic validation
    if (!passwordConfirmation) {
      setPasswordConfirmationError("Please confirm your password.");
      return;
    }

    // Check if password confirmation matches original password
    if (passwordConfirmation !== userCredentials.password) {
      setPasswordConfirmationError("Password confirmation does not match.");
      return;
    }

    // Set loading state
    isCreatingAccount = true;

    try {
      // Create account with password confirmation
      const result = await createUserAccount(
        userCredentials,
        passwordConfirmation,
      );

      if (result.success) {
        info("Account created successfully!");
        goto("/");
      } else {
        // Account creation failed - show general error
        setGeneralError(result.message);
      }
    } catch (e) {
      error(`Account creation error: ${e}`);
      setGeneralError("Account creation failed. Please try again.");
    } finally {
      isCreatingAccount = false;
    }
  }
</script>

<div class="container">
  <div class="create-account-card">
    <div class="create-account-header">
      <User size="64" />
      <h1>Creating an Account</h1>
    </div>

    <div class="create-account-form">
      <div class="create-account-form-field">
        <label for="password-confirmation">Re-enter your password</label>
        <input
          type="password"
          id="password-confirmation"
          bind:value={passwordConfirmation}
          oninput={handleInputChange}
          class="create-account-form-field-input {passwordConfirmation
            ? 'has-value'
            : ''} {hasPasswordConfirmationError ? 'has-error' : ''}"
        />
      </div>

      <div class="button-row">
        <button
          type="button"
          class="back-btn"
          onclick={handleBack}
          disabled={isCreatingAccount}
        >
          Back
        </button>

        <button
          type="button"
          class="confirm-btn"
          onclick={handleConfirm}
          disabled={isCreatingAccount}
        >
          Confirm
        </button>
      </div>

      {#if errorMessage}
        <div class="error-message">
          {errorMessage}
        </div>
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
