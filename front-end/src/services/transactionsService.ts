import type { GetTransactionsRequest, Transaction } from '@/models/transactions'
import { HttpClientService } from '@/services/httpClientService.ts'
import { Endpoints } from '@/shared/endpoints.ts'

export default class TransactionsService {
  static readonly httpClient: HttpClientService = new HttpClientService()

  static async getTransactions(payload: GetTransactionsRequest): Promise<Transaction[]> {
    try {
      const response = await this.httpClient.get<Transaction[]>(
        Endpoints.GET_TRANSACTIONS,
        payload
      )

      // The API returns amount as a string; coerce to a number so the
      // Transaction.amount: number contract holds for all consumers.
      return response.data.map((transaction) => ({
        ...transaction,
        amount: Number(transaction.amount),
      }))
    } catch (error) {
      console.error(error)
      throw error
    }
  }
}

