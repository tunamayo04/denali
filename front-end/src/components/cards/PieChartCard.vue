<template>
  <div class="visual-card">
    <div class="chart-wrapper">
      <div class="donut-chart" :style="{ background: dynamicConicGradient }">
        <div class="donut-hole"></div>
      </div>
    </div>

    <div class="chart-legend">
      <span
        v-for="row in props.budgetData"
        :key="row.category"
        class="legend-pill"
        :style="{ backgroundColor: row.color }"
      >
        {{ row.category }}
      </span>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { BudgetItem } from '@/dtos/budget.d'
import { computed } from 'vue'

const props = defineProps<{
  budgetData: BudgetItem[]
  totalActual: number
}>()

// Formats the pie segments dynamically based on whatever active data is picked
const dynamicConicGradient = computed(() => {
  let accumulatedPercentage = 0
  const segments = props.budgetData.map((row) => {
    const share = props.totalActual > 0 ? (row.actual / props.totalActual) * 100 : 0
    const start = accumulatedPercentage
    accumulatedPercentage += share
    return `${row.color} ${start.toFixed(1)}% ${accumulatedPercentage.toFixed(1)}%`
  })
  return segments.length ? `conic-gradient(${segments.join(', ')})` : '#EAEAEA'
})
</script>

<style scoped>
.visual-card {
  background: #ffffff;
  border-radius: 16px;
  padding: 30px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.01);
  height: fit-content;
}

.chart-wrapper {
  margin-bottom: 30px;
}

.donut-chart {
  width: 180px;
  height: 180px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.3s ease;
}

.donut-hole {
  width: 120px;
  height: 120px;
  background-color: #ffffff;
  border-radius: 50%;
}

.chart-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  justify-content: center;
}

.legend-pill {
  font-size: 11px;
  font-weight: 600;
  padding: 6px 14px;
  border-radius: 20px;
  color: #ffffff;
}
</style>
