import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { BudgetRow } from '@/dtos/budget'
import BudgetService from '@/services/budgetService.ts'

export const useBudgetStore = defineStore(
  "Budget",
  () => {
    // state
    const rows = ref<BudgetRow[]>();
    const loading = ref<boolean>(false);
    const error = ref<string | null>(null);

    async function fetchBudgetRows(): Promise<BudgetRow[] | undefined> {
      loading.value = true;
      error.value = null;
      try {
        rows.value = await BudgetService.getBudgetRows();

        return rows.value;
      } catch (error: any) {
        error.value = error.message || "Could not fetch BudgetRows";
      } finally {
        loading.value = false;
      }
    }

    return {
      // state
      rows,
      loading,
      error,

      // actions
      fetchBudgetRows,
    }
  }
)
