<template>
  <div v-if="visible" class="modal-overlay" @click.self="close">
    <div class="modal">
      <div class="modal-header">
        <h3>{{ isEditMode ? 'Edit Budget Item' : 'Add Budget Item' }}</h3>
        <button class="close-btn" @click="close">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label>Category</label>
          <input v-model="form.category" type="text" placeholder="e.g. Groceries" />
        </div>

        <div class="form-group">
          <label>Budget Amount</label>
          <input v-model.number="form.budget_amount" type="number" min="0" />
        </div>

        <div class="form-group">
          <label>Actual Amount</label>
          <input v-model.number="form.actual_amount" type="number" min="0" />
        </div>

        <div class="form-group">
          <label>Color</label>
          <input v-model="form.color" type="color" />
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="close">Cancel</button>
        <button class="btn-primary" @click="submit">
          {{ isEditMode ? 'Save Changes' : 'Add Item' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { reactive, watch, computed } from 'vue'
import type { BudgetItem } from '@/models/budget.d'

const props = defineProps<{
  visible: boolean
  modelValue?: BudgetItem | null
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', value: BudgetItem): void
}>()

const isEditMode = computed(() => !!props.modelValue)

const form = reactive<BudgetItem>({
  id: 0,
  month: new Date("2026/06/01"),
  category: '',
  budget_amount: 0,
  actual_amount: 0,
  color: '#4f46e5',
})

const resetForm = () => {
  form.id = 0
  form.category = ''
  form.budget_amount = 0
  form.actual_amount = 0
  form.color = '#4f46e5'
}

// Sync incoming edit data into form
watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      Object.assign(form, val)
    } else {
      resetForm()
    }
  },
  { immediate: true },
)

const close = () => emit('close')

const submit = () => {
  emit('save', { ...form })
  close()
}
</script>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.35);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.modal {
  width: 420px;
  background: var(--color-background-soft);
  border-radius: 12px;
  padding: 20px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.15);
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.modal-body {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.form-group input {
  padding: 8px 10px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
}

.modal-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 18px;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
  border: none;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
}

.btn-secondary {
  background: transparent;
  border: 1px solid #ddd;
  padding: 8px 12px;
  border-radius: 6px;
  cursor: pointer;
}

.close-btn {
  background: transparent;
  border: none;
  cursor: pointer;
}
</style>
