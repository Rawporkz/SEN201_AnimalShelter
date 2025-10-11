<!-- 
lib/components/NormalButton/NormalButton.svelte

This file defines a reusable NormalButton component with optional icon support.
-->

<script lang="ts">
    import "./style.scss";

    // Button background color
    export let color: string = "#1e40af";

    // Button text color
    export let textColor: string = "#ffffff";

    // Button text content
    export let text: string = "Button";

    // Icon to display (emoji or text)
    export let icon: string = "";

    // Whether to show the icon
    export let showIcon: boolean = false;

    // Icon position: 'left' or 'right'
    export let iconPosition: "left" | "right" = "left";

    // Text alignment: 'left', 'center', or 'right'
    export let textAlign: "left" | "center" | "right" = "center";

    // Button width
    export let width: string = "150px";

    // Click event handler
    export let onClick: () => void = () => {};

    /**
     * Determines the flex justification based on text alignment
     * @param align - The text alignment value ('left' | 'center' | 'right')
     * @returns string - The corresponding justify-content value
     */
    function getJustifyContent(align: "left" | "center" | "right"): string {
        if (align === "left") {
            return "flex-start";
        } else if (align === "right") {
            return "flex-end";
        } else {
            return "center";
        }
    }
  
</script>

<button
    class="normal-button"
    style="
        background-color: {color};
        color: {textColor};
        width: {width};
        text-align: {textAlign};
        justify-content: {getJustifyContent(textAlign)};
    "
    on:click={onClick}
>
    {#if showIcon && icon && iconPosition === "left"}
        <span class="icon icon-left">{icon}</span>
    {/if}
    <span class="text">{text}</span>
    {#if showIcon && icon && iconPosition === "right"}
        <span class="icon icon-right">{icon}</span>
    {/if}
</button>