<template>
  <div class="budget-container">
    <header class="page-header">
      <div class="header-left">
        <h2>Budget</h2>
        <p class="subtitle">Track your targeted budget parameters against actual spending.</p>
      </div>

      <div class="header-actions">
        <div class="dropdown-wrapper" v-click-outside="closeDropdown">
          <button class="date-badge" @click="toggleDropdown">
            {{ selectedMonth }}
            <span class="material-symbols-outlined">expand_more</span>
          </button>

          <ul v-if="isDropdownOpen" class="dropdown-menu">
            <li
              v-for="month in availableMonths"
              :key="month"
              :class="{ active: month === selectedMonth }"
              @click="selectMonth(month)"
            >
              {{ month }}
            </li>
          </ul>
        </div>
      </div>
    </header>

    <section class="metrics-grid">
      <MetricCard title="Total Budgeted" :amount="2000" />
      <MetricCard title="Actual spent" :amount="3000" />
      <MetricCard title="Over budget by" :amount="1000" alert />
    </section>

    <section class="budget-content">
      <PieChartCard :budget-data="budgetData" :total-actual="totalActual" />
      <TableCard :budget-data="budgetData" />
    </section>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import MetricCard from '@/components/cards/MetricCard.vue'
import type { BudgetRow } from '@/dtos/budget.ts'
import PieChartCard from '@/components/cards/PieChartCard.vue'
import TableCard from '@/components/cards/TableCard.vue'

// Mock Database structured by Month to show dynamic switching working
const monthlyDataDataSets: Record<string, BudgetRow[]> = {
  'October 2026': [
    { category: 'Rent', budget: 4000, actual: 4100, color: '#0084FF' },
    { category: 'Utilities', budget: 1000, actual: 1000, color: '#A052FF' },
    { category: 'Groceries', budget: 1000, actual: 1000, color: '#7033FF' },
    { category: 'Insurance', budget: 900, actual: 1000, color: '#FFB800' },
    { category: 'Entertainment', budget: 1000, actual: 1000, color: '#00E5A3' },
    { category: 'Health and beauty', budget: 800, actual: 1000, color: '#0057FF' },
    { category: 'Pet', budget: 0, actual: 100, color: '#FF7EA5' },
  ],
  'November 2026': [
    { category: 'Rent', budget: 4000, actual: 4000, color: '#0084FF' },
    { category: 'Utilities', budget: 1100, actual: 1250, color: '#A052FF' },
    { category: 'Groceries', budget: 1000, actual: 920, color: '#7033FF' },
    { category: 'Insurance', budget: 900, actual: 900, color: '#FFB800' },
    { category: 'Entertainment', budget: 800, actual: 1100, color: '#00E5A3' },
  ],
  'December 2026': [
    { category: 'Rent', budget: 4000, actual: 4100, color: '#0084FF' },
    { category: 'Utilities', budget: 1000, actual: 950, color: '#A052FF' },
    { category: 'Groceries', budget: 1500, actual: 1750, color: '#7033FF' },
    { category: 'Gifts & Holiday', budget: 2000, actual: 2400, color: '#FF7EA5' },
  ],
}

// Dropdown State Elements
const selectedMonth = ref<string>('October 2026')
const isDropdownOpen = ref<boolean>(false)
const availableMonths = Object.keys(monthlyDataDataSets)

const budgetData = computed<BudgetRow[]>(() => {
  return monthlyDataDataSets[selectedMonth.value] || []
})
const totalActual = computed(() => budgetData.value.reduce((acc, row) => acc + row.actual, 0))

// Dropdown Logic Toggles
const toggleDropdown = () => (isDropdownOpen.value = !isDropdownOpen.value)
const closeDropdown = () => (isDropdownOpen.value = false)
const selectMonth = (month: string) => {
  selectedMonth.value = month
  isDropdownOpen.value = false
}

const vClickOutside = {
  mounted(el: any, binding: any) {
    el.clickOutsideEvent = (event: Event) => {
      if (!(el === event.target || el.contains(event.target))) {
        binding.value()
      }
    }
    document.body.addEventListener('click', el.clickOutsideEvent)
  },
  unmounted(el: any) {
    document.body.removeEventListener('click', el.clickOutsideEvent)
  },
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
  color: #1c1f21;
  margin: 0 0 4px 0;
}

.subtitle {
  font-size: 14px;
  color: #757575;
  margin: 0;
}

.dropdown-wrapper {
  position: relative;
  display: inline-block;
}

.date-badge {
  background: #ffffff;
  padding: 10px 18px;
  border-radius: 8px;
  border: 1px solid #eaeaea;
  font-size: 14px;
  font-weight: 500;
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
  color: #1c1f21;
  transition: background-color 0.15s;
}

.date-badge:hover {
  background-color: #f8fafc;
}

.date-badge span {
  font-size: 18px;
  color: #666;
}

.dropdown-menu {
  position: absolute;
  top: calc(100% + 6px);
  right: 0;
  background: #ffffff;
  border: 1px solid #eaeaea;
  border-radius: 8px;
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.08);
  padding: 6px 0;
  list-style: none;
  min-width: 160px;
  z-index: 100;
  margin: 0;
}

.dropdown-menu li {
  padding: 10px 16px;
  font-size: 14px;
  color: #333;
  cursor: pointer;
  transition: background 0.15s;
}

.dropdown-menu li:hover {
  background: #f1f5f9;
}

.dropdown-menu li.active {
  background: #eceff3;
  font-weight: 600;
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
