/// --- Requests ---
export interface GetBudgetItemsRequest {
  year: number;
  month: number;
}

export interface AddBudgetItemRequest {
  year: number;
  month: number;
  category: string;
  budget_amount: number;
  actual_amount: number;
  color: string;
}

export interface EditBudgetItemRequest {
  id: number,
  year: number
  month: number
  category: string
  budget_amount: number
  actual_amount: number
  color: string
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
