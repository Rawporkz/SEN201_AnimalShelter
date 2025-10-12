<!--
lib/components/ConfirmationModel/ConfirmationModel.svelte

Reusable confirmation modal with:
- NormalButton-based actions
- Named slot `extra` to inject custom content (e.g., warning box, dropdown)
- Cancel/Confirm events and ESC/backdrop handling
-->

<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import NormalButton from "../NormalButton/NormalButton.svelte";
    import "./style.scss";

    // Visibility (bindable)
    export let open: boolean = false;

    // Content
    export let title: string = "Confirm Action";
    export let message: string = "Are you sure you want to proceed?";

        // Sizing
        export let width: string = "600px";
        // Inner content width: keeps body/actions width constant while frame padding can grow
        export let contentWidth: string = "360px";

    // Actions
    export let confirmText: string = "Confirm";
    export let cancelText: string = "Cancel";
    export let destructive: boolean = false; // if true, confirm emphasized as destructive

    // Button states
    export let confirmDisabled: boolean = false;
    export let cancelDisabled: boolean = false;

    // Button colors (overridable)
    export let confirmColor: string = destructive ? "#ea4444" : "#00b047"; // red or green
    export let confirmTextColor: string = "#ffffff";
    export let cancelColor: string = "#e5e7eb"; // light gray
    export let cancelTextColor: string = "#111827"; // near-black

    // Button styling
    export let confirmButtonWidth: string = "150px";
    export let cancelButtonWidth: string = "150px";

    // Behavior
    export let closeOnEscape: boolean = true;
    export let closeOnBackdropClick: boolean = true;

    const dispatch = createEventDispatcher();

    function handleConfirm() {
        if (confirmDisabled) return;
        dispatch("confirm");
        open = false;
    }

    function handleCancel() {
        if (cancelDisabled) return;
        dispatch("cancel");
        open = false;
    }

    function onBackdrop(e: MouseEvent) {
        if (!closeOnBackdropClick) return;
        // Close only when clicking the overlay itself
        if ((e.target as HTMLElement)?.classList.contains("modal-overlay")) {
            handleCancel();
        }
    }

        function onBackdropKeydown(e: KeyboardEvent) {
            if (!closeOnBackdropClick) return;
            const target = e.target as HTMLElement;
            if (!target?.classList.contains("modal-overlay")) return;
            if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                handleCancel();
            }
        }

    function onKeydown(e: KeyboardEvent) {
        if (!closeOnEscape) return;
        if (e.key === "Escape" && open) {
            handleCancel();
        }
    }
  
</script>

<svelte:window on:keydown={onKeydown} />

{#if open}
    <div
        class="modal-overlay"
        on:click={onBackdrop}
        on:keydown={onBackdropKeydown}
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
                <div class="modal__content" style={`width: ${contentWidth}`}> 
                    <header class="modal__header">
                        <h3 id="cm-title" class="modal__title">{title}</h3>
                    </header>

                    <section class="modal__body">
                        {#if message}
                            <p class="modal__message">{message}</p>
                        {/if}
                        <!-- Extra content slot (e.g., warning box, dropdown, etc.) -->
                        <div class="modal__extra">
                            <slot name="extra" />
                        </div>
                    </section>

                    <footer class="modal__actions">
                        <NormalButton
                            text={cancelText}
                            color={cancelColor}
                            textColor={cancelTextColor}
                            width={cancelButtonWidth}
                            disabled={cancelDisabled}
                            onClick={handleCancel}
                        />
                        <NormalButton
                            text={confirmText}
                            color={confirmColor}
                            textColor={confirmTextColor}
                            width={confirmButtonWidth}
                            disabled={confirmDisabled}
                            onClick={handleConfirm}
                        />
                    </footer>
                </div>
        </div>
    </div>
{/if}

<style>
    /* Keep component-scoped layout fallbacks if SCSS fails to load */
    .modal-overlay { position: fixed; inset: 0; display: grid; place-items: center; background: rgba(0,0,0,.35); z-index: 50; }
    .modal { background: white; border-radius: 16px; padding: 20px; max-width: 90vw; box-shadow: 0 10px 30px rgba(0,0,0,.2); }
        .modal__content { width: 360px; max-width: 100%; margin: 0 auto; }
    .modal__actions { display: flex; gap: 24px; justify-content: center; margin-top: 22px; }
    .modal__extra:empty { display: none; }
    .modal__title { margin: 0; }
</style>