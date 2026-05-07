<script setup lang="ts">
import type { MenuOption } from 'naive-ui'
import type { RouteRecordRaw } from 'vue-router'
import { computed, h } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'
import { routes } from 'vue-router/auto-routes'

const route = useRoute()
const appRouter = useRouter()

function getTabRoutes(routes: readonly RouteRecordRaw[]): RouteRecordRaw[] {
  return routes.flatMap((item) => {
    const children = item.children ? getTabRoutes(item.children) : []
    return item.meta?.isTab ? [item, ...children] : children
  })
}

function getTabOrder(route: RouteRecordRaw) {
  return typeof route.meta?.tabOrder === 'number'
    ? route.meta.tabOrder
    : Number.MAX_SAFE_INTEGER
}

const menuOptions = computed<MenuOption[]>(() =>
  getTabRoutes(routes)
    .sort((a, b) => getTabOrder(a) - getTabOrder(b))
    .flatMap((item) => {
      const name = typeof item.name === 'string' ? item.name : ''
      const tabsName = typeof item.meta?.tabsName === 'string' ? item.meta.tabsName : ''

      if (!name || !tabsName) {
        return []
      }

      return [
        {
          label: () =>
            h(
              RouterLink as any,
              {
                to: {
                  name,
                },
              },
              { default: () => tabsName },
            ),
          key: name,
        },
      ]
    }),
)

const activeKey = computed(() => {
  const name = route.name
  return typeof name === 'string' ? name : ''
})
</script>

<template>
  <div class="tabs-header-nav">
    <n-menu class="tabs-menu" mode="horizontal" :value="activeKey" :options="menuOptions" />
    <button type="button" aria-label="打开设置" class="settings-btn" @click="appRouter.push({ name: 'Settings' })">
      <div class="settings-icon" aria-hidden="true" />
      <span>设置</span>
    </button>
  </div>
</template>

<style scoped>
.tabs-header-nav {
  --uno: flex items-center gap-3;
}

.tabs-menu {
  --uno: min-w-0 flex-1;
  --tab-hover-bg: rgba(255, 255, 255, 0.14);
  --tab-active-bg: rgba(255, 255, 255, 0.14);
  --tab-active-text: #fff;
}

.tabs-menu :deep(.n-menu-item) {
  height: 32px;
  margin-right: 8px;
}

.tabs-menu :deep(.n-menu-item-content) {
  min-height: 28px;
  padding: 0 10px;
  border-radius: 8px;
  transition: background-color 0.2s ease, color 0.2s ease;
}

.tabs-menu :deep(.n-menu-item-content-header a) {
  color: #fff;
}

.tabs-menu :deep(.n-menu-item-content:hover) {
  background-color: var(--tab-hover-bg);
}

.tabs-menu :deep(.n-menu-item-content--selected) {
  background-color: var(--tab-active-bg);
  color: var(--tab-active-text);
}

.tabs-menu :deep(.n-menu-item--selected .n-menu-item-content) {
  background-color: var(--tab-active-bg);
  color: var(--tab-active-text);
}

.settings-btn {
  --uno: flex w-15 py-1 cursor-pointer items-center justify-center gap-1 rounded-md text-white transition-all duration-200 hover:bg-white/14 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-white/55;
}

.settings-icon {
  --uno: i-iconamoon-settings;
}
</style>
