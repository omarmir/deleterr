<template>
    <table class="w-full whitespace-no-wrap">
        <thead>
            <tr
                class="text-xs font-semibold tracking-wide text-left text-gray-500 uppercase border-b dark:border-gray-700 bg-gray-50 dark:text-gray-400 dark:bg-gray-800">
                <th class="px-4 py-3">ID</th>
                <th class="px-4 py-3">Type</th>
                <th class="px-4 py-3">Watched</th>
                <th class="px-4 py-3">Plex Username</th>
            </tr>
        </thead>
        <tbody class="bg-white divide-y dark:divide-gray-700 dark:bg-gray-800">
            <tr class="text-gray-700 dark:text-gray-400">
                <td class="px-4 py-3">
                    <div class="flex items-center text-sm">
                        <!-- Avatar with inset shadow -->
                        <div class="relative hidden w-8 h-8 mr-3 rounded-full md:block">
                            <img class="object-cover w-full h-full rounded-full"
                                src="https://images.unsplash.com/flagged/photo-1570612861542-284f4c12e75f?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=200&fit=max&ixid=eyJhcHBfaWQiOjE3Nzg0fQ"
                                alt="" loading="lazy" />
                            <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true">
                            </div>
                        </div>
                        <div>
                            <p class="font-semibold">Hans Burger</p>
                            <p class="text-xs text-gray-600 dark:text-gray-400">
                                10x Developer
                            </p>
                        </div>
                    </div>
                </td>
                <td class="px-4 py-3 text-sm">
                    $ 863.45
                </td>
                <td class="px-4 py-3 text-xs">
                    <span
                        class="px-2 py-1 font-semibold leading-tight text-green-700 bg-green-100 rounded-full dark:bg-green-700 dark:text-green-100">
                        Approved
                    </span>
                </td>
                <td class="px-4 py-3 text-sm">
                    6/10/2020
                </td>
            </tr>

            <tr class="text-gray-700 dark:text-gray-400">
                <td class="px-4 py-3">
                    <div class="flex items-center text-sm">
                        <!-- Avatar with inset shadow -->
                        <div class="relative hidden w-8 h-8 mr-3 rounded-full md:block">
                            <img class="object-cover w-full h-full rounded-full"
                                src="https://images.unsplash.com/photo-1494790108377-be9c29b29330?ixlib=rb-0.3.5&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=200&facepad=3&fit=facearea&s=707b9c33066bf8808c934c8ab394dff6"
                                alt="" loading="lazy" />
                            <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true">
                            </div>
                        </div>
                        <div>
                            <p class="font-semibold">Jolina Angelie</p>
                            <p class="text-xs text-gray-600 dark:text-gray-400">
                                Unemployed
                            </p>
                        </div>
                    </div>
                </td>
                <td class="px-4 py-3 text-sm">
                    $ 369.95
                </td>
                <td class="px-4 py-3 text-xs">
                    <span
                        class="px-2 py-1 font-semibold leading-tight text-orange-700 bg-orange-100 rounded-full dark:text-white dark:bg-orange-600">
                        Pending
                    </span>
                </td>
                <td class="px-4 py-3 text-sm">
                    6/10/2020
                </td>
            </tr>

            <tr class="text-gray-700 dark:text-gray-400">
                <td class="px-4 py-3">
                    <div class="flex items-center text-sm">
                        <!-- Avatar with inset shadow -->
                        <div class="relative hidden w-8 h-8 mr-3 rounded-full md:block">
                            <img class="object-cover w-full h-full rounded-full"
                                src="https://images.unsplash.com/photo-1551069613-1904dbdcda11?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=200&fit=max&ixid=eyJhcHBfaWQiOjE3Nzg0fQ"
                                alt="" loading="lazy" />
                            <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true">
                            </div>
                        </div>
                        <div>
                            <p class="font-semibold">Sarah Curry</p>
                            <p class="text-xs text-gray-600 dark:text-gray-400">
                                Designer
                            </p>
                        </div>
                    </div>
                </td>
                <td class="px-4 py-3 text-sm">
                    $ 86.00
                </td>
                <td class="px-4 py-3 text-xs">
                    <span
                        class="px-2 py-1 font-semibold leading-tight text-red-700 bg-red-100 rounded-full dark:text-red-100 dark:bg-red-700">
                        Denied
                    </span>
                </td>
                <td class="px-4 py-3 text-sm">
                    6/10/2020
                </td>
            </tr>

            <tr class="text-gray-700 dark:text-gray-400">
                <td class="px-4 py-3">
                    <div class="flex items-center text-sm">
                        <!-- Avatar with inset shadow -->
                        <div class="relative hidden w-8 h-8 mr-3 rounded-full md:block">
                            <img class="object-cover w-full h-full rounded-full"
                                src="https://images.unsplash.com/photo-1546456073-6712f79251bb?ixlib=rb-1.2.1&q=80&fm=jpg&crop=entropy&cs=tinysrgb&w=200&fit=max&ixid=eyJhcHBfaWQiOjE3Nzg0fQ"
                                alt="" loading="lazy" />
                            <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true">
                            </div>
                        </div>
                        <div>
                            <p class="font-semibold">Wenzel Dashington</p>
                            <p class="text-xs text-gray-600 dark:text-gray-400">
                                Actor
                            </p>
                        </div>
                    </div>
                </td>
                <td class="px-4 py-3 text-sm">
                    $ 863.45
                </td>
                <td class="px-4 py-3 text-xs">
                    <span
                        class="px-2 py-1 font-semibold leading-tight text-gray-700 bg-gray-100 rounded-full dark:text-gray-100 dark:bg-gray-700">
                        Expired
                    </span>
                </td>
                <td class="px-4 py-3 text-sm">
                    6/10/2020
                </td>
            </tr>

        </tbody>
    </table>
</template>