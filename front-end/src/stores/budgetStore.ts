import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { BudgetItem, GetBudgetItemsRequest } from '@/models/budget'
import BudgetService from '@/services/budgetService.ts'

export const useBudgetStore = defineStore(
  "Budget",
  () => {
    // state
    const budgetItems = ref<BudgetItem[]>();
    const loading = ref<boolean>(false);
    const error = ref<string | null>(null);

    async function fetchBudgetItems(month: number, year: number) {
      loading.value = true;
      error.value = null;
      try {
        const requestPayload: GetBudgetItemsRequest = {
          month,
          year
        }

        budgetItems.value = await BudgetService.getBudgetItems(requestPayload);
      } catch (error: any) {
        error.value = error.message || "Could not fetch BudgetRows";
      } finally {
        loading.value = false;
      }
    }

    return {
      // state
      rows: budgetItems,
      loading,
      error,

      // actions
      fetchBudgetRows: fetchBudgetItems,
    }
  }
)
