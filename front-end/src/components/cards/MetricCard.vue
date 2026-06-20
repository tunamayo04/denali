<template>
  <div class="metric-card" :class="{ 'alert-card': alert, 'green-card': green }">
    <span class="card-label">{{ props.title }}</span>
    <h3 class="card-value">{{ formatCurrency(props.amount) }}</h3>
  </div>
</template>

<script setup lang="ts">
import '@/assets/base.css'
import { formatCurrency } from '@/shared/utils'

const props = defineProps({
  title: {
    type: String,
    required: true,
  },
  amount: {
    type: Number,
    required: true,
  },
  alert: {
    type: Boolean,
    default: false,
  },
  green: {
    type: Boolean,
    default: false,
  }
})
</script>

<style scoped>
.metric-card {
  background: var(--color-background-soft);
  border-radius: 16px;
  padding: 24px;
  flex: 1;
  border: 1px solid rgba(0, 0, 0, 0.02);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.01);
  transition: transform 0.2s;
}

.metric-card .card-label {
  font-size: 12px;
  color: var(--color-text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.5px;
  display: block;
  margin-bottom: 6px;
}

.metric-card .card-value {
  font-size: 28px;
  font-weight: 700;
  color: var(--color-text-main);
  margin: 0;
}

.highlight-card {
  background: linear-gradient(135deg, #1d1e2c, #040510);
}
.highlight-card .card-label {
  color: rgba(255, 255, 255, 0.6);
}
.highlight-card .card-value {
  color: #ffffff;
}

.alert-card {
  background-color: var(--c-dark-red);
  border: 1px solid var(--c-medium-red);
}
.alert-card .card-value {
  color: var(--c-bright-red);
}

.green-card {
  background-color: var(--c-dark-green);
  border: 1px solid var(--c-medium-green);
}
.green-card .card-value {
  color: var(--c-bright-green);
}

</style>
