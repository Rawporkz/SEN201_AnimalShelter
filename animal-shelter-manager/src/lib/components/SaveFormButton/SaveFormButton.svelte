<!--
SaveFormButton.svelte

Reusable Save button component.
Follows project Code-Standards: minimal script, typed exports, and JSDoc-style header.
-->

<script lang="ts">
    import "./style.scss";
    import { Save } from '@lucide/svelte';

    /**
     * Button label
     * @type {string}
     */
    let label: string = 'Save';

    /**
     * When true, the button is disabled
     * @type {boolean}
     */
    export let disabled: boolean = false;
</script>


<button
    type="button"
    class="save-button"
    aria-pressed="false"
    aria-label={label}
    {disabled}
>
    <!-- Render save icon inside button-->
    <Save class="save-button__icon" aria-hidden="true" />

    <!-- Render label as button text -->
    {label}

</button>
