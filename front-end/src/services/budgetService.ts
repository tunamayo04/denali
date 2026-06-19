import type { BudgetItem } from '@/dtos/budget'
import axios, { isCancel, AxiosError } from 'axios'
import { Endpoints } from '@/shared/constants.ts'

export default class BudgetService {
  static async getBudgetItems(): Promise<BudgetItem[]> {
    try {
      const response = await axios.get<BudgetItem[]>(Endpoints.GET_BUDGET_ITEMS)
      return response.data;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }
}
