<template>
  <aside class="navbar">
    <div class="nav-header">
      <span class="material-symbols-outlined logo-icon">account_balance_wallet</span>
      <h1 class="brand-name">Denali</h1>
    </div>

    <nav class="nav-body">
      <ul class="nav-list">
        <li v-for="item in navItems" :key="item.path">
          <router-link :to="item.path" class="nav-item">
            <span class="material-symbols-outlined nav-icon">{{ item.icon }}</span>
            <span class="nav-label">{{ item.label }}</span>
          </router-link>
        </li>
      </ul>
    </nav>

    <div class="nav-footer">
      <router-link to="/help" class="nav-item footer-item">
        <span class="material-symbols-outlined nav-icon">help</span>
        <span class="nav-label">Help</span>
      </router-link>
      <router-link to="/profile" class="nav-item footer-item">
        <span class="material-symbols-outlined nav-icon">person</span>
        <span class="nav-label">Profile</span>
      </router-link>
    </div>
  </aside>
</template>

<script setup lang="ts">
import '@/assets/base.css'

interface NavItem {
  label: string
  path: string
  icon: string
}

const navItems: NavItem[] = [
  { label: 'Overview', path: '/overview', icon: 'insert_chart' },
  { label: 'Budget', path: '/budget', icon: 'savings' },
  { label: 'Transactions', path: '/transactions', icon: 'sync_alt' },
]
</script>

<style scoped>
.navbar {
  --nav-width: 220px;
  --header-padding: 30px 24px;
  --body-padding: 10px 14px;
  --item-padding: 12px 16px;
  --item-border-radius: 8px;

  display: flex;
  flex-direction: column;
  width: var(--nav-width);
  min-height: 100vh;
  background-color: var(--color-background-soft);
  box-shadow: 1px 0 5px rgba(0, 0, 0, 0.02);
  box-sizing: border-box;
}

.nav-header {
  padding: var(--header-padding);
  display: flex;
  align-items: center;
  gap: 12px;
}

.logo-icon {
  font-size: 24px;
  color: var(--c-whisteria);
}

.brand-name {
  font-size: 20px;
  font-weight: 700;
  color: var(--color-text-main);
  margin: 0;
}

.nav-body {
  flex-grow: 1;
  padding: var(--body-padding);
}

.nav-list {
  list-style: none;
  padding: 0;
  margin: 0;
}

.nav-item {
  display: flex;
  align-items: center;
  padding: var(--item-padding);
  text-decoration: none;
  border-radius: var(--item-border-radius);
  color: var(--color-text-secondary);
  font-size: 14px;
  font-weight: 500;
  transition:
    background-color 0.2s,
    color 0.2s;
  margin-bottom: 6px;
}

.nav-item:not(.router-link-active):hover {
  background-color: rgba(0, 0, 0, 0.03);
  color: var(--color-text-main);
}

/* Vue Router Active State Class */
.nav-item.router-link-active {
  background-color: var(--color-background);
  color: var(--color-text-main);
}

/* Font Icon Specific Styling */
.nav-icon {
  font-size: 20px;
  margin-right: 12px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
}

.nav-footer {
  padding: var(--body-padding);
  display: flex;
  flex-direction: column;
  margin-bottom: 16px;
}
</style>
