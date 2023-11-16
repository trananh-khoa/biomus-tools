<script lang="ts" setup>
import { ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport } from 'radix-vue'
</script>

<template>
  <!-- https://www.radix-vue.com/components/scroll-area.html -->
  <ScrollAreaRoot un-overflow-hidden>
    <!-- Content (horizontal padding to avoid vertical scrollbar) -->
    <!-- Need to set h-full manually -->
    <ScrollAreaViewport un-h-full>
      <!-- Padding must be done inside viewport -->
      <div un-px-6 un-py-4>
        <slot />
      </div>
    </ScrollAreaViewport>
    <!-- Horizontal scroll -->
    <ScrollAreaScrollbar
      orientation="horizontal"
      un-select-none
      un-touch-none
    >
      <ScrollAreaThumb />
    </ScrollAreaScrollbar>
    <!-- Vertical scroll -->
    <ScrollAreaScrollbar
      orientation="vertical"
      un-p="x-0.75 y-1"
      un-select-none
      un-touch-none
      un-w="1.5"
    >
      <ScrollAreaThumb un-bg="neutral-500/25" un-rounded-full />
    </ScrollAreaScrollbar>
    <!-- Scroll corner -->
    <ScrollAreaCorner />
  </ScrollAreaRoot>
</template>
