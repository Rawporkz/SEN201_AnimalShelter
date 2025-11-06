<!--
ExpandableStatus.svelte

A component that displays a status as a colored, expandable indicator.
-->

<script lang="ts">
  // Props
  interface Props {
    /** The text to display inside the status indicator */
    text: string;
    /** The background color of the status indicator */
    color: string;
  }

  const { text, color }: Props = $props();
</script>

{#if text}
  <div class="status-indicator" style="background-color: {color};">
    <div class="status-tooltip">
      {text}
    </div>
  </div>
{/if}

<style lang="scss">
  @use "./style.scss";
</style>
