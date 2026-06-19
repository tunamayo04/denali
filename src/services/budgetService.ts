import type { BudgetRow } from '@/dtos/budget'
import axios, { isCancel, AxiosError } from 'axios'
import { Endpoints } from '@/shared/constants.ts'

export default class BudgetService {
  static async getBudgetRows(): Promise<BudgetRow[]> {
    try {
      const response = await axios.get<BudgetRow[]>(Endpoints.GET_BUDGET_ROWS)
      return response.data;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }
}
