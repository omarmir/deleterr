<template>
  <div class="flex h-screen bg-gray-50 dark:bg-gray-900">
    <Sidebar></Sidebar>
    <div class="flex flex-col flex-1 w-full pb-2">
      <Header />
      <main class="h-full overflow-y-auto">
        <div class="container px-6 mx-auto grid">
          <router-view />
        </div>
      </main>
    </div>
  </div>
</template>

<script lang='ts' setup>
import Sidebar from './components/Sidebar.vue'
import Header from './components/Header.vue'
</script>

<style scoped></style>
