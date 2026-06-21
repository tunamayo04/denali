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

      return response.data
    } catch (error) {
      console.error(error)
      throw error
    }
  }
}

