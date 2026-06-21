<template>
  <div class="card transactions-summary">
    <div>
      <h3>Summary</h3>
      <section>
        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.totalTransactions') }}</div>
          <div class="summary-value">{{ totalTransactions }}</div>
        </div>

        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.largestTransaction') }}</div>
          <div class="summary-value" :class="largestTransaction > 0 ? 'positive' : 'negative'">
            {{ formatCurrency(largestTransaction, true) }}
          </div>
        </div>
        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.largestExpense') }}</div>
          <div class="summary-value negative">{{ formatCurrency(largestExpense, true) }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.averageTransaction') }}</div>
          <div class="summary-value">{{ formatCurrency(averageTransaction, true) }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.totalIncome') }}</div>
          <div class="summary-value positive">{{ formatCurrency(totalIncome, true) }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.totalExpenses') }}</div>
          <div class="summary-value negative">{{ formatCurrency(totalExpenses, true) }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.firstTransaction') }}</div>
          <div class="summary-value">{{ firstTransactionDate }}</div>
        </div>
        <div class="summary-item">
          <div class="summary-label subtitle">{{ t('transactions.summary.lastTransaction') }}</div>
          <div class="summary-value">{{ lastTransactionDate }}</div>
        </div>
      </section>
    </div>
    <div class="download-csv">Download CSV</div>
  </div>
</template>

<script setup lang="ts">
import type { Transaction } from '@/models/transactions'
import { computed } from 'vue'
import { formatCurrency } from '@/shared/utils.ts'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()

const props = defineProps<{
  transactions: Transaction[]
}>()

const totalTransactions = computed(() => props.transactions.length)
const largestTransaction = computed(() =>
  props.transactions.reduce(
    (largest, t) => (Math.abs(t.amount) > Math.abs(largest) ? t.amount : largest),
    0,
  ),
)
const largestExpense = computed(() =>
  Math.min(...props.transactions.filter((t) => t.amount < 0).map((t) => t.amount)),
)
const averageTransaction = computed(() =>
  Math.round(props.transactions.reduce((acc, t) => acc + t.amount, 0) / props.transactions.length),
)
const totalIncome = computed(() =>
  props.transactions.filter((t) => t.amount > 0).reduce((acc, t) => acc + t.amount, 0),
)
const totalExpenses = computed(() =>
  props.transactions
    .filter((t) => t.amount < 0)
    .reduce((acc, t) => acc + (t.amount < 0 ? t.amount : 0), 0),
)
const firstTransactionDate = computed(() => props.transactions[0]?.date)
const lastTransactionDate = computed(() => props.transactions[props.transactions.length - 1]?.date)
</script>

<style scoped>
.transactions-summary {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}

.transactions-summary h3 {
  margin: 0 0 40px 0;
}

.summary-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin: 20px 0;
}

.download-csv {
  margin-top: 20px;
  text-align: center;
  font-size: 14px;
  color: #5ec5f1;
  font-weight: 500;
}
</style>
