<template>
  <div class="month-stepper">
    <button class="step-btn" @click="changeMonth(-1)" aria-label="Previous month">
      <span class="material-symbols-outlined">chevron_left</span>
    </button>
    <span class="month-label">{{ selectedMonth }}</span>
    <button class="step-btn" @click="changeMonth(1)" aria-label="Next month">
      <span class="material-symbols-outlined">chevron_right</span>
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const model = defineModel<Date>({ required: true })

const monthNames = [
  'January', 'February', 'March', 'April', 'May', 'June',
  'July', 'August', 'September', 'October', 'November', 'December',
]

const selectedMonth = computed<string>(
  () => `${monthNames[model.value.getMonth()]} ${model.value.getFullYear()}`,
)

const changeMonth = (delta: number) => {
  const next = new Date(model.value)
  next.setMonth(next.getMonth() + delta)
  model.value = next
}
</script>

<style scoped>
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
  transition: background-color 0.15s, color 0.15s;
}

.step-btn:hover {
  background-color: rgba(255, 255, 255, 0.06);
  color: var(--color-primary);
}

.step-btn .material-symbols-outlined {
  font-size: 20px;
}
</style>
