<template>
  <div class="account-card">
    <div class="account-group-header">
      <div class="left">
        <span class="account-label">{{ accountType === AccountType.CREDITCARD ? 'Credit Card' : accountType }}</span>
        <span class="overtime-change" :class="{ positive: isPositive, negative: !isPositive }">
          <span v-if="isPositive" class="material-symbols-outlined"> north </span>
          <span v-if="!isPositive" class="material-symbols-outlined"> south </span>
          {{ formatCurrency(accountsTotal - accountsOpeningTotal) }}
          ({{ Math.abs(growth).toFixed(2) }}%)
        </span>
      </div>
      <span class="account-total">{{ formatCurrency(accountsTotal) }}</span>
    </div>
    <div class="accounts">
      <div class="account-row" v-for="account in accounts" :key="account.id">
        <div class="left">
          <div class="account-name">{{ account.name }}</div>
          <div class="subtitle">{{ account.institution_name }}</div>
        </div>
        <div class="right">
          <div class="account-balance">{{ formatCurrency(account.current_balance) }}</div>
          <div class="subtitle">Updated {{ formatDateDifference(account.updated_at) }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { type Account, AccountType } from '@/models/accounts.d'
import { formatCurrency, formatDateDifference } from '@/shared/utils.ts'
import { computed } from 'vue'

const props = defineProps<{
  key: AccountType
  accounts: Account[]
  type: AccountType
}>()

const accountType = props.type
const accountsTotal = props.accounts.reduce((total, account) => total + account.current_balance, 0)
const accountsOpeningTotal = props.accounts.reduce(
  (total, account) => total + account.opening_balance,
  0,
)
const isPositive =
  accountType == AccountType.CREDIT_CARD || accountType == AccountType.LOAN
    ? accountsTotal <= accountsOpeningTotal
    : accountsTotal > accountsOpeningTotal
const growth = computed(() => accountsTotal / accountsOpeningTotal - 1)
</script>

<style scoped>
.account-card {
  background: var(--color-background-soft);
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.account-group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  background: var(--color-background);
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.account-total {
  font-variant-numeric: tabular-nums;
}

.overtime-change {
  display: inline-flex;
  align-items: center;
}

.overtime-change .material-symbols-outlined {
  font-size: 1em;
  margin-right: 2px;
}

.account-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 14px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  transition: background-color 0.15s;
}

.account-row:hover {
  background-color: rgba(255, 255, 255, 0.03);
}

.account-name,
.account-balance {
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-main);
  margin-bottom: 7px;
}

.account-balance {
  font-weight: 600;
  font-variant-numeric: tabular-nums;
}

.left {
  text-align: left;
}

.left span {
  margin-right: 10px;
}

.right {
  text-align: right;
}
</style>
