<script lang="ts">
  import "./style.scss";
  // Props
  export let label: string;
  export let placeholder: string = '';
  export let value: string = '';
  export let boxWidth: string = '350px'; // still pixel based width
  export let rows: number = 3; // visible text rows
  // Deprecated: boxHeight & limitToBox removed; sizing now via rows attribute

  let textareaEl: HTMLTextAreaElement;
  let prevValue = value; // last value that fit

  function handleInput(e: Event) {
    const el = e.target as HTMLTextAreaElement;
    // If new content causes overflow, revert completely
    if (el.scrollHeight > el.clientHeight) {
      const cursor = el.selectionStart;
      el.value = prevValue; // revert
      // place cursor at end of allowed content
      el.selectionStart = el.selectionEnd = Math.min(cursor - 1, el.value.length);
    } else {
      prevValue = el.value; // accept new content
      value = el.value;
    }
  }

  function handleKeyDown(e: KeyboardEvent) {
    if (e.key === 'Enter') {
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
  <label for="textbox-input" class="label">{label}</label>
  <div class="box {value ? 'has-content' : ''}">
    <textarea
      id="textbox-input"
      class="text-box {value ? 'has-content' : ''}"
      placeholder={placeholder}
      rows={rows}
      bind:this={textareaEl}
      bind:value={value}
      on:input={handleInput}
      on:keydown={handleKeyDown}
    ></textarea>
  </div>
</div>