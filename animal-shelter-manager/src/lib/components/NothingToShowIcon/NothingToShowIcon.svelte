<!--
  NothingToShowIcon.svelte

  Reusable component that displays a "nothing to show" message with a crossed-out circle icon.
  Used when there are no items or results to display in lists, tables, or search results.
-->

<script lang="ts">
  import { CircleOff } from "@lucide/svelte";
</script>

<div class="nothing-to-show-icon">
  <div class="icon-container">
    <CircleOff size="80" />
  </div>

  <p class="message-text">Nothing to Show</p>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
