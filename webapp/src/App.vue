<template>
  <div class="flex h-screen bg-gray-50 dark:bg-gray-900">
    <Sidebar></Sidebar>
    <div class="flex flex-col flex-1 w-full pb-2">
      <Header />
      <Dashboard />
    </div>
  </div>
</template>

<script lang='ts' setup>
import Sidebar from './components/Sidebar.vue'
import Header from './components/Header.vue'
import Dashboard from './views/Dashboard.vue'
</script>

<style scoped></style>
