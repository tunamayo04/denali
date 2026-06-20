import { defineStore, acceptHMRUpdate } from 'pinia'
import { ref } from 'vue'
import type {
  AddBudgetItemRequest,
  BudgetItem,
  EditBudgetItemRequest,
  GetBudgetItemsRequest,
} from '@/models/budget'
import BudgetService from '@/services/budgetService'

export const useBudgetStore = defineStore(
  "Budget",
  () => {
    // state
    const budgetItems = ref<BudgetItem[]>([]);
    const loading = ref<boolean>(false);
    const error = ref<string | null>(null);

    async function fetchBudgetItems(month: number, year: number): Promise<void> {
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

    async function addBudgetItem(item: BudgetItem): Promise<void> {
      console.log('addBudget')
      loading.value = true
      error.value = null
      try {
        const payload: AddBudgetItemRequest = {
          actual_amount: item.actual_amount,
          budget_amount: item.budget_amount,
          category: item.category,
          color: item.color,
          month: item.month.getMonth() + 1,
          year: item.month.getFullYear(),
        }
        const status = await BudgetService.addBudgetItem(payload)

        if (status === 204) {
          await fetchBudgetItems(payload.month, payload.year);
        } else {
          throw new Error(`Could not create budget item`)
        }
      } catch (e: any) {
        error.value = e.message || 'Could not delete BudgetItem'
      }
    }

    async function editBudgetItem(new_item: BudgetItem): Promise<void> {
      loading.value = true
      error.value = null
      try {
        const payload: EditBudgetItemRequest = {
          id: new_item.id,
          actual_amount: new_item.actual_amount,
          budget_amount: new_item.budget_amount,
          category: new_item.category,
          color: new_item.color,
          month: new_item.month.getMonth() + 1,
          year: new_item.month.getFullYear(),
        }
        const status = await BudgetService.editBudgetItem(payload)
        if (status === 204) {
          const index = budgetItems.value.findIndex((item) => item.id == new_item.id);
          if (index !== -1) {
            budgetItems.value[index] = new_item;
          }
        } else {
          throw new Error(`Could not edit budget item`)
        }
      } catch (e: any) {
        console.error(e)
        error.value = e.message || 'Could not edit BudgetItem'
      }
    }

    async function deleteBudgetItem(id: number): Promise<void> {
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
      addBudgetItem,
      editBudgetItem,
      deleteBudgetItem,
    }
  }
)

if (import.meta.hot) {
  import.meta.hot.accept(acceptHMRUpdate(useBudgetStore, import.meta.hot))
}
