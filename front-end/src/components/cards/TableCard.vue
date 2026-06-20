<template>
  <BudgetItemModal :visible="modalVisible" :modelValue="editingItem" @close="modalVisible = false" @save="saveItem" />
  <div class="table-card">
    <div class="table-header-row">
      <h3>Budget Allocations</h3>
      <button class="btn-add-item" @click="openAddModal()">
        <span class="material-symbols-outlined">add</span>
        Add Item
      </button>
    </div>

    <table class="budget-table">
      <thead>
        <tr>
          <th>Category</th>
          <th>Budget</th>
          <th>Actual</th>
          <th class="actions-header">Actions</th>
        </tr>
      </thead>
      <tbody>
        <tr
          v-for="row in props.budgetData"
          :key="row.category"
          :class="{ 'row-warning': row.actual_amount > row.budget_amount }"
        >
          <td class="category-cell-container">
            <div class="category-cell-inner">
              <span class="color-dot" :style="{ backgroundColor: row.color }"></span>
              {{ row.category }}
            </div>
          </td>

          <td class="numeric-cell">{{ formatCurrency(row.budget_amount) }}</td>

          <td class="numeric-cell">
            <div class="status-cell-inner">
              {{ formatCurrency(row.actual_amount) }}
              <span v-if="row.actual_amount <= row.budget_amount" class="status-icon success-icon">
                <span class="material-symbols-outlined">check</span>
              </span>
              <span v-else class="status-icon warning-icon">
                <span class="material-symbols-outlined">warning</span>
              </span>
            </div>
          </td>
          <td class="actions-cell">
            <button
              class="icon-btn edit-btn"
              title="Edit Item"
              @click="openEditModal(row)"
            >
              <span class="material-symbols-outlined">edit</span>
            </button>
            <button class="icon-btn delete-btn" title="Remove Item" @click="confirmDelete(row.id)">
              <span class="material-symbols-outlined">delete</span>
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { formatCurrency } from '@/shared/utils'
import type { BudgetItem } from '@/models/budget.d'
import { useConfirm } from 'primevue'
import { ref } from 'vue'
import BudgetItemModal from '@/components/BudgetItemModal.vue'

const confirm = useConfirm()
const props = defineProps<{
  budgetData: BudgetItem[],
  onDelete: (id: number) => void,
  onAdd: (item: BudgetItem) => void,
  onEdit: (item: BudgetItem) => void,
}>()

// Delete button
const confirmDelete = (id: number): void => {
  confirm.require({
    message: 'Are you sure you want to delete item?',
    header: 'Confirm',
    icon: 'pi pi-exclamation-triangle',

    accept: () => {
      props.onDelete(id)
    },

    reject: () => {},
  })
}

// Add and edit buttons
const modalVisible = ref(false)
const editingItem = ref<BudgetItem | null>(null)

const openAddModal = () => {
  editingItem.value = null
  modalVisible.value = true
}

const openEditModal = (row: BudgetItem) => {
  editingItem.value = row
  modalVisible.value = true
}

const saveItem = (item: BudgetItem) => {
  if (item.id) {
    props.onEdit(item)
  } else {
    props.onAdd(item)
  }
}
</script>

<style scoped>
.table-card {
  background: var(--color-background-soft);
  border-radius: 16px;
  padding: 24px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.01);
}

.table-header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 18px;
}

.table-header-row h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
  color: var(--color-text-main);
}

.btn-add-item {
  background-color: var(--color-background);
  color: white;
  border: none;
  padding: 8px 14px;
  font-size: 13px;
  font-weight: 500;
  border-radius: 6px;
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  transition: background-color 0.15s;
}

.btn-add-item:hover {
  background-color: #242636;
}

.btn-add-item span {
  font-size: 16px;
}

.budget-table {
  width: 100%;
  border-collapse: collapse;
}

.budget-table th {
  font-size: 13px;
  font-weight: 800;
  color: var(--color-text-main);
  padding: 12px 10px;
  border-bottom: 1px solid #f2f2f7;
  text-align: left;
}

.budget-table .actions-header {
  text-align: right;
}

.budget-table td {
  padding: 14px 10px;
  font-size: 14px;
  color: var(--color-text-main);
  border-bottom: 1px solid #f2f2f7;
  vertical-align: middle;
}

.category-cell-container {
  font-weight: 500;
}

.category-cell-inner {
  display: flex;
  align-items: center;
  gap: 10px;
}

.status-cell-inner {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
  min-width: 100px;
}

.color-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
  flex-shrink: 0;
}

.numeric-cell {
  font-variant-numeric: tabular-nums;
  font-weight: 500;
}

.status-cell {
  display: flex;
  justify-content: space-between;
  align-items: center;
  min-width: 100px;
}

.status-icon {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  border-radius: 50%;
}

.status-icon span {
  font-size: 12px;
  font-weight: bold;
}

.success-icon {
  background-color: #e2f9f0;
  color: #00c781;
}
.warning-icon {
  background-color: #fff1f1;
  color: #ff3b30;
}
.row-warning {
  background-color: rgba(255, 59, 48, 0.02);
}

.actions-header {
  text-align: right;
  padding-right: 14px !important;
}

.actions-cell {
  text-align: right;
  white-space: nowrap;
}

.icon-btn {
  background: transparent;
  border: none;
  cursor: pointer;
  padding: 6px;
  border-radius: 4px;
  color: #8e8e93;
  margin-left: 4px;
  display: inline-flex;
  align-items: center;
  transition:
    color 0.15s,
    background-color 0.15s;
}

.icon-btn span {
  font-size: 18px;
}

.edit-btn:hover {
  color: #0084ff;
  background-color: #f0f7ff;
}

.delete-btn:hover {
  color: #ff3b30;
  background-color: #fff1f1;
}
</style>
