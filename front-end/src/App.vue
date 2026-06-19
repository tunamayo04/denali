<template>
  <div class="app-layout">
    <Navbar class="app-sidebar" />

    <main class="main-content">
      <AppConfirmDialog />
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import AppConfirmDialog from './components/AppConfirmDialog.vue'
import Navbar from './components/NavBar.vue'
import '@/assets/base.css'
</script>

<style>
html,
body,
#app {
  margin: 0;
  padding: 0;
  width: 100%;
  height: 100vh;
  overflow: hidden;
  background-color: var(--color-background);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.app-layout {
  display: flex;
  width: 100%;
  height: 100%;
}

.app-sidebar {
  flex-shrink: 0;
  height: 100vh;
}

.main-content {
  flex-grow: 1;
  padding: 40px;
  box-sizing: border-box;

  height: 100vh;
  overflow-y: auto;
}
</style>
