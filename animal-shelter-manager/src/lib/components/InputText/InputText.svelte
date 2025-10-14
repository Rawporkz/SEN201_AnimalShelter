<!--
InputText.svelte

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
  }

  let {
    label,
    placeholder = "",
    value = $bindable(""),
    boxWidth = "350px",
    rows = 3,
  }: Props = $props();

  /** Last accepted value that fit within the visible rows; used to reject overflow input */
  let prevValue: string = value;

  /**
   * Handle input events. If the new content causes the textarea to overflow
   * its visible area, revert to the previously accepted value to enforce
   * the visual limit.
   *
   * @param e - Input event from the textarea
   */
  function handleInput(e: Event): void {
    const el = e.target as HTMLTextAreaElement;
    // If new content causes overflow, revert completely
    if (el.scrollHeight > el.clientHeight) {
      const cursor = el.selectionStart;
      el.value = prevValue; // revert
      // place cursor at end of allowed content
      el.selectionStart = el.selectionEnd = Math.min(
        cursor - 1,
        el.value.length,
      );
    } else {
      prevValue = el.value; // accept new content
      value = el.value;
    }
  }

  /**
   * Prevent adding newlines when the number of lines already equals the
   * configured visible rows (hard stop behavior).
   *
   * @param e - Keyboard event from the textarea
   */
  function handleKeyDown(e: KeyboardEvent): void {
    if (e.key === "Enter") {
      const el = e.currentTarget as HTMLTextAreaElement;
      const lineCount = el.value.split(/\n/).length;
      // If already at max visible rows, block new line
      if (lineCount >= rows) {
        e.preventDefault();
      }
    }
  }
</script>

<div class="input-field" style="width:{boxWidth};">
  <label for="text-field" class="label">{label}</label>

  <textarea
    id="text-field"
    class="text-field {value ? 'has-content' : ''}"
    {placeholder}
    {rows}
    bind:value
    oninput={handleInput}
    onkeydown={handleKeyDown}
  ></textarea>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
