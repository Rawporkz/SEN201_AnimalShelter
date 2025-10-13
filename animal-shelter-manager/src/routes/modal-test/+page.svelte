<!--
routes/modal-test/+page.svelte

Demo page to test ConfirmationModal with multiple variations:
1. Deleting an Animal (with radio choices)
2. Rejecting a Request (simple confirmation)
3. Approving a Request (with warning box)
-->

<script lang="ts">
  import ConfirmationModal from "$lib/components/ConfirmationModal/ConfirmationModal.svelte";
  import GenericButton from "$lib/components/GenericButton/GenericButton.svelte";
  import ActionButton from "$lib/components/ActionButton/ActionButton.svelte";
  import ActionDropdownButton from "$lib/components/ActionDropdownButton/ActionDropdownButton.svelte";
  import { Settings, Trash2, Eye, Pencil } from "@lucide/svelte";
  import { info } from "@tauri-apps/plugin-log";
  import "./style.scss";

  // Modal states
  let deleteOpen = false;
  let rejectOpen = false;
  let approveOpen = false;

  // Delete modal state
  let selectedReason: "passed" | "mistake" | "other" | null = null;
  let otherText = "";

  const animalName = "Bella";
  const requestorName = "John Smith";

  // Delete modal functions
  function handleDeleteOpen() {
    deleteOpen = true;
    selectedReason = null;
    otherText = "";
  }

  function deleteConfirmDisabled(): boolean {
    if (selectedReason === "passed" || selectedReason === "mistake")
      return false;
    if (selectedReason === "other") return otherText.trim().length === 0;
    return true; // nothing selected
  }

  function onDeleteConfirm() {
    const payload = {
      reason: selectedReason,
      otherText: selectedReason === "other" ? otherText.trim() : undefined,
    };
    info(`Delete confirmed: ${JSON.stringify(payload)}`);
  }

  function onDeleteCancel() {
    info("Delete cancelled");
  }

  // Reject modal functions
  function handleRejectOpen() {
    rejectOpen = true;
  }

  function onRejectConfirm() {
    info(`Request rejected for: ${requestorName}`);
  }

  function onRejectCancel() {
    info("Reject cancelled");
  }

  // Approve modal functions
  function handleApproveOpen() {
    approveOpen = true;
  }

  function onApproveConfirm() {
    info(`Request approved for: ${requestorName}`);
  }

  function onApproveCancel() {
    info("Approve cancelled");
  }

  // Dropdown button options
  const dropdownOptions = [
    {
      label: "View Details",
      icon: Eye,
      onclick: () => info("View Details clicked"),
    },
    {
      label: "Edit Animal",
      icon: Pencil,
      onclick: () => info("Edit Animal clicked"),
    },
    {
      label: "Delete Animal",
      icon: Trash2,
      onclick: () => info("Delete Animal clicked from dropdown"),
    },
  ];
</script>

<div class="demo">
  <h1>Component Testing Examples</h1>

  <div class="demo-section">
    <h2>Confirmation Modals</h2>
    <div class="demo-buttons">
      <GenericButton
        text="Delete Animal"
        color="#ea4444"
        textColor="#ffffff"
        width="200px"
        onclick={handleDeleteOpen}
      />

      <GenericButton
        text="Reject Request"
        color="#ea4444"
        textColor="#ffffff"
        width="200px"
        onclick={handleRejectOpen}
      />

      <GenericButton
        text="Approve Request"
        color="#00b047"
        textColor="#ffffff"
        width="200px"
        onclick={handleApproveOpen}
      />
    </div>
  </div>

  <div class="demo-section">
    <h2>Action Components</h2>
    <div class="demo-buttons">
      <ActionDropdownButton
        label="Animal Actions"
        icon={Settings}
        options={dropdownOptions}
        title="Animal management actions"
      />
      <ActionButton
        text="Animal Actions"
        icon={Settings}
        onclick={() => info("ActionButton clicked")}
      />
    </div>
  </div>
</div>

<!-- Delete Animal Modal -->
<ConfirmationModal
  bind:open={deleteOpen}
  title="Deleting an Animal"
  message={`Please provide a reason of the deletion of ${animalName}.`}
  cancelText="Cancel"
  cancelColor="#003a62"
  cancelTextColor="#ffffff"
  confirmText="Confirm"
  confirmColor="#ea4444"
  confirmTextColor="#ffffff"
  confirmDisabled={deleteConfirmDisabled()}
  destructive={true}
  onconfirm={onDeleteConfirm}
  oncancel={onDeleteCancel}
>
  {#snippet extra()}
    <div class="choices">
      <label class="choice">
        <input
          type="radio"
          name="reason"
          value="passed"
          bind:group={selectedReason}
        />
        <span>The animal has passed away</span>
      </label>
      <label class="choice">
        <input
          type="radio"
          name="reason"
          value="mistake"
          bind:group={selectedReason}
        />
        <span>The animal was added by mistake</span>
      </label>
      <label class="choice">
        <input
          type="radio"
          name="reason"
          value="other"
          bind:group={selectedReason}
        />
        <span>Other</span>
      </label>

      {#if selectedReason === "other"}
        <textarea
          class="other-text"
          bind:value={otherText}
          placeholder="Type Here..."
          rows={4}
        ></textarea>
      {/if}
    </div>
  {/snippet}
</ConfirmationModal>

<!-- Reject Request Modal -->
<ConfirmationModal
  bind:open={rejectOpen}
  title="Rejecting a Request"
  message={`Are you sure you want to reject this adoption request by ${requestorName}?`}
  cancelText="Cancel"
  cancelColor="#003a62"
  cancelTextColor="#ffffff"
  confirmText="Reject"
  confirmColor="#ea4444"
  confirmTextColor="#ffffff"
  destructive={true}
  contentWidth="400px"
  onconfirm={onRejectConfirm}
  oncancel={onRejectCancel}
/>

<!-- Approve Request Modal -->
<ConfirmationModal
  bind:open={approveOpen}
  title="Approving a Request"
  message={`Are you sure you want to approve this adoption request by ${requestorName}?`}
  cancelText="Cancel"
  cancelColor="#003a62"
  cancelTextColor="#ffffff"
  confirmText="Approve"
  confirmColor="#00b047"
  confirmTextColor="#ffffff"
  destructive={false}
  contentWidth="400px"
  onconfirm={onApproveConfirm}
  oncancel={onApproveCancel}
>
  {#snippet extra()}
    <div class="warning-box">
      <div class="warning-icon">⚠️</div>
      <div class="warning-text">
        This will reject all other requests to this animal.
      </div>
    </div>
  {/snippet}
</ConfirmationModal>
