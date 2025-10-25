<!--
ConfirmationModal.svelte

Reusable confirmation modal component with customizable actions and content,
providing cancel/confirm buttons, keyboard navigation, and backdrop interaction.
-->

<script lang="ts">
  import GenericButton from "$lib/components/GenericButton/GenericButton.svelte";
  import type { Snippet } from "svelte";

  // Props interface
  interface Props {
    /** Whether the modal is currently visible and open */
    open?: boolean;
    /** Title text displayed in the modal header */
    title?: string;
    /** Main message text displayed in the modal body */
    message?: string;
    /** Overall width of the modal container */
    width?: string;
    /** Inner content width for consistent body/actions sizing */
    contentWidth?: string;
    /** Text displayed on the confirm button */
    confirmText?: string;
    /** Text displayed on the cancel button */
    cancelText?: string;
    /** Whether to style the confirm action as destructive (red) */
    destructive?: boolean;
    /** Whether the confirm button is disabled */
    confirmDisabled?: boolean;
    /** Whether the cancel button is disabled */
    cancelDisabled?: boolean;
    /** Background color of the confirm button */
    confirmColor?: string;
    /** Text color of the confirm button */
    confirmTextColor?: string;
    /** Background color of the cancel button */
    cancelColor?: string;
    /** Text color of the cancel button */
    cancelTextColor?: string;
    /** Width of the confirm button */
    confirmButtonWidth?: string;
    /** Width of the cancel button */
    cancelButtonWidth?: string;
    /** Whether to close modal on Escape key press */
    closeOnEscape?: boolean;
    /** Whether to close modal when clicking backdrop */
    closeOnBackdropClick?: boolean;
    /** Extra content slot for custom components */
    extra?: Snippet;
    /** Callback function called when confirm button is clicked */
    onconfirm?: () => void;
    /** Callback function called when cancel button is clicked */
    oncancel?: () => void;
  }

  let {
    open = $bindable(false),
    title = "Confirm Action",
    message = "Are you sure you want to proceed?",
    width = "365px",
    contentWidth = "275px",
    confirmText = "Confirm",
    cancelText = "Cancel",
    destructive = false,
    confirmDisabled = false,
    cancelDisabled = false,
    confirmColor,
    confirmTextColor = "#ffffff",
    cancelColor = "#e5e7eb",
    cancelTextColor = "#111827",
    confirmButtonWidth = "150px",
    cancelButtonWidth = "150px",
    closeOnEscape = true,
    closeOnBackdropClick = true,
    extra,
    onconfirm,
    oncancel,
  }: Props = $props();

  /** Reactive confirm button color based on destructive prop */
  let computedConfirmColor: string = $derived(
    confirmColor || (destructive ? "#ea4444" : "#00b047"),
  );

  /**
   * Handles the confirm button click, calling the confirm callback and closing modal.
   * @returns void
   */
  function handleConfirm(): void {
    if (confirmDisabled) return;
    if (onconfirm) {
      onconfirm();
    }
    open = false;
  }

  /**
   * Handles the cancel button click, calling the cancel callback and closing modal.
   * @returns void
   */
  function handleCancel(): void {
    if (cancelDisabled) return;
    if (oncancel) {
      oncancel();
    }
    open = false;
  }

  /**
   * Handles mouse clicks on the backdrop overlay to close the modal.
   * @param e - The mouse event from clicking on the backdrop
   * @returns void
   */
  function onBackdrop(e: MouseEvent): void {
    if (!closeOnBackdropClick) return;
    // Close only when clicking the overlay itself
    if ((e.target as HTMLElement)?.classList.contains("modal-overlay")) {
      handleCancel();
    }
  }

  /**
   * Handles keyboard navigation on the backdrop overlay.
   * @param e - The keyboard event triggered on the backdrop
   * @returns void
   */
  function onBackdropKeydown(e: KeyboardEvent): void {
    if (!closeOnBackdropClick) return;
    const target = e.target as HTMLElement;
    if (!target?.classList.contains("modal-overlay")) return;
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      handleCancel();
    }
  }

  /**
   * Handles global keydown events, specifically Escape key to close modal.
   * @param e - The keyboard event triggered globally
   * @returns void
   */
  function onKeydown(e: KeyboardEvent): void {
    if (!closeOnEscape) return;
    if (e.key === "Escape" && open) {
      handleCancel();
    }
  }
</script>

<svelte:window onkeydown={onKeydown} />

{#if open}
  <div
    class="modal-overlay"
    onclick={onBackdrop}
    onkeydown={onBackdropKeydown}
    role="button"
    aria-label="Close modal"
    tabindex="0"
  >
    <div
      class="modal"
      style={`width: ${width}; --content-width: ${contentWidth}`}
      role="dialog"
      aria-modal="true"
      aria-labelledby="cm-title"
      tabindex="-1"
    >
      <div class="modal-content" style={`width: ${contentWidth}`}>
        <header class="modal-header">
          <h3 id="cm-title" class="modal-title">{title}</h3>
        </header>

        <section class="modal-body">
          {#if message}
            <p class="modal-message">{message}</p>
          {/if}
          <div class="modal-extra">
            {#if extra}
              {@render extra()}
            {/if}
          </div>
        </section>

        <footer class="modal-actions">
          <GenericButton
            text={cancelText}
            color={cancelColor}
            textColor={cancelTextColor}
            width={cancelButtonWidth}
            disabled={cancelDisabled}
            onclick={handleCancel}
          />
          <GenericButton
            text={confirmText}
            color={computedConfirmColor}
            textColor={confirmTextColor}
            width={confirmButtonWidth}
            disabled={confirmDisabled}
            onclick={handleConfirm}
          />
        </footer>
      </div>
    </div>
  </div>
{/if}

<style lang="scss">
  @use "./style.scss";
</style>
