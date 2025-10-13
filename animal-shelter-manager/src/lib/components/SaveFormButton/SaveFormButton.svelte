<!--
SaveFormButton.svelte

Reusable Save button for forms.
-->

<script lang="ts">
  import { Save } from "@lucide/svelte";

  // Props
  interface Props {
    /** When true, the button is disabled */
    disabled?: boolean;
  }

  const { disabled = false }: Props = $props();
</script>

<button type="button" class="save-button" aria-pressed="false" {disabled}>
  <Save aria-hidden="true" />
  Save
</button>

<style lang="scss">
  @use "./style.scss";
</style>
