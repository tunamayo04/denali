import { defineStore } from 'pinia'
import type { GetTransactionsRequest, Transaction } from '@/models/transactions'
import TransactionsService from '@/services/transactionsService.ts'
import { ref } from 'vue'
import type { Account, AddAccountRequest } from '@/models/accounts'
import AccountsService from '@/services/accounts.ts'

export const useAccountsStore = defineStore('accounts', () => {
  // state
  const accounts = ref<Account[]>([])
  const loading = ref<boolean>(false)
  const error = ref<string | null>(null)

  // actions
  async function fetchAccounts(): Promise<void> {
    loading.value = true
    error.value = null
    try {
      accounts.value = await AccountsService.getAccounts()
    } catch (e: any) {
      error.value = e.message || 'Could not fetch accounts'
    } finally {
      loading.value = false
    }
  }

  async function addAccount(payload: AddAccountRequest): Promise<void> {
    loading.value = true
    error.value = null
    try {
      const status = await AccountsService.addAccount(payload)

      if (status === 204 || status === 200 || status === 201) {
        await fetchAccounts()
      } else {
        throw new Error('Could not create account')
      }
    } catch (e: any) {
      error.value = e.message || 'Could not create account'
    } finally {
      loading.value = false
    }
  }

  return {
    accounts,
    loading,
    error,

    fetchAccounts,
    addAccount,
  }
})
