<template>
  <div class="accounts-container">
    <header class="page-header">
      <div class="header-left">
        <h2>Accounts</h2>
      </div>

      <div class="header-actions">
        <button class="btn-primary" @click="modalVisible = true">
          <span class="material-symbols-outlined">add</span>
          Add Account
        </button>
      </div>
    </header>
    <AccountCard
      v-for="accountGroup in accountGroups"
      class="account-card"
      :key="accountGroup.type"
      :accounts="accountGroup.accounts"
      :type="accountGroup.type"
    />

    <AccountModal :visible="modalVisible" @close="modalVisible = false" @save="onAdd" />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import { useAccountsStore } from '@/stores/accountsStore.ts'
import AccountCard from '@/components/accounts/AccountCard.vue'
import AccountModal from '@/components/accounts/AccountModal.vue'
import { type Account, type AddAccountRequest, AccountType } from '@/models/accounts.d'

const accountsStore = useAccountsStore()

const modalVisible = ref(false)

onMounted(async () => {
  await accountsStore.fetchAccounts()
})

const onAdd = (account: AddAccountRequest) => {
  console.log('Adding account:', account)
  accountsStore.addAccount(account)
}

const accountGroups = computed(() => {
  return accountsStore.accounts.reduce(
    (groups, account) => {
      const group = groups.find((group) => group.type === account.account_type)
      if (group) {
        group.accounts.push(account)
      } else {
        groups.push({ type: account.account_type, accounts: [account] })
      }
      return groups
    },
    [] as { type: AccountType; accounts: Account[] }[],
  )
})
</script>

<style scoped>
.accounts-container {
  max-width: 1100px;
  margin: 0 auto;
  padding-bottom: 60px;
}

.account-card {
  margin: 20px 0;
}

.btn-primary {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  background: var(--color-primary);
  color: white;
  border: none;
  padding: 8px 14px;
  border-radius: 8px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
}

.btn-primary .material-symbols-outlined {
  font-size: 18px;
}
</style>
