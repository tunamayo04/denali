<template>
  <div class="transactions-container">
    <header class="page-header">
      <div class="header-left">
        <h2>{{ t('transactions.title') }}</h2>
      </div>

      <div class="header-actions">
        <MonthPicker v-model="currentMonth" />
      </div>
    </header>

    <section class="transactions-content">
      <TransactionsTable class="transactions-table" :transactions="transactionsStore.transactions" />
      <TransactionsSummary class="transactions-summary" :transactions="transactionsStore.transactions" />
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { useTransactionsStore } from '@/stores/transactionsStore.ts'
import type { GetTransactionsRequest } from '@/models/transactions'
import { Filter } from '@/models/shared.ts'
import MonthPicker from '@/components/MonthPicker.vue'
import TransactionsTable from '@/components/transactions/TransactionsTable.vue'
import TransactionsSummary from '@/components/transactions/TransactionsSummary.vue'
import { useI18n } from 'vue-i18n'
const { t } = useI18n();

const transactionsStore = useTransactionsStore()
const currentMonth = ref<Date>(new Date(2026, 5, 1))

const fetchTransactionsForMonth = async (date: Date) => {
  const from = new Date(date.getFullYear(), date.getMonth(), 1)
  const to = new Date(date.getFullYear(), (date.getMonth() + 1) % 12, 0)
  const payload: GetTransactionsRequest = {
    date: Filter.rangeInclusive(from.toISOString().split('T')[0], to.toISOString().split('T')[0]),
  }

  await transactionsStore.fetchTransactions(payload)
}

onMounted(async () => {
  await fetchTransactionsForMonth(currentMonth.value)
})

watch(currentMonth, (date) => {
  fetchTransactionsForMonth(date)
})
</script>

<style>
.transactions-container {
  max-width: 1100px;
  margin: 0 auto;
  padding-bottom: 60px;
}

.transactions-content {
  display: flex;
  flex-direction: row;
}

.transactions-table {
  flex: 4;
  margin-right: 2%;
}

.transactions-summary {
  flex: 2;
}
</style>
