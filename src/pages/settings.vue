<route lang="json5">
{
  name: 'Settings',
  meta: {
    layout: 'settings',
  },
}
</route>

<script setup lang="ts">
import type { CloseBehavior } from '@/utils/desktop-preferences'
import { disable as disableAutostart, enable as enableAutostart, isEnabled as isAutostartEnabled } from '@tauri-apps/plugin-autostart'
import { openUrl } from '@tauri-apps/plugin-opener'

import { computed, onMounted, ref } from 'vue'
import { appConfig } from '@/config/app'
import {
  getCloseBehavior,
  setCloseBehavior,
  syncCloseBehavior,
} from '@/utils/desktop-preferences'

const isAutoStart = ref(false)
const isAutoStartLoading = ref(false)
const closeBehavior = ref<CloseBehavior>(getCloseBehavior())
const closeBehaviorOptions = computed(() => [
  {
    label: '退出程序',
    value: 'exit' satisfies CloseBehavior,
  },
  {
    label: '最小化到托盘',
    value: 'tray' satisfies CloseBehavior,
  },
])

async function openAuthorSite(url: string) {
  await openUrl(url)
}

async function syncAutostartState() {
  try {
    isAutoStart.value = await isAutostartEnabled()
  }
  catch (error) {
    console.error('Failed to load autostart setting', error)
    isAutoStart.value = false
  }
}

async function handleAutoStartChange(value: boolean) {
  isAutoStartLoading.value = true

  try {
    if (value)
      await enableAutostart()
    else
      await disableAutostart()

    isAutoStart.value = value
  }
  catch (error) {
    console.error('Failed to update autostart setting', error)
    await syncAutostartState()
  }
  finally {
    isAutoStartLoading.value = false
  }
}

async function handleCloseBehaviorChange(value: CloseBehavior) {
  closeBehavior.value = value
  setCloseBehavior(value)
  await syncCloseBehavior(value)
}

onMounted(() => {
  void syncAutostartState()
})
</script>

<template>
  <div class="flex flex-1 flex-col min-h-0">
    <div class="flex flex-col gap-2">
      <n-card>
        <div class="flex items-center justify-between">
          <span>开机自启</span>
          <n-switch
            :value="isAutoStart"
            :loading="isAutoStartLoading"
            @update:value="handleAutoStartChange"
          />
        </div>
      </n-card>
      <n-card>
        <div class="flex gap-4 items-center justify-between">
          <div class="flex flex-col gap-1">
            <span>关闭时</span>
            <span class="text-xs text-white/55">
              点击关闭按钮后的行为
            </span>
          </div>
          <n-select
            :value="closeBehavior"
            :options="closeBehaviorOptions"
            class="shrink-0 w-32"
            size="small"
            @update:value="handleCloseBehaviorChange"
          />
        </div>
      </n-card>
      <n-card>
        <div class="flex items-center justify-between">
          <span>版本信息</span>
          <div>
            {{ appConfig.version }}
          </div>
        </div>
      </n-card>
    </div>

    <footer class="text-xs text-white/55 mt-auto pt-4 flex items-center justify-between">
      <span class="inline-flex gap-1 items-center">
        Built with
        <button class="link" type="button" @click="openAuthorSite(appConfig.projectTemplateUrl)">
          {{ appConfig.projectTemplate }}
        </button>
      </span>
      <span class="inline-flex gap-1 items-center">
        ©2026
        <button class="link" type="button" @click="openAuthorSite(appConfig.authorUrl)">
          {{ appConfig.authorName }}
        </button>
      </span>
    </footer>
  </div>
</template>

<style scoped>
.link {
  --uno: 'ml-1 cursor-pointer text-white/80 transition hover:text-white'
}
</style>
