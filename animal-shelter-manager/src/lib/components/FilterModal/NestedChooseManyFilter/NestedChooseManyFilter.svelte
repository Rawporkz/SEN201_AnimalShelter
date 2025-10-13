<!--
FilterModal/NestedChooseManyFilter.svelte

Filter component for criteria that allow multiple selections with
nested hierarchical structure. Presents options as checkboxes
organized in two levels where users can select items from
different categories and subcategories.
-->

<script lang="ts">
  // Props
  interface Props {
    /** The title to display for this filter */
    title: string;
    /** Nested options with categories and their items */
    nestedOptions: Record<string, { label: string; items: string[] }>;
    /** Currently selected values organized by category */
    selectedValues: Record<string, string[]>;
    /** Callback when selections change */
    onSelect: (selectedOptions: Record<string, string[]>) => void;
  }

  const { title, nestedOptions, selectedValues, onSelect }: Props = $props();

  /** Currently selected category for showing items on the right */
  let selectedCategory: string | null = $state(
    Object.keys(nestedOptions).length > 0
      ? Object.keys(nestedOptions)[0]
      : null,
  );

  /**
   * Checks if a category is fully selected.
   *
   * @param category - The category to check
   * @returns boolean - True if all items in category are selected
   */
  function isCategoryFullySelected(category: string): boolean {
    const items = nestedOptions[category]?.items || [];
    const selected = selectedValues[category] || [];
    return items.length > 0 && selected.length === items.length;
  }

  /**
   * Checks if a category is partially selected.
   *
   * @param category - The category to check
   * @returns boolean - True if some but not all items are selected
   */
  function isCategoryPartiallySelected(category: string): boolean {
    const items = nestedOptions[category]?.items || [];
    const selected = selectedValues[category] || [];
    return selected.length > 0 && selected.length < items.length;
  }

  /**
   * Handles clicking on a category.
   *
   * @param category - The category that was clicked
   */
  function handleCategoryClick(category: string): void {
    selectedCategory = category;
  }

  /**
   * Handles category checkbox changes.
   *
   * @param category - The category being toggled
   * @param isChecked - Whether the checkbox is now checked
   */
  function handleCategoryCheckboxChange(
    category: string,
    isChecked: boolean,
  ): void {
    const items = nestedOptions[category]?.items || [];
    const newSelections = { ...selectedValues };

    if (isChecked) {
      newSelections[category] = [...items];
    } else {
      newSelections[category] = [];
    }

    onSelect(newSelections);
  }

  /**
   * Handles individual item checkbox changes.
   *
   * @param category - The category the item belongs to
   * @param item - The item being toggled
   * @param isChecked - Whether the checkbox is now checked
   */
  function handleItemCheckboxChange(
    category: string,
    item: string,
    isChecked: boolean,
  ): void {
    const newSelections = { ...selectedValues };
    const categorySelections = newSelections[category] || [];

    if (isChecked) {
      newSelections[category] = [...categorySelections, item];
    } else {
      newSelections[category] = categorySelections.filter(
        (selectedItem) => selectedItem !== item,
      );
    }

    onSelect(newSelections);
  }

  /**
   * Handles select all functionality.
   */
  function handleSelectAll(): void {
    const newSelections: Record<string, string[]> = {};

    for (const category in nestedOptions) {
      newSelections[category] = [...nestedOptions[category].items];
    }

    onSelect(newSelections);
  }

  /**
   * Handles deselect all functionality.
   */
  function handleDeselectAll(): void {
    const newSelections: Record<string, string[]> = {};

    for (const category in nestedOptions) {
      newSelections[category] = [];
    }

    onSelect(newSelections);
  }
</script>

<div class="nested-choose-many-filter">
  <div class="filter-header">
    <h3 class="filter-title">{title}</h3>
    <div class="filter-actions">
      <button type="button" class="action-btn" onclick={handleSelectAll}>
        Select All
      </button>
      <span class="separator">/</span>
      <button type="button" class="action-btn" onclick={handleDeselectAll}>
        Deselect All
      </button>
    </div>
  </div>

  <div class="filter-body">
    <div class="categories-column">
      {#each Object.keys(nestedOptions) as category (category)}
        <div
          class="category-item {selectedCategory === category ? 'active' : ''}"
        >
          <label class="category-label">
            <input
              type="checkbox"
              class="category-checkbox"
              checked={isCategoryFullySelected(category)}
              indeterminate={isCategoryPartiallySelected(category)}
              onchange={(e) =>
                handleCategoryCheckboxChange(category, e.currentTarget.checked)}
            />
            <span class="category-name">{nestedOptions[category].label}</span>
          </label>
          <button
            type="button"
            class="category-arrow"
            onclick={() => handleCategoryClick(category)}
          >
            ›
          </button>
        </div>
      {/each}
    </div>

    <div class="items-column">
      {#if selectedCategory && nestedOptions[selectedCategory]}
        {#each nestedOptions[selectedCategory].items as item (item)}
          <label class="item-label">
            <input
              type="checkbox"
              class="item-checkbox"
              checked={(selectedValues[selectedCategory] || []).includes(item)}
              onchange={(e) =>
                handleItemCheckboxChange(
                  selectedCategory!,
                  item,
                  e.currentTarget.checked,
                )}
            />
            <span class="item-name">{item}</span>
          </label>
        {/each}
      {/if}
    </div>
  </div>
</div>

<style lang="scss">
  @use "./style.scss";
</style>
