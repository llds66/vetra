<script setup lang="ts">
import { useRouter } from 'vue-router'

withDefaults(
  defineProps<{
    title?: string
  }>(),
  {
    title: '标题',
  },
)

const router = useRouter()
</script>

<template>
  <header class="hearder_container">
    <button class="header_btn" type="button" @click="router.back()">
      <div class="i-iconamoon-arrow-left-6-circle-light text-xl" />
    </button>
    <div class="text-center">
      {{ title }}
    </div>
    <div aria-hidden="true" />
  </header>
</template>

<style scoped>
.hearder_container {
  --uno: 'mb-2 gap-3 grid grid-cols-[36px_minmax(0,1fr)_36px] items-center ';
}

.header_btn {
  --uno: 'text-white flex h-9 w-9 items-center justify-center rounded-full cursor-pointer hover:bg-white/14';
}
</style>
