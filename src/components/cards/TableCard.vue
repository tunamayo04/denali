<template>
  <div class="table-card">
    <div class="table-header-row">
      <h3>Budget Allocations</h3>
      <button class="btn-add-item" @click="triggerAction('Add Item')">
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
          :class="{ 'row-warning': row.actual > row.budget }"
        >
          <td class="category-cell-container">
            <div class="category-cell-inner">
              <span class="color-dot" :style="{ backgroundColor: row.color }"></span>
              {{ row.category }}
            </div>
          </td>

          <td class="numeric-cell">{{ formatCurrency(row.budget) }}</td>

          <td class="numeric-cell">
            <div class="status-cell-inner">
              {{ formatCurrency(row.actual) }}
              <span v-if="row.actual <= row.budget" class="status-icon success-icon">
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
              @click="triggerAction(`Edit ${row.category}`)"
            >
              <span class="material-symbols-outlined">edit</span>
            </button>
            <button
              class="icon-btn delete-btn"
              title="Remove Item"
              @click="triggerAction(`Remove ${row.category}`)"
            >
              <span class="material-symbols-outlined">delete</span>
            </button>
          </td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script setup lang="ts">
import { formatCurrency } from '@/utils.js'
import type { BudgetRow } from '@/dtos/budget.js'

const props = defineProps<{
  budgetData: BudgetRow[]
}>()

// Frontend Placeholder click logger
const triggerAction = (actionName: string) => {
  alert(`Frontend Action Called: "${actionName}"\n(Hook dynamic CRUD logic / modals here)`)
}
</script>

<style scoped>
.table-card {
  background: #ffffff;
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
  color: #1c1f21;
}

.btn-add-item {
  background-color: #0f101a;
  color: #fff;
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
  font-weight: 500;
  color: #8e8e93;
  padding: 12px 10px;
  border-bottom: 1px solid #f2f2f7;
}

.budget-table td {
  padding: 14px 10px;
  font-size: 14px;
  color: #1c1f21;
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
