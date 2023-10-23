<template>
    <aside class="z-20 hidden w-64 flex-shrink-0 overflow-y-auto bg-white dark:bg-gray-800 md:block">
        <div class="py-4 text-gray-500 dark:text-gray-400">
            <Logo />
            <ul class="mt-6">
                <li class="relative px-6 py-3">
                    <span class="absolute inset-y-0 left-0 w-1 rounded-br-lg rounded-tr-lg bg-purple-600"
                        aria-hidden="true"></span>
                    <a class="inline-flex w-full items-center text-sm font-semibold text-gray-800 transition-colors duration-150 hover:text-gray-800 dark:text-gray-100 dark:hover:text-gray-200"
                        href="index.html">
                        <svg class="h-5 w-5" aria-hidden="true" fill="none" stroke-linecap="round" stroke-linejoin="round"
                            stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                            <path
                                d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6">
                            </path>
                        </svg>
                        <span class="ml-4">Dashboard</span>
                    </a>
                </li>
            </ul>
            <ul>
                <li class="relative px-6 py-3">
                    <a class="inline-flex w-full items-center text-sm font-semibold transition-colors duration-150 hover:text-gray-800 dark:hover:text-gray-200"
                        href="/services">
                        <svg class="h-5 w-5" aria-hidden="true" fill="none" stroke-linecap="round" stroke-linejoin="round"
                            stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                            <path
                                d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-3 7h3m-3 4h3m-6-4h.01M9 16h.01">
                            </path>
                        </svg>
                        <span class="ml-4">Services</span>
                    </a>
                </li>
                <li class="relative px-6 py-3">
                    <a class="inline-flex w-full items-center text-sm font-semibold transition-colors duration-150 hover:text-gray-800 dark:hover:text-gray-200"
                        href="cards.html">
                        <svg class="h-5 w-5" aria-hidden="true" fill="none" stroke-linecap="round" stroke-linejoin="round"
                            stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                            <path
                                d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10">
                            </path>
                        </svg>
                        <span class="ml-4">Settings</span>
                    </a>
                </li>
                <li class="relative px-6 py-3">
                    <a class="inline-flex w-full items-center text-sm font-semibold transition-colors duration-150 hover:text-gray-800 dark:hover:text-gray-200"
                        href="charts.html">
                        <svg class="h-5 w-5" aria-hidden="true" fill="none" stroke-linecap="round" stroke-linejoin="round"
                            stroke-width="2" viewBox="0 0 24 24" stroke="currentColor">
                            <path d="M11 3.055A9.001 9.001 0 1020.945 13H11V3.055z"></path>
                            <path d="M20.488 9H15V3.512A9.025 9.025 0 0120.488 9z"></path>
                        </svg>
                        <span class="ml-4">Automation</span>
                    </a>
                </li>
            </ul>
        </div>
    </aside>
</template>
<script lang="ts" setup>
import Logo from '~/components/Logo.vue'
</script>
