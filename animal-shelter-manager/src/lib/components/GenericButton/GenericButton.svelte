<!--
  GenericButton.svelte

  Generic button component with fully customizable styling, text, and optional icon support,
  providing complete flexibility for various use cases throughout the application.
-->

<script lang="ts">
  import type { Component } from "svelte";

  // Props interface
  interface Props {
    /** Background color of the button */
    color?: string;
    /** Text color of the button */
    textColor?: string;
    /** Text content displayed on the button */
    text?: string;
    /** Lucide icon component to display */
    icon?: Component<any>;
    /** Whether to show the icon */
    showIcon?: boolean;
    /** Position of the icon relative to text */
    iconPosition?: "left" | "right";
    /** Alignment of button text and content */
    textAlign?: "left" | "center" | "right";
    /** Width of the button */
    width?: string;
    /** Whether the button is disabled */
    disabled?: boolean;
    /** Click event handler function */
    onclick?: () => void;
  }

  const {
    color = "#1e40af",
    textColor = "#ffffff",
    text = "Button",
    icon,
    showIcon = false,
    iconPosition = "left",
    textAlign = "center",
    width = "150px",
    disabled = false,
    onclick = () => {},
  }: Props = $props();

  /**
   * Determines the CSS flex justification property based on text alignment.
   * @param align - The text alignment value specifying content positioning
   * @returns The corresponding justify-content CSS value
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
  class="generic-button"
  style="
        background-color: {color};
        color: {textColor};
        width: {width};
        text-align: {textAlign};
        justify-content: {getJustifyContent(textAlign)};
    "
  {disabled}
  {onclick}
>
  {#if showIcon && icon && iconPosition === "left"}
    {@const IconComponent = icon}
    <div class="icon icon-left">
      <IconComponent size={18} />
    </div>
  {/if}
  <span class="text">{text}</span>
  {#if showIcon && icon && iconPosition === "right"}
    {@const IconComponent = icon}
    <div class="icon icon-right">
      <IconComponent size={18} />
    </div>
  {/if}
</button>

<style lang="scss">
  @use "./style.scss";
</style>

