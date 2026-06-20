<template>
  <div class="visual-card">
    <div class="chart-wrapper">
      <Doughnut :options="chartOptions" :data="chartData" />
    </div>

    <div class="chart-legend">
      <span
        v-for="row in pills"
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
import { Doughnut } from 'vue-chartjs'
import { Chart as ChartJS, Tooltip, ArcElement, type ChartData } from 'chart.js'
import { computed } from 'vue'
import { formatCurrency } from '@/shared/utils.ts'

const props = defineProps<{
  labels: string[]
  colors: string[]
  data: number[]
}>()

ChartJS.register(ArcElement, Tooltip)
const chartData = computed<ChartData<'doughnut', number[], string>>(() => ({
  labels: props.labels,
  datasets: [
    {
      backgroundColor: props.colors,
      data: props.data,
    },
  ],
}))
const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  borderWidth: 1,
  plugins: {
    tooltip: {
      callbacks: {
        label(context) {
          return `${context.label}: ${formatCurrency(context.parsed)}`
        },
      },
    },
  },
}

const pills = computed(() =>
  props.colors.map((color, i) => ({
    color,
    category: props.labels?.[i],
  })),
)
</script>

<style scoped>
.visual-card {
  background: var(--color-background-soft);
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
  color: white;
}
</style>
