<template>
  <div class="metric-card" :class="{ 'alert-card': alert }">
    <span class="card-label">{{ props.title }}</span>
    <h3 class="card-value">{{ formatCurrency(props.amount) }}</h3>
  </div>
</template>

<script setup lang="ts">
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
})
</script>

<style scoped>
.metric-card {
  background: #ffffff;
  border-radius: 16px;
  padding: 24px;
  flex: 1;
  border: 1px solid rgba(0, 0, 0, 0.02);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.01);
  transition: transform 0.2s;
}

.metric-card .card-label {
  font-size: 12px;
  color: #8e8e93;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  display: block;
  margin-bottom: 6px;
}

.metric-card .card-value {
  font-size: 28px;
  font-weight: 700;
  color: #1c1f21;
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
  background-color: #fff5f5;
  border: 1px solid #ffe2e2;
}
.alert-card .card-value {
  color: #e53e3e;
}
</style>
