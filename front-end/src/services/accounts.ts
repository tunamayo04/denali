import { HttpClientService } from '@/services/httpClientService.ts'
import { Endpoints } from '@/shared/endpoints.ts'
import type { Account, AddAccountRequest } from '@/models/accounts'

export default class AccountsService {
  static readonly httpClient: HttpClientService = new HttpClientService()

  static async getAccounts(): Promise<Account[]> {
    try {
      const response = await this.httpClient.get<Account[]>(Endpoints.GET_ACCOUNTS)
      console.log(response.data[0]?.updated_at)

      return response.data.map(account => {
        return {
          ...account,
          opening_balance: Number(account.opening_balance),
          current_balance: Number(account.current_balance),
          created_at: new Date(account.created_at),
          updated_at: new Date(account.updated_at),
        }
      });
    } catch (error) {
      console.error(error)
      throw error
    }
  }

  static async addAccount(payload: AddAccountRequest): Promise<number> {
    try {
      const response = await this.httpClient.post(Endpoints.ADD_ACCOUNT, payload)

      return response.status
    } catch (error) {
      console.error(error)
      throw error
    }
  }
}

