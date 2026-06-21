import { defineStore } from 'pinia'
import type { GetTransactionsRequest, Transaction } from '@/models/transactions'
import TransactionsService from '@/services/transactionsService.ts'
import { ref } from 'vue'

export const useTransactionsStore = defineStore(
  'transactions', () => {
    // state
    const transactions = ref<Transaction[]>([])
    const loading = ref<boolean>(false)
    const error = ref<string | null>(null)

    // actions
    async function fetchTransactions(payload: GetTransactionsRequest): Promise<void> {
      loading.value = true
      error.value = null
      try {
        transactions.value = await TransactionsService.getTransactions(payload)
      } catch (e: any) {
        error.value = e.message || 'Could not fetch transactions'
      } finally {
        loading.value = false
      }
    }

    return {
      transactions,

      fetchTransactions
    }
  }
)
