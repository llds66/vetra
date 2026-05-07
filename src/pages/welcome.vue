<route lang="json5">
{
  name: 'Welcome',
  meta: {
    layout: 'main',
    isTab: true,
    tabsName: '欢迎',
    tabOrder: 1,
  },
}
</route>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()
</script>

<template>
  <div class="flex flex-col h-full items-center justify-center">
    <div class="flex gap-2 items-center justify-center">
      <img class="h-12 w-12 object-contain" src="/logo.png" alt="Vetra Logo" />
      <div class="text-4xl font-500">Vetra</div>
    </div>
    <div class="text-stone-300 mt-10">Vue3 + Tauri2 + Naive UI</div>
    <div class="text-stone-300 mt-3">Startup Template</div>
    <n-button
      round
      class="mt-10"
      icon-placement="right"
      type="primary"
      @click="router.push('/start')"
    >
      <template #icon>
        <div class="i-mdi-menu-right" />
      </template>
      开始
    </n-button>
  </div>
</template>
