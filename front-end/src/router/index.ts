import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'

const routes: Array<RouteRecordRaw> = [
  {
    path: '/',
    redirect: '/overview',
  },
  {
    path: '/overview',
    name: 'Overview',
    component: () => import('@/views/OverviewView.vue'),
  },
  {
    path: '/budget',
    name: 'Budget',
    component: () => import('@/views/BudgetView.vue'),
  },
  {
    path: '/transactions',
    name: 'Transactions',
    component: () => import('../views/TransactionsView.vue'),
  },
  // {
  //   path: '/profile',
  //   name: 'Profile',
  //   component: () => import('../views/Profile.vue'),
  // },
  // {
  //   path: '/help',
  //   name: 'Help',
  //   component: () => import('../views/Help.vue'),
  // },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
