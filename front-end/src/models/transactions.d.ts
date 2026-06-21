import type { IsoDate } from '@/models/shared.ts'

export interface Transaction {
  id: number,
  vendor_id: number,
  amount: number,
  date: Date,
}

export interface GetTransactionsRequest {
  vendor_id?: number,
  amount?: Filter<number>,
  date?: Filter<IsoDate>,
}
