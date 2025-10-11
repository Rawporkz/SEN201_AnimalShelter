<!--
  NothingToShowIcon.svelte

  Reusable component that displays a "nothing to show" message with a crossed-out circle icon.
  Used when there are no items or results to display in lists, tables, or search results.
-->

<script lang="ts">
  import { CircleOff } from "@lucide/svelte";
  import "./style.scss";
</script>

<div class="nothing-to-show-icon">
  <div class="icon-container">
    <CircleOff size="80" class="nothing-icon" />
  </div>
  
  <p class="message-text">Nothing to Show</p>
</div>
