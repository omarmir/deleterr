<template>
  <div class="flex h-screen bg-gray-50 dark:bg-gray-900">
    <Sidebar></Sidebar>
    <div class="flex w-full flex-1 flex-col pb-2">
      <Header />
      <main class="h-full overflow-y-auto">
        <div class="container mx-auto grid px-6">
          <router-view></router-view>
        </div>
      </main>
    </div>
  </div>
</template>
<script setup lang="ts">
import Sidebar from '~/components/Sidebar.vue'
import Header from '~/components/Header.vue'
</script>
