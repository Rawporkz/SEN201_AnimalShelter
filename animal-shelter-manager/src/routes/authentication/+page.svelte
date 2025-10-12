<!-- 
routes/authentication/+page.svelte

This file defines the authentication page of the application.
-->

<script lang="ts">
  import { User } from "@lucide/svelte";
  import { goto } from "$app/navigation";
  import {
    authenticateUser,
    type UserCredentials,
    type UserRole,
  } from "./authentication-utils";
  import { info, warn, error } from "@tauri-apps/plugin-log";

  /** Currently selected user role for authentication (staff or customer) */
  let selectedRole = $state("staff");

  /** Username entered by the user in the authentication form */
  let username = $state("");

  /** Password entered by the user in the authentication form */
  let password = $state("");

  /** Error message to display when authentication fails */
  let errorMessage = $state("");

  /** Flag to indicate if username field should show error state (red border) */
  let hasUsernameError = $state(false);

  /** Flag to indicate if password field should show error state (red border) */
  let hasPasswordError = $state(false);

  /** Flag to indicate if authentication is in progress */
  let isAuthenticating = $state(false);

  /**
   * Selects the user role (Staff or Customer) for authentication.
   *
   * @param role - The role to select ("Staff" or "Customer")
   */
  function selectRole(role: string): void {
    selectedRole = role;
    clearAllErrors();
  }

  /**
   * Clears all error states and error message.
   */
  function clearAllErrors(): void {
    errorMessage = "";
    hasUsernameError = false;
    hasPasswordError = false;
  }

  /**
   * Sets an error for the username field.
   *
   * @param message - The error message to display
   */
  function setUsernameError(message: string): void {
    errorMessage = message;
    hasUsernameError = true;
    hasPasswordError = false;
  }

  /**
   * Sets an error for the password field.
   *
   * @param message - The error message to display
   */
  function setPasswordError(message: string): void {
    errorMessage = message;
    hasPasswordError = true;
    hasUsernameError = false;
    password = "";
  }

  /**
   * Sets a general authentication error.
   *
   * @param message - The error message to display
   */
  function setGeneralError(message: string): void {
    errorMessage = message;
    hasUsernameError = false;
    hasPasswordError = false;
  }

  /**
   * Handles input changes to clear error state when user starts typing.
   */
  function handleInputChange(): void {
    if (errorMessage || hasUsernameError || hasPasswordError) {
      clearAllErrors();
    }
  }

  /**
   * Handles the submit button click for user authentication.
   */
  async function handleSubmit(): Promise<void> {
    // Clear any existing errors
    clearAllErrors();

    // Basic validation
    if (!username.trim()) {
      setUsernameError("Please enter a username.");
      return;
    }

    if (!password) {
      setPasswordError("Please enter a password.");
      return;
    }

    // Set loading state
    isAuthenticating = true;

    try {
      // Role is already in backend format (lowercase)
      const userRole: UserRole = selectedRole as UserRole;

      const credentials: UserCredentials = {
        username: username.trim(),
        password: password,
        role: userRole,
      };

      info(
        `Attempting authentication with: ${JSON.stringify({
          username: credentials.username,
          role: credentials.role,
        })}`,
      );

      // Attempt authentication
      const result = await authenticateUser(credentials);

      // Handle authentication result
      if (result.success) {
        info("Authentication successful!");
        goto("/");
      } else if (result.requiresAccountCreation) {
        // Username doesn't exist - navigate to create account page
        info("Username doesn't exist, navigating to create account...");
        const params = new URLSearchParams({
          username: credentials.username,
          password: credentials.password,
          role: credentials.role,
        });
        await goto(`/authentication/create-account?${params.toString()}`);
      } else if (result.invalidPassword) {
        // Password is wrong - show error on password field
        warn(`Invalid password, showing password error: ${result.message}`);
        setPasswordError(result.message);
      } else {
        // General authentication error - show as general error
        warn(`General authentication error: ${result.message}`);
        setGeneralError(result.message);
      }
    } catch (e) {
      error(`Authentication error: ${e}`);
      setGeneralError("Authentication failed. Please try again.");
    } finally {
      isAuthenticating = false;
    }
  }
</script>

<div class="container">
  <div class="auth-card">
    <div class="auth-header">
      <User size="64" />
      <h1>Sign Up / Log In</h1>
    </div>

    <div class="auth-form">
      <div class="auth-form-field">
        <label for="role">Role</label>
        <div class="role-selector" role="tablist" aria-label="Role selector">
          <div
            class="role-selector-slider {selectedRole === 'customer'
              ? 'right'
              : ''}"
            aria-hidden="true"
          ></div>
          <button
            type="button"
            role="tab"
            class="role-btn {selectedRole === 'staff' ? 'active' : ''}"
            aria-selected={selectedRole === "staff"}
            onclick={() => selectRole("staff")}
          >
            Staff
          </button>
          <button
            type="button"
            role="tab"
            class="role-btn {selectedRole === 'customer' ? 'active' : ''}"
            aria-selected={selectedRole === "customer"}
            onclick={() => selectRole("customer")}
          >
            Customer
          </button>
        </div>
      </div>

      <div class="auth-form-field">
        <label for="username">Username</label>
        <input
          type="text"
          id="username"
          bind:value={username}
          oninput={handleInputChange}
          class="auth-form-field-input {username
            ? 'has-value'
            : ''} {hasUsernameError ? 'has-error' : ''}"
        />
      </div>

      <div class="auth-form-field">
        <label for="password">Password</label>
        <input
          type="password"
          id="password"
          bind:value={password}
          oninput={handleInputChange}
          class="auth-form-field-input {password
            ? 'has-value'
            : ''} {hasPasswordError ? 'has-error' : ''}"
        />
      </div>
    </div>

    <button
      type="button"
      class="submit-btn"
      onclick={handleSubmit}
      disabled={isAuthenticating}
    >
      {isAuthenticating ? "Authenticating..." : "Sign Up / Log In"}
    </button>

    {#if errorMessage}
      <div class="error-message">
        {errorMessage}
      </div>
    {/if}
  </div>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
