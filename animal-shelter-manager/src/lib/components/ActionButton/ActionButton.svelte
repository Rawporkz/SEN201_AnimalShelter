<!--
  ActionButton.svelte

  Standard action button component with fixed blue styling and consistent appearance,
  providing configurable text, icon, and width while maintaining uniform design.
-->

<script lang="ts">
  import type { Component } from "svelte";

  // Props interface
  interface Props {
    /** Text content displayed on the button */
    text: string;
    /** Lucide icon component to display */
    icon: Component<any>;
    /** Width of the button */
    width?: string;
    /** Whether the button is disabled */
    disabled?: boolean;
    /** Click event handler function */
    onclick?: () => void;
  }

  const {
    text,
    icon,
    width = "auto",
    disabled = false,
    onclick = () => {},
  }: Props = $props();
</script>

<button
  class="action-button"
  style="width: {width};"
  {disabled}
  {onclick}
  type="button"
>
  {#if icon}
    {@const IconComponent = icon}
    <div class="button-icon">
      <IconComponent size={18} />
    </div>
  {/if}
  <span class="button-text">{text}</span>
</button>

<style lang="scss">
  @use "./style.scss";
</style>

