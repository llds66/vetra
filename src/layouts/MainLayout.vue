<script setup lang="ts">
import type { RouteRecordRaw } from 'vue-router'
import { AnimatePresence, motion, MotionConfig, useReducedMotion } from 'motion-v'
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'
import { routes } from 'vue-router/auto-routes'

import Tabs from '@/layouts/components/Tabs.vue'

const route = useRoute()
const prefersReducedMotion = useReducedMotion()

function getTabRoutes(items: readonly RouteRecordRaw[]): RouteRecordRaw[] {
  return items.flatMap((item) => {
    const children = item.children ? getTabRoutes(item.children) : []
    return item.meta?.isTab ? [item, ...children] : children
  })
}

const routeOrderMap = new Map(
  getTabRoutes(routes).flatMap((item) => {
    if (typeof item.name !== 'string')
      return []

    const order = typeof item.meta?.tabOrder === 'number'
      ? item.meta.tabOrder
      : Number.MAX_SAFE_INTEGER

    return [[item.name, order] as const]
  }),
)

function getRouteKey() {
  return String(route.name)
}

function getRouteOrder() {
  const routeName = typeof route.name === 'string' ? route.name : ''
  return routeOrderMap.get(routeName) ?? Number.MAX_SAFE_INTEGER
}

const navigationDirection = ref(0)
const previousRouteOrder = ref(getRouteOrder())

watch(
  () => route.fullPath,
  () => {
    const nextRouteOrder = getRouteOrder()
    navigationDirection.value = nextRouteOrder === previousRouteOrder.value
      ? 0
      : nextRouteOrder > previousRouteOrder.value
        ? 1
        : -1
    previousRouteOrder.value = nextRouteOrder
  },
)

const currentMotionState = computed(() => {
  if (prefersReducedMotion.value) {
    return {
      initial: { opacity: 0.01 },
      animate: { opacity: 1 },
      exit: { opacity: 0.01 },
      transition: { duration: 0.12, ease: 'linear' as const },
    }
  }

  const direction = navigationDirection.value
  const enterY = direction === 0 ? 18 : direction > 0 ? 28 : -28
  const exitY = direction === 0 ? -14 : direction > 0 ? -24 : 24

  return {
    initial: {
      opacity: 0,
      y: enterY,
      filter: 'blur(10px)',
    },
    animate: {
      opacity: 1,
      y: 0,
      filter: 'blur(0px)',
    },
    exit: {
      opacity: 0,
      y: exitY,
      filter: 'blur(8px)',
    },
    transition: {
      duration: 0.28,
      ease: [0.22, 1, 0.36, 1] as const,
    },
  }
})
</script>

<template>
  <div class="layout-shell">
    <header class="tabs-header">
      <Tabs />
    </header>
    <div class="content-area">
      <div class="route-viewport">
        <MotionConfig reduced-motion="user">
          <AnimatePresence mode="wait" :initial="false">
            <motion.div
              :key="getRouteKey()"
              class="route-shell"
              :initial="currentMotionState.initial"
              :animate="currentMotionState.animate"
              :exit="currentMotionState.exit"
              :transition="currentMotionState.transition"
            >
              <div class="route-scroll">
                <slot />
              </div>
            </motion.div>
          </AnimatePresence>
        </MotionConfig>
      </div>
    </div>
  </div>
</template>

<style scoped>
.route-viewport {
  --uno: h-full overflow-hidden;
}

.route-shell {
  --uno: h-full flex flex-col;
  transform-origin: center top;
}

.route-scroll {
  --uno: h-full overflow-auto;
}

.layout-shell {
  --uno: h-screen overflow-hidden;
}

.tabs-header {
  --uno: fixed left-0 right-0 top-0 z-20 px-3 py-2;
}

.content-area {
  --uno: mt-12 box-border h-[calc(100vh-56px)] p-3;
}
</style>
