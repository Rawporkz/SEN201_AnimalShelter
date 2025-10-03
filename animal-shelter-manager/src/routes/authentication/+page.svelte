<!-- 
routes/authentication/+page.svelte

This file defines the authentication page of the application.
-->

<script lang="ts">
  import "./style.scss";

  // Currently selected user role for authentication (Staff or Customer)
  let selectedRole = $state("Staff");

  // Username entered by the user in the authentication form
  let username = $state("");

  // Password entered by the user in the authentication form
  let password = $state("");

  /**
   * Selects the user role (Staff or Customer) for authentication.
   *
   * @param role - The role to select ("Staff" or "Customer")
   */
  function selectRole(role: string) {
    selectedRole = role;
    // TODO: Handle role selection logic
  }

  /**
   * Handles the form submission for user authentication.
   * Prevents the default form submission and processes authentication logic.
   *
   * @param event - The form submit event
   */
  function handleSubmit(event: Event) {
    event.preventDefault();
    // TODO: Implement authentication logic
  }
</script>

<div class="auth-container">
  <div class="auth-card">
    <div class="auth-header">
      <div class="user-icon">
        <!-- TODO: Use imported icon -->
        <svg width="40" height="40" viewBox="0 0 24 24" fill="currentColor">
          <circle cx="12" cy="8" r="4" />
          <path d="M12 14c-6 0-8 2.5-8 6v2h16v-2c0-3.5-2-6-8-6z" />
        </svg>
      </div>
      <h1>Sign Up / Log In</h1>
    </div>

    <form onsubmit={(event) => handleSubmit(event)} class="auth-form">
      <div class="form-group">
        <label for="role">Role</label>
        <div class="role-selector">
          <button
            type="button"
            class="role-btn {selectedRole === 'Staff' ? 'active' : ''}"
            onclick={() => selectRole("Staff")}
          >
            Staff
          </button>
          <button
            type="button"
            class="role-btn {selectedRole === 'Customer' ? 'active' : ''}"
            onclick={() => selectRole("Customer")}
          >
            Customer
          </button>
        </div>
      </div>

      <div class="form-group">
        <label for="username">Username</label>
        <input
          type="text"
          id="username"
          bind:value={username}
          class="form-input"
          required
        />
      </div>

      <div class="form-group">
        <label for="password">Password</label>
        <input
          type="password"
          id="password"
          bind:value={password}
          class="form-input"
          required
        />
      </div>

      <button type="submit" class="submit-btn"> Sign Up / Log In </button>
    </form>
  </div>
</div>
