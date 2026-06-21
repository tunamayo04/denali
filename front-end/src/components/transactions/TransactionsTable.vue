<template>
  <div class="transactions-table">
    <template v-for="group in groupedTransactions" :key="group.key">
      <div class="date-group-header">
        <span class="date-label">{{ group.label }}</span>
        <span
          class="date-total"
          :class="{ positive: group.total > 0 }"
        >{{ formatCurrency(group.total, true) }}</span>
      </div>

      <div
        v-for="transaction in group.transactions"
        :key="transaction.id"
        class="transaction-row"
      >
        <span class="row-icon material-symbols-outlined">receipt_long</span>

        <div class="row-vendor">{{ transaction.vendor_name }}</div>

        <div class="row-account">{{ transaction.account_name }}</div>

        <span
          class="row-amount"
          :class="{ positive: transaction.amount > 0 }"
        >{{ formatCurrency(transaction.amount, true) }}</span>

        <span class="row-chevron material-symbols-outlined">chevron_right</span>
      </div>
    </template>

    <p v-if="!transactions.length" class="empty-state">No transactions to show.</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import type { Transaction } from '@/models/transactions'
import { formatCurrency, parseLocalDate } from '@/shared/utils'

const props = defineProps<{
  transactions: Transaction[],
}>()

interface TransactionGroup {
  key: string,
  label: string,
  total: number,
  transactions: Transaction[],
}


const groupedTransactions = computed<TransactionGroup[]>(() => {
  const groups = new Map<string, TransactionGroup>()

  for (const transaction of props.transactions) {
    const date = parseLocalDate(transaction.date)
    const key = `${date.getFullYear()}-${String(date.getMonth() + 1).padStart(2, '0')}-${String(date.getDate()).padStart(2, '0')}`

    let group = groups.get(key)
    if (!group) {
      group = {
        key,
        label: date.toLocaleDateString('en-US', {
          month: 'long',
          day: 'numeric',
          year: 'numeric',
        }),
        total: 0,
        transactions: [],
      }
      groups.set(key, group)
    }

    group.total += transaction.amount
    group.transactions.push(transaction)
  }

  // Most recent date first
  return [...groups.values()].sort((a, b) => b.key.localeCompare(a.key))
})
</script>

<style scoped>
.transactions-table {
  background: var(--color-background-soft);
  border-radius: 16px;
  overflow: hidden;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
}

.date-group-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 10px 20px;
  background: var(--color-background);
  font-size: 13px;
  font-weight: 600;
  color: var(--color-text-secondary);
}

.date-total {
  font-variant-numeric: tabular-nums;
}

.transaction-row {
  display: flex;
  align-items: center;
  gap: 14px;
  padding: 14px 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  transition: background-color 0.15s;
}

.transaction-row:hover {
  background-color: rgba(255, 255, 255, 0.03);
}

.row-icon {
  font-size: 20px;
  color: var(--color-text-secondary);
  flex-shrink: 0;
}

.row-vendor {
  flex: 0 0 200px;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-main);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-account {
  flex: 0 0 200px;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-main);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.row-amount {
  margin-left: auto;
  font-size: 14px;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  color: var(--color-text-main);
}

.row-chevron {
  font-size: 18px;
  color: var(--color-text-secondary);
  flex-shrink: 0;
}

.empty-state {
  padding: 40px 20px;
  text-align: center;
  color: var(--color-text-secondary);
  font-size: 14px;
}
</style>
