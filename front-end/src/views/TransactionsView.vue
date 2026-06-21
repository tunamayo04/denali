<template>
  <div class="transactions-container">
    <header class="page-header">
      <div class="header-left">
        <h2>Transactions</h2>
        <p class="subtitle">Review and audit inboud and outbound transactions.</p>
      </div>

      <div class="header-actions">
        <MonthPicker v-model="currentMonth" />
      </div>
    </header>

    <section class="transactions-content">
      <TransactionsTable :transactions="transactionsStore.transactions" />
    </section>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useTransactionsStore } from '@/stores/transactionsStore.ts'
import type { GetTransactionsRequest } from '@/models/transactions'
import { Filter } from '@/models/shared.ts'
import MonthPicker from '@/components/MonthPicker.vue'
import TransactionsTable from '@/components/transactions/TransactionsTable.vue'

const transactionsStore = useTransactionsStore()
const currentMonth = ref<Date>(new Date(2026, 5, 1))

onMounted(async () => {
  const payload: GetTransactionsRequest = {
    date: Filter.lt('2026-06-30'),
  }

  await transactionsStore.fetchTransactions(payload)
})
</script>

<style>
.transactions-container {
  max-width: 1100px;
  margin: 0 auto;
  padding-bottom: 60px;
}
</style>
