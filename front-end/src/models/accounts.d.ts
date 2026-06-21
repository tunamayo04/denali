export interface AddAccountRequest {
  name: string;
  institution_name: string;
  account_type: AccountType;
  currency: string;
  opening_balance: number;
  current_balance: number;
  is_closed: boolean;
}

export interface Account {
  id: string;
  name: string;
  institution_name: string;
  account_type: AccountType;
  currency: string;
  opening_balance: number;
  current_balance: number;
  is_closed: boolean;
  created_at: Date;
  updated_at: Date;
}

export enum AccountType {
  CHECKING = 'Checking',
  SAVINGS = 'Saving',
  CREDIT_CARD = 'CreditCard',
  INVESTMENT = 'Investment',
  CASH = 'Cash',
  LOAN = 'Loan',
}
