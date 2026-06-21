<template>
  <div class="accounts-container">
    <header class="page-header">
      <div class="header-left">
        <h2>Accounts</h2>
      </div>
    </header>
    <AccountCard
      v-for="accountGroup in accountGroups"
      class="account-card"
      :key="accountGroup.type"
      :accounts="accountGroup.accounts"
      :type="accountGroup.type"
    />
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useAccountsStore } from '@/stores/accountsStore.ts'
import AccountCard from '@/components/accounts/AccountCard.vue'
import { type Account, AccountType } from '@/models/accounts.d'

const accountsStore = useAccountsStore()

onMounted(async () => {
  await accountsStore.fetchAccounts()
})

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
</style>
