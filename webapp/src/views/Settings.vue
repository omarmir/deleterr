<template>
  <h2 class="my-6 text-2xl font-semibold text-gray-700 dark:text-gray-200">Settings</h2>
  <div class="flex flex-col space-y-5">
    <CTA action-link="https://github.com/omarmir/deleterr">
      <span>This is ALPHA software. Features are unstable. Please report any issues on GitHub!</span>
      <template #action>
        <span>GitHub &RightArrow;</span>
      </template>
    </CTA>
    <NewPassword></NewPassword>
    <Suspense>
      <Settings />
      <template #fallback>
        <PageLoading>
          <template #message>Loading</template>
        </PageLoading>
      </template>
    </Suspense>
  </div>
</template>

<script lang="ts" setup>
import PageLoading from '~/components/PageLoading.vue'
import NewPassword from '~/components/NewPassword.vue'
import CTA from '~/components/CTA.vue'
import Settings from '~/components/Settings.vue'
</script>
