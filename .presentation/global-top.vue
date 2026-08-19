<script setup lang="ts">
import { useSlideContext } from '@slidev/client'
import type { TocItem } from '@slidev/types'
import { computed, ref } from 'vue'

const { $nav } = useSlideContext()
const open = ref(false)

// Active branch of the Toc tree, root first, so it doubles as a breadcrumb trail.
const breadcrumb = computed<TocItem[]>(() => {
  const path: TocItem[] = []
  let level = $nav.value.tocTree
  while (level.length > 0) {
    const active = level.find(item => item.active || item.activeParent)
    if (!active)
      break
    path.push(active)
    level = active.children
  }
  return path
})
</script>

<template>
  <div class="fixed top-0 left-0 right-0 z-40 print:hidden">
    <button
      class="w-full flex items-center gap-2 px-4 py-2 text-sm bg-main/90 backdrop-blur border-b border-main text-left"
      @click="open = !open"
    >
      <template v-for="(item, i) in breadcrumb" :key="item.path">
        <span :class="i === breadcrumb.length - 1 ? 'font-semibold' : 'opacity-60'">{{ item.title }}</span>
        <span v-if="i < breadcrumb.length - 1" class="opacity-40">/</span>
      </template>
      <div class="i-carbon:chevron-down ml-auto opacity-50 transition-transform" :class="{ 'rotate-180': open }" />
    </button>
    <div
      v-show="open"
      class="max-h-[70vh] overflow-y-auto bg-main/95 backdrop-blur border-b border-main shadow-lg p-3 text-sm"
    >
      <Toc :columns="1" />
    </div>
  </div>
</template>
