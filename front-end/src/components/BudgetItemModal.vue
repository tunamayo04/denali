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
          <input
            v-model="category"
            v-bind="categoryAttrs"
            type="text"
            placeholder="e.g. Groceries"
            :class="{ 'has-error': errors.category }"
          />
          <span v-if="errors.category" class="error-message">{{ errors.category }}</span>
        </div>

        <div class="form-group">
          <label>Budget Amount</label>
          <input
            v-model.number="budget_amount"
            v-bind="budgetAmountAttrs"
            type="number"
            min="0"
            step="0.01"
            :class="{ 'has-error': errors.budget_amount }"
          />
          <span v-if="errors.budget_amount" class="error-message">{{ errors.budget_amount }}</span>
        </div>

        <div class="form-group">
          <label>Actual Amount</label>
          <input
            v-model.number="actual_amount"
            v-bind="actualAmountAttrs"
            type="number"
            min="0"
            step="0.01"
            :class="{ 'has-error': errors.actual_amount }"
          />
          <span v-if="errors.actual_amount" class="error-message">{{ errors.actual_amount }}</span>
        </div>

        <div class="form-group">
          <label>Color</label>
          <input v-model="color" v-bind="colorAttrs" type="color" />
          <span v-if="errors.color" class="error-message">{{ errors.color }}</span>
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
import { watch, computed, ref } from 'vue'
import { useForm } from 'vee-validate'
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

type BudgetFormValues = Pick<
  BudgetItem,
  'category' | 'budget_amount' | 'actual_amount' | 'color'
>

const defaultValues: BudgetFormValues = {
  category: '',
  budget_amount: 0,
  actual_amount: 0,
  color: '#4f46e5',
}

const id = ref(0)
const month = ref<Date>(new Date('2026/06/01'))

const validationSchema = {
  category(value: unknown) {
    if (typeof value !== 'string' || !value.trim()) {
      return 'Category is required'
    }
    return true
  },
  budget_amount(value: unknown) {
    if (value === '' || value === null || value === undefined) {
      return 'Budget amount is required'
    }
    if (typeof value !== 'number' || Number.isNaN(value)) {
      return 'Budget amount must be a number'
    }
    if (value < 0) {
      return 'Budget amount cannot be negative'
    }
    return true
  },
  actual_amount(value: unknown) {
    if (value === '' || value === null || value === undefined) {
      return 'Actual amount is required'
    }
    if (typeof value !== 'number' || Number.isNaN(value)) {
      return 'Actual amount must be a number'
    }
    if (value < 0) {
      return 'Actual amount cannot be negative'
    }
    return true
  },
  color(value: unknown) {
    if (typeof value !== 'string' || !value) {
      return 'Color is required'
    }
    return true
  },
}

const { handleSubmit, errors, defineField, resetForm } = useForm<BudgetFormValues>({
  validationSchema,
  initialValues: { ...defaultValues },
})

const [category, categoryAttrs] = defineField('category')
const [budget_amount, budgetAmountAttrs] = defineField('budget_amount')
const [actual_amount, actualAmountAttrs] = defineField('actual_amount')
const [color, colorAttrs] = defineField('color')

watch(
  () => props.modelValue,
  (val) => {
    if (val) {
      id.value = val.id
      month.value = val.month
      resetForm({
        values: {
          category: val.category,
          budget_amount: val.budget_amount,
          actual_amount: val.actual_amount,
          color: val.color,
        },
      })
    } else {
      id.value = 0
      month.value = new Date('2026/06/01')
      resetForm({ values: { ...defaultValues } })
    }
  },
  { immediate: true },
)

const close = () => emit('close')

const submit = handleSubmit((values) => {
  emit('save', {
    id: id.value,
    month: month.value,
    ...values,
  })
  close()
})
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
  border: 1px solid transparent;
  border-radius: 6px;
  font-size: 14px;
}

.form-group input.has-error {
  border-color: #dc2626;
}

.error-message {
  color: #dc2626;
  font-size: 12px;
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
