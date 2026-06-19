import type { BudgetItem, GetBudgetItemsRequest } from '@/models/budget'
import { Endpoints } from '@/shared/endpoints.ts'
import { HttpClientService } from '@/services/httpClientService.ts'

export default class BudgetService {
  static readonly httpClient: HttpClientService = new HttpClientService();

  static async getBudgetItems(payload: GetBudgetItemsRequest): Promise<BudgetItem[]> {
    try {
      const response = await this.httpClient.get<BudgetItem[]>(
        Endpoints.GET_BUDGET_ITEMS,
        payload
      );

      return response.data.map((item) => ({
        ...item,
        budget_amount: Number(item.budget_amount),
        actual_amount: Number(item.actual_amount),
      }));
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  static async deleteBudgetItem(id: number): Promise<number> {
    try {
      const response = await this.httpClient.delete(
        Endpoints.DELETE_BUDGET_ITEM,
        { id }
      );

      return response.status;
      } catch (error) {
      console.error(error);
      throw error;
    }
  }
}
