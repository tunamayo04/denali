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
      } catch (e: any) {
        error.value = e.message || "Could not fetch BudgetRows";
      } finally {
        loading.value = false;
      }
    }

    async function deleteBudgetItem(id: number) {
      loading.value = true;
      error.value = null;
      try {
        const status = await BudgetService.deleteBudgetItem(id);
        if (status === 204) {
          budgetItems.value = budgetItems.value?.filter(item => item.id !== id);
        }
        else {
          throw new Error(`Could not delete BudgetItem with id ${id}`);
        }
      } catch (e: any) {
        error.value = e.message || "Could not delete BudgetItem";
      }
    }

    return {
      // state
      budgetItems,
      loading,
      error,

      // actions
      fetchBudgetItems,
      deleteBudgetItem,
    }
  }
)
