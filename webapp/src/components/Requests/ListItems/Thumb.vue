<template>
  <div>
    <img v-if="posterUrl" class="h-16 w-11" :src="posterUrl" lazy />
    <div v-else class="make-do-poster h-16 w-11 overflow-hidden bg-teal-700 p-1 text-white">
      <p>{{ title }}</p>
    </div>
  </div>
</template>
<script setup lang="ts">
import { PropType, Ref, ref } from 'vue'
import { Image, MediaType } from '~/@types/deleterr'

const props = defineProps({
  images: { type: Array<Image>, required: false },
  mediaType: { type: String as PropType<MediaType>, required: false },
  title: { type: String, required: false },
})

const posterUrl: Ref<string | undefined> = ref(undefined)

const getImage = () => {
  let posterUrls = props.images?.filter((type) => type.coverType == 'poster') ?? []
  if (props.mediaType && posterUrls.length > 0) {
    let endpoint = props.mediaType == 'movie' ? '/api/v1/json/movie/poster/' : '/api/v1/json/series/poster/'

    if (posterUrls.length > 0) {
      posterUrl.value = posterUrls[0].url?.replace('/MediaCover/', endpoint) ?? undefined
    }
  }
}

getImage()
</script>
<style lang="postcss" scoped>
.make-do-poster {
  container-type: inline-size;
  p {
    font-size: 20cqw;
  }
}
</style>
