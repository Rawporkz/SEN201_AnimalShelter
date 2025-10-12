<!--
routes/modal-test/+page.svelte

Demo page to test ConfirmationModel with multiple variations:
1. Deleting an Animal (with radio choices)
2. Rejecting a Request (simple confirmation)
3. Approving a Request (with warning box)
-->

<script lang="ts">
  import ConfirmationModel from "$lib/components/ConfirmationModel/ConfirmationModel.svelte";
  import NormalButton from "$lib/components/NormalButton/NormalButton.svelte";
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
    if (selectedReason === "passed" || selectedReason === "mistake") return false;
    if (selectedReason === "other") return otherText.trim().length === 0;
    return true; // nothing selected
  }

  function onDeleteConfirm() {
    const payload = {
      reason: selectedReason,
      otherText: selectedReason === "other" ? otherText.trim() : undefined,
    };
    console.log("Delete confirmed:", payload);
    deleteOpen = false;
  }

  // Reject modal functions
  function handleRejectOpen() {
    rejectOpen = true;
  }

  function onRejectConfirm() {
    console.log("Request rejected for:", requestorName);
    rejectOpen = false;
  }

  // Approve modal functions
  function handleApproveOpen() {
    approveOpen = true;
  }

  function onApproveConfirm() {
    console.log("Request approved for:", requestorName);
    approveOpen = false;
  }
</script>

<div class="demo">
  <h1>Confirmation Modal Examples</h1>
  
  <div class="demo-buttons">
    <NormalButton 
      text="Delete Animal" 
      color="#ea4444" 
      textColor="#ffffff" 
      width="200px" 
      onClick={handleDeleteOpen} 
    />
    
    <NormalButton 
      text="Reject Request" 
      color="#ea4444" 
      textColor="#ffffff" 
      width="200px" 
      onClick={handleRejectOpen} 
    />
    
    <NormalButton 
      text="Approve Request" 
      color="#00b047" 
      textColor="#ffffff" 
      width="200px" 
      onClick={handleApproveOpen} 
    />
  </div>
</div>

<!-- Delete Animal Modal -->
<ConfirmationModel
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
  on:confirm={onDeleteConfirm}
>
  <div slot="extra">
    <div class="choices">
      <label class="choice">
        <input type="radio" name="reason" value="passed" bind:group={selectedReason} />
        <span>The animal has passed away</span>
      </label>
      <label class="choice">
        <input type="radio" name="reason" value="mistake" bind:group={selectedReason} />
        <span>The animal was added by mistake</span>
      </label>
      <label class="choice">
        <input type="radio" name="reason" value="other" bind:group={selectedReason} />
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
  </div>
</ConfirmationModel>

<!-- Reject Request Modal -->
<ConfirmationModel
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
  on:confirm={onRejectConfirm}
/>

<!-- Approve Request Modal -->
<ConfirmationModel
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
  on:confirm={onApproveConfirm}
>
  <div slot="extra" class="warning-box">
    <div class="warning-icon">⚠️</div>
    <div class="warning-text">
      This will reject all other requests to this animal.
    </div>
  </div>
</ConfirmationModel>
