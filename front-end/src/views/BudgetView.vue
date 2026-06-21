<template>
  <div class="budget-container">
    <header class="page-header">
      <div class="header-left">
        <h2>Budget</h2>
        <p class="subtitle">Track your targeted budget parameters against actual spending.</p>
      </div>

      <div class="header-actions">
        <MonthPicker v-model="currentMonth" />
      </div>
    </header>

    <section class="metrics-grid">
      <MetricCard title="Total Budgeted" :amount="totalBudgeted" />
      <MetricCard title="Actual spent" :amount="totalActual" />
      <MetricCard
        :title="totalActual - totalBudgeted > 0 ? 'Over budget by' : 'Under budget by'"
        :amount="Math.abs(totalActual - totalBudgeted)"
        :alert="totalActual - totalBudgeted > 0"
        :green="totalActual - totalBudgeted <= 0"
      />
    </section>

    <section class="budget-content">
      <PieChartCard
        :labels="budgetData.filter((item) => item.budget_amount > 0).map((item) => item.category)"
        :colors="budgetData.filter((item) => item.budget_amount > 0).map((item) => item.color)"
        :data="
          budgetData.filter((item) => item.budget_amount > 0).map((item) => item.budget_amount)
        "
      />
      <TableCard
        :budget-data="budgetData"
        :on-delete="onDelete"
        :on-add="onAdd"
        :on-edit="onEdit"
      />
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import MetricCard from '@/components/cards/MetricCard.vue'
import MonthPicker from '@/components/MonthPicker.vue'
import type { BudgetItem } from '@/models/budget.d'
import PieChartCard from '@/components/cards/PieChartCard.vue'
import TableCard from '@/components/cards/TableCard.vue'
import { useBudgetStore } from '@/stores/budgetStore'
import { useTransactionsStore } from '@/stores/transactionsStore.ts'
import type { GetTransactionsRequest } from '@/models/transactions'
import { Filter } from '@/models/shared.ts'

const budgetStore = useBudgetStore()
const transactionsStore = useTransactionsStore()

// Selected month, owned here and driven by the MonthPicker
const currentMonth = ref<Date>(new Date(2026, 5, 1))

const fetchMonthlyData = (date: Date) =>
  budgetStore.fetchBudgetItems(date.getMonth() + 1, date.getFullYear())

watch(currentMonth, (date) => fetchMonthlyData(date))

onMounted(async () => {
  await fetchMonthlyData(currentMonth.value)

  const payload: GetTransactionsRequest = {
    date: Filter.eq('2026-06-19'),
  }

  await transactionsStore.fetchTransactions(payload)
  console.log(transactionsStore.transactions);
})

// Budget Store Computed Values
const budgetData = computed<BudgetItem[]>(() => budgetStore.budgetItems ?? [])
const totalBudgeted = computed(() =>
  (budgetStore.budgetItems ?? []).reduce((sum, item) => sum + item.budget_amount, 0),
)
const totalActual = computed(() =>
  (budgetStore.budgetItems ?? []).reduce((sum, item) => sum + item.actual_amount, 0),
)

// Callback Props
const onDelete = (id: number) => {
  budgetStore.deleteBudgetItem(id)
}
const onAdd = (item: BudgetItem) => {
  item.month = currentMonth.value
  budgetStore.addBudgetItem(item)
}
const onEdit = (item: BudgetItem) => {
  budgetStore.editBudgetItem(item)
}
</script>

<style scoped>
.budget-container {
  max-width: 1100px;
  margin: 0 auto;
  padding-bottom: 60px;
}

.page-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
}

.page-header h2 {
  font-size: 24px;
  font-weight: 600;
  color: var(--color-text-main);
  margin: 0 0 4px 0;
}

.subtitle {
  font-size: 14px;
  color: var(--color-text-secondary);
  margin: 0;
}

.month-stepper {
  display: flex;
  align-items: center;
  gap: 4px;
  background: var(--color-background-soft);
  border: 1px solid rgba(255, 255, 255, 0.06);
  border-radius: 8px;
  padding: 4px;
}

.month-label {
  min-width: 130px;
  text-align: center;
  font-size: 14px;
  font-weight: 500;
  color: var(--color-text-main);
  user-select: none;
}

.step-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  border-radius: 6px;
  background: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  transition:
    background-color 0.15s,
    color 0.15s;
}

.step-btn:hover {
  background-color: rgba(255, 255, 255, 0.06);
  color: var(--color-primary);
}

.step-btn .material-symbols-outlined {
  font-size: 20px;
}

.metrics-grid {
  display: flex;
  gap: 20px;
  margin-bottom: 30px;
}

.budget-content {
  display: grid;
  grid-template-columns: 1fr 1.4fr;
  gap: 24px;
}
</style>
