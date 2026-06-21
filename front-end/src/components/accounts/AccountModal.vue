<template>
  <div v-if="visible" class="modal-overlay" @click.self="close">
    <div class="modal">
      <div class="modal-header">
        <h3>Add Account</h3>
        <button class="close-btn" @click="close">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>

      <div class="modal-body">
        <div class="form-group">
          <label>Name</label>
          <input
            v-model="name"
            v-bind="nameAttrs"
            type="text"
            placeholder="e.g. Everyday Checking"
            :class="{ 'has-error': errors.name }"
          />
          <span v-if="errors.name" class="error-message">{{ errors.name }}</span>
        </div>

        <div class="form-group">
          <label>Institution</label>
          <input
            v-model="institution_name"
            v-bind="institutionNameAttrs"
            type="text"
            placeholder="e.g. Royal Bank"
            :class="{ 'has-error': errors.institution_name }"
          />
          <span v-if="errors.institution_name" class="error-message">{{ errors.institution_name }}</span>
        </div>

        <div class="form-group">
          <label>Type</label>
          <select
            v-model="account_type"
            v-bind="accountTypeAttrs"
            :class="{ 'has-error': errors.account_type }"
          >
            <option v-for="type in accountTypes" :key="type" :value="type">
              {{ type === AccountType.CREDIT_CARD ? 'Credit Card' : type }}
            </option>
          </select>
          <span v-if="errors.account_type" class="error-message">{{ errors.account_type }}</span>
        </div>

        <div class="form-group">
          <label>Currency</label>
          <input
            v-model="currency"
            v-bind="currencyAttrs"
            type="text"
            placeholder="e.g. USD"
            :class="{ 'has-error': errors.currency }"
          />
          <span v-if="errors.currency" class="error-message">{{ errors.currency }}</span>
        </div>

        <div class="form-group">
          <label>Opening Balance</label>
          <input
            v-model.number="opening_balance"
            v-bind="openingBalanceAttrs"
            type="number"
            step="0.01"
            :class="{ 'has-error': errors.opening_balance }"
          />
          <span v-if="errors.opening_balance" class="error-message">{{ errors.opening_balance }}</span>
        </div>

        <div class="form-group">
          <label>Current Balance</label>
          <input
            v-model.number="current_balance"
            v-bind="currentBalanceAttrs"
            type="number"
            step="0.01"
            :class="{ 'has-error': errors.current_balance }"
          />
          <span v-if="errors.current_balance" class="error-message">{{ errors.current_balance }}</span>
        </div>
      </div>

      <div class="modal-footer">
        <button class="btn-secondary" @click="close">Cancel</button>
        <button class="btn-primary" @click="submit">Add Account</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { watch } from 'vue'
import { useForm } from 'vee-validate'
import { AccountType, type AddAccountRequest } from '@/models/accounts.d'

const props = defineProps<{
  visible: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
  (e: 'save', value: AddAccountRequest): void
}>()

const accountTypes = Object.values(AccountType)

const defaultValues: AddAccountRequest = {
  name: '',
  institution_name: '',
  account_type: AccountType.CHECKING,
  currency: 'USD',
  opening_balance: 0,
  current_balance: 0,
  is_closed: false,
}

const validationSchema = {
  name(value: unknown) {
    if (typeof value !== 'string' || !value.trim()) {
      return 'Name is required'
    }
    return true
  },
  institution_name(value: unknown) {
    if (typeof value !== 'string' || !value.trim()) {
      return 'Institution is required'
    }
    return true
  },
  account_type(value: unknown) {
    if (!value) {
      return 'Type is required'
    }
    return true
  },
  currency(value: unknown) {
    if (typeof value !== 'string' || !value.trim()) {
      return 'Currency is required'
    }
    return true
  },
  opening_balance(value: unknown) {
    if (value === '' || value === null || value === undefined) {
      return 'Opening balance is required'
    }
    if (typeof value !== 'number' || Number.isNaN(value)) {
      return 'Opening balance must be a number'
    }
    return true
  },
  current_balance(value: unknown) {
    if (value === '' || value === null || value === undefined) {
      return 'Current balance is required'
    }
    if (typeof value !== 'number' || Number.isNaN(value)) {
      return 'Current balance must be a number'
    }
    return true
  },
}

const { handleSubmit, errors, defineField, resetForm } = useForm<AddAccountRequest>({
  validationSchema,
  initialValues: { ...defaultValues },
})

const [name, nameAttrs] = defineField('name')
const [institution_name, institutionNameAttrs] = defineField('institution_name')
const [account_type, accountTypeAttrs] = defineField('account_type')
const [currency, currencyAttrs] = defineField('currency')
const [opening_balance, openingBalanceAttrs] = defineField('opening_balance')
const [current_balance, currentBalanceAttrs] = defineField('current_balance')

watch(
  () => props.visible,
  (visible) => {
    if (visible) {
      resetForm({ values: { ...defaultValues } })
    }
  },
)

const close = () => emit('close')

const submit = handleSubmit((values) => {
  emit('save', { ...values })
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

.form-group input,
.form-group select {
  padding: 8px 10px;
  border: 1px solid transparent;
  border-radius: 6px;
  font-size: 14px;
}

.form-group input.has-error,
.form-group select.has-error {
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
