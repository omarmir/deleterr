<template>
  <div class="flex h-screen bg-gray-50 dark:bg-gray-900">
    <Sidebar></Sidebar>
    <div class="flex w-full flex-1 flex-col pb-2">
      <Header>
        <input
          class="form-input h-8 w-full rounded-md border-0 bg-gray-100 pl-8 pr-2 text-sm text-gray-700 placeholder-gray-600 focus:border-purple-300 focus:bg-white focus:placeholder-gray-500 focus:shadow-outline-purple focus:outline-none focus:ring-0 dark:bg-gray-700 dark:text-gray-200 dark:placeholder-gray-500 dark:focus:placeholder-gray-600 dark:focus:shadow-outline-gray"
          type="text"
          placeholder="Search for requests"
          aria-label="Search"
          v-model="search" />
      </Header>
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
import { provide, Ref, ref } from 'vue'
const search: Ref<string> = ref('')
provide('search', search)
</script>
