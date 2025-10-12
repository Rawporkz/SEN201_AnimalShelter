<!--
SaveFormButton.svelte

Reusable Save button component.
Follows project Code-Standards: minimal script, typed exports, and JSDoc-style header.
-->

<script lang="ts">
    import "./style.scss";

    /**
     * Button label (default: 'Save')
     * @type {string}
     */
    export let label: string = 'Save';

    /**
     * When true, the button is disabled
     * @type {boolean}
     */
    export let disabled: boolean = false;

    /**
     * Per-instance icon sizing (CSS values). Keep default in sync with SCSS (21px).
     * @type {string}
     */
    export let iconWidth: string = '21px';
    export let iconHeight: string = '21px';

    /**
     * Unique mask id for the SVG to avoid collisions when multiple components render.
     * @type {string}
     */
    const maskId: string = `saveDiskMask-${Math.random().toString(36).slice(2, 9)}`;

    /**
     * Note: Native DOM attributes/events passed to the component are forwarded
     * via Svelte's built-in `$$restProps` (e.g. aria, data-*, inline style).
     */
</script>

<button
    type="button"
    class="save-button"
    aria-pressed="false"
    aria-label={label}
    {disabled}
    {...$$restProps}
>
    <span class="save-button__icon" aria-hidden="true" style={`width: ${iconWidth}; height: ${iconHeight};`}>
        <svg viewBox="0 0 24 24" preserveAspectRatio="none" xmlns="http://www.w3.org/2000/svg" focusable="false" role="img" aria-hidden="true">
            <!-- Filled floppy-disk with transparent holes for the inner rectangle and circle using an SVG mask -->
            <defs>
                <mask id={maskId} maskUnits="userSpaceOnUse" x="0" y="0" width="24" height="24">
                    <!-- white = visible, black = transparent (hole) -->
                    <rect x="0" y="0" width="24" height="24" fill="white" />
                    <rect x="7" y="6.5" width="9" height="3" rx="0.6" fill="black" />
                    <circle cx="12" cy="15" r="3" fill="black" />
                </mask>
            </defs>

            <!-- Draw the filled disk shapes, but apply the mask so the rect & circle become transparent holes -->
            <g mask={`url(#${maskId})`}>
                <path d="M6 2h9l5 5v12a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2z" fill="#ffffff" />
                <path d="M15 3.5V8h4.5L15 3.5z" fill="#ffffff" opacity="0.98" />
            </g>
        </svg>
    </span>

    <span class="save-button__label">{label}</span>
</button>
