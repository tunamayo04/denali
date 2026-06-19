/// --- Requests ---
export interface GetBudgetItemsRequest {
  year: number;
  month: number;
}

/// --- Responses ---
export interface BudgetItem {
  id: number;
  month: Date;
  category: string
  budget_amount: number
  actual_amount: number
  color: string
}
