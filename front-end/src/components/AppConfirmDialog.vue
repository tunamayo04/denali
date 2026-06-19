<template>
  <ConfirmDialog>
    <template #message="{ message: confirmation }">
      <div class="app-confirm-content">
        <span class="material-symbols-outlined app-confirm-icon">warning</span>
        <p class="app-confirm-message">{{ confirmation.message }}</p>
      </div>
    </template>
  </ConfirmDialog>
</template>

<script setup lang="ts">
import ConfirmDialog from 'primevue/confirmdialog'
</script>

<style>
.p-confirmdialog.p-dialog {
  background: var(--color-background-soft);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 16px;
  box-shadow: 0 16px 48px rgba(0, 0, 0, 0.45);
  color: var(--color-text-main);
  max-width: 420px;
  width: calc(100vw - 48px);
}

.p-dialog-mask.p-overlay-mask {
  background: rgba(0, 0, 0, 0.6);
}

.p-confirmdialog .p-dialog-header {
  padding: 24px 24px 0;
  border: none;
  background: transparent;
}

.p-confirmdialog .p-dialog-title {
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-main);
}

.p-confirmdialog .p-dialog-content {
  padding: 16px 24px 8px;
  background: transparent;
}

.p-confirmdialog .p-dialog-footer {
  padding: 16px 24px 24px;
  border: none;
  background: transparent;
  gap: 10px;
}

.app-confirm-content {
  display: flex;
  align-items: flex-start;
  gap: 14px;
}

.app-confirm-icon {
  font-size: 24px;
  color: var(--c-bright-red);
  background: rgba(231, 75, 75, 0.12);
  border-radius: 50%;
  width: 44px;
  height: 44px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.app-confirm-message {
  margin: 0;
  font-size: 14px;
  line-height: 1.5;
  color: var(--color-text-secondary);
}

.p-confirmdialog .p-confirmdialog-reject-button {
  color: var(--color-text-secondary);
  background: var(--color-background-soft);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  font-size: 13px;
  font-weight: 500;
  padding: 8px 16px;
}

.p-confirmdialog .p-confirmdialog-reject-button:hover {
  color: var(--color-text-main);
  //background: rgba(255, 255, 255, 0.04);
  background-color: var(--color-background-soft);
  border-color: rgba(255, 255, 255, 0.16);
}

.p-confirmdialog .p-confirmdialog-accept-button {
  background: var(--c-medium-red);
  border: 1px solid var(--c-medium-red);
  border-radius: 8px;
  color: var(--c-off-white);
  font-size: 13px;
  font-weight: 500;
  padding: 8px 16px;
}

.p-confirmdialog .p-confirmdialog-accept-button:hover {
  background: var(--c-bright-red);
  border-color: var(--c-bright-red);
}
</style>
