<route lang="json5">
{
  name: 'Start',
  meta: {
    layout: 'main',
    isTab: true,
    tabsName: '开始',
    tabOrder: 2,
  },
}
</route>

<script setup lang="ts">
import { openUrl } from '@tauri-apps/plugin-opener'

function toUrl(url: string) {
  openUrl(url)
}
</script>

<template>
  <div>
    <n-card title="简介">
      这是一个面向 Windows 的桌面应用起步模板，基于 Vue 3 与 Tauri 2 构建，组件库采用Naive UI。
    </n-card>
    <n-card title="组件展示" class="mt-5">
      <template #header-extra>
        <div
          class="text-stone-400 flex gap-1 cursor-default items-center hover:text-stone-300"
          @click="toUrl('https://www.naiveui.com/')"
        >
          参考 Naive UI
          <div class="i-mdi-link-variant" />
        </div>
      </template>
      <div class="">
        <div class="flex gap-2">
          <n-button>Default</n-button>
          <n-button type="tertiary">
            Tertiary
          </n-button>
          <n-button type="primary">
            Primary
          </n-button>
        </div>
        <n-divider />
        <div class="flex gap-2">
          <n-tag> 爱在西元前 </n-tag>
          <n-tag type="success">
            不该
          </n-tag>
          <n-tag type="warning">
            超人不会飞
          </n-tag>
          <n-tag type="error">
            手写的从前
          </n-tag>
        </div>
        <n-divider />
        <n-slider :step="1" />
        <n-divider />
        <n-switch />
        <n-divider />
        <n-input type="text" placeholder="基本的 Input" />
        <n-divider />
        <n-input type="textarea" placeholder="基本的 Textarea" />
      </div>
    </n-card>
    <n-card title="变化" class="mt-5">
      <n-timeline>
        <n-timeline-item type="info" title="新增开始自启功能" time="2026年4月7日 16:57" />
        <n-timeline-item type="info" title="支持关闭与托盘最小化配置" time="2026年4月2日 16:49" />
        <n-timeline-item type="info" title="项目命名为 Vetra" time="2026年3月28日 18:03" />
        <n-timeline-item type="success" title="项目初始化" time="2026年3月28日 17:51" line-type="dashed" />
      </n-timeline>
    </n-card>
  </div>
</template>
