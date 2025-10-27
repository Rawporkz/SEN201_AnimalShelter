/**
 * lib/utils/authentication-utils.ts
 *
 * This module contains utility functions for authentication-related operations
 * that communicate with the Tauri backend for user login and account creation.
 */

import { invoke } from "@tauri-apps/api/core";
import { info, error } from "@tauri-apps/plugin-log";

// ==================== TYPES ====================

/** User role enum type */
export enum UserRole {
  STAFF = "staff",
  CUSTOMER = "customer",
}

/** Login result enum type */
export enum LoginResult {
  SUCCESS = "success",
  INVALID_PASSWORD = "invalid-password",
  USER_NOT_FOUND = "user-not-found",
}

/** Current user type containing username and role */
export interface CurrentUser {
  /** Username of the current user */
  username: string;
  /** Role of the current user */
  role: UserRole;
}

/** Validation result type */
export interface ValidationResult {
  /** Indicates if the input is valid */
  isValid: boolean;
  /** Error message if validation fails */
  errorMessage: string;
}

// ==================== INTERFACES ====================

/** Authentication result interface */
export interface AuthResult {
  success: boolean;
  message: string;
  requiresAccountCreation?: boolean;
  invalidPassword?: boolean;
}

/** User credentials interface */
export interface UserCredentials {
  username: string;
  password: string;
  role: UserRole;
}

// ==================== VALIDATION FUNCTIONS ====================

/**
 * Validates if a username is in valid format.
 *
 * @param username - The username to validate
 * @returns ValidationResult - An object with `isValid` and `errorMessage`
 */
export function validateUsername(username: string): ValidationResult {
  const trimmedUsername: string = username.trim();
  if (trimmedUsername.length < 3) {
    return {
      isValid: false,
      errorMessage: "Username must be at least 3 characters long.",
    };
  }
  if (!/^[a-zA-Z0-9]+$/.test(trimmedUsername)) {
    return {
      isValid: false,
      errorMessage: "Username can only contain alphanumeric characters.",
    };
  }
  return {
    isValid: true,
    errorMessage: "",
  };
}

/**
 * Validates if a password meets the minimum requirements.
 *
 * @param password - The password to validate
 * @returns ValidationResult - An object with `isValid` and `errorMessage`
 */
export function validatePassword(password: string): ValidationResult {
  if (password.length < 8) {
    return {
      isValid: false,
      errorMessage: "Password must be at least 8 characters long.",
    };
  }
  if (!/\d/.test(password)) {
    return {
      isValid: false,
      errorMessage: "Password must contain at least one number.",
    };
  }
  if (!/[a-z]/.test(password)) {
    return {
      isValid: false,
      errorMessage: "Password must contain at least one lowercase letter.",
    };
  }
  if (!/[A-Z]/.test(password)) {
    return {
      isValid: false,
      errorMessage: "Password must contain at least one uppercase letter.",
    };
  }
  if (!/[^a-zA-Z0-9]/.test(password)) {
    return {
      isValid: false,
      errorMessage: "Password must contain at least one special character.",
    };
  }
  return {
    isValid: true,
    errorMessage: "",
  };
}

/**
 * Validates user credentials format before attempting authentication.
 *
 * @param credentials - The user credentials to validate
 * @returns ValidationResult - Contains isValid boolean and error message if invalid
 */
export function validateCredentials(
  credentials: UserCredentials,
): ValidationResult {
  // Check username format
  const usernameValidation = validateUsername(credentials.username);
  if (!usernameValidation.isValid) {
    return usernameValidation;
  }

  // Check password format
  const passwordValidation = validatePassword(credentials.password);
  if (!passwordValidation.isValid) {
    return passwordValidation;
  }

  // Check role validity
  if (!["staff", "customer"].includes(credentials.role)) {
    return {
      isValid: false,
      errorMessage: "Please select a valid role.",
    };
  }

  return {
    isValid: true,
    errorMessage: "",
  };
}

// ==================== AUTHENTICATION API FUNCTIONS ====================

/**
 * Attempts to log in a user with their credentials.
 * Handles different login results: success, invalid password, or user not found.
 *
 * @param credentials - The user credentials for authentication
 * @returns Promise<AuthResult> - Authentication result with success status and message
 */
export async function authenticateUser(
  credentials: UserCredentials,
): Promise<AuthResult> {
  try {
    // Validate credentials format first
    const validation = validateCredentials(credentials);
    if (!validation.isValid) {
      return {
        success: false,
        message: validation.errorMessage,
      };
    }

    info(
      `Calling Tauri log_in with: ${JSON.stringify({
        username: credentials.username,
      })}`,
    );

    // Attempt login via Tauri backend
    const loginResult: LoginResult = (await invoke("log_in", {
      username: credentials.username,
      password: credentials.password,
    })) as LoginResult;

    info(`Tauri log_in result: ${loginResult}`);

    switch (loginResult) {
      case "success":
        return {
          success: true,
          message: "Login successful.",
        };

      case "invalid-password":
        return {
          success: false,
          message: "Incorrect password. Please try again.",
          invalidPassword: true,
        };

      case "user-not-found":
        return {
          success: false,
          message: "Username not found. Creating account...",
          requiresAccountCreation: true,
        };

      default:
        return {
          success: false,
          message: "Unknown login result. Please try again.",
        };
    }
  } catch (e) {
    error(`Authentication error: ${e}`);
    return {
      success: false,
      message: "Authentication failed. Please try again.",
    };
  }
}

/**
 * Creates a new user account with the provided credentials.
 *
 * @param credentials - The user credentials for account creation
 * @param passwordConfirmation - The password confirmation entered by user
 * @returns Promise<AuthResult> - Account creation result with success status and message
 */
export async function createUserAccount(
  credentials: UserCredentials,
  passwordConfirmation: string,
): Promise<AuthResult> {
  try {
    // Validate credentials format
    const validation = validateCredentials(credentials);
    if (!validation.isValid) {
      return {
        success: false,
        message: validation.errorMessage,
      };
    }

    // Validate password confirmation
    if (credentials.password !== passwordConfirmation) {
      return {
        success: false,
        message: "Password confirmation does not match.",
      };
    }

    if (passwordConfirmation.trim().length === 0) {
      return {
        success: false,
        message: "Please confirm your password.",
      };
    }

    // Create account via Tauri backend
    await invoke("sign_up", {
      username: credentials.username,
      password: credentials.password,
      role: credentials.role,
    });

    return {
      success: true,
      message: "Account created successfully. You are now logged in.",
    };
  } catch (e) {
    error(`Account creation error: ${e}`);

    /** Handle specific error cases */
    const errorMessage: string = e as string;
    if (errorMessage.includes("already exists")) {
      return {
        success: false,
        message: "Username already exists. Please choose a different username.",
      };
    }

    return {
      success: false,
      message: "Account creation failed. Please try again.",
    };
  }
}

// ==================== USER SESSION FUNCTIONS ====================

/**
 * Gets the current logged-in user information.
 *
 * @returns Promise<CurrentUser | null> - Current user info or null if not logged in
 */
export async function getCurrentUser(): Promise<CurrentUser | null> {
  try {
    const currentUser: CurrentUser | null = await invoke("get_current_user");
    return currentUser;
  } catch (e) {
    error(`Get current user error: ${e}`);
    return null;
  }
}

/**
 * Logs out the current user.
 *
 * @returns Promise<boolean> - True if logout successful, false otherwise
 */
export async function logoutUser(): Promise<boolean> {
  try {
    await invoke("log_out");
    return true;
  } catch (e) {
    error(`Logout error: ${e}`);
    return false;
  }
}
