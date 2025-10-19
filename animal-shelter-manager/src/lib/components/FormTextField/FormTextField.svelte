<!--
FormTextField.svelte

Reusable textarea component with configurable number of visible rows and width.
-->

<script lang="ts">
  // Props
  interface Props {
    /** The visible label shown above the textarea */
    label: string;
    /** Placeholder text shown when the field is empty */
    placeholder?: string;
    /** Two-way bound value of the textarea */
    value?: string;
    /** Width of the component (CSS value, e.g. '350px' or '100%') */
    boxWidth?: string;
    /** Number of visible rows for the textarea (controls height) */
    rows?: number;
    /** Flag to indicate if the input is invalid */
    isInvalid?: boolean;
  }

  let {
    label,
    placeholder = "",
    value = $bindable(""),
    boxWidth = "350px",
    rows = 3,
    isInvalid = false,
  }: Props = $props();

  /*
   * Prevent adding newlines in single-line mode.
   * @param e - Keyboard event from the textarea
   */
  function handleKeyDown(e: KeyboardEvent): void {
    if (rows === 1 && e.key === "Enter") {
      e.preventDefault();
    }
  }
</script>

<div class="input-field" style="width:{boxWidth};">
  <label for="text-field" class="label">{label}</label>

  <textarea
    id="text-field"
    class="text-field {value ? 'has-content' : ''}"
    class:single-line={rows === 1}
    class:multi-line={rows > 1}
    class:error-border={isInvalid}
    {placeholder}
    {rows}
    bind:value
    onkeydown={handleKeyDown}
  ></textarea>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
