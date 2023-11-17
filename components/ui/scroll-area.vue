<script lang="ts" setup>
import { ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport } from 'radix-vue'
</script>

<template>
  <!-- https://www.radix-vue.com/components/scroll-area.html -->
  <ScrollAreaRoot un-overflow-hidden>
    <!-- Content (horizontal padding to avoid vertical scrollbar) -->
    <!-- Need to set h-full manually -->
    <ScrollAreaViewport as-child un-h-full un-px-4 un-py-4>
      <!-- Padding must be done inside viewport -->
      <slot />
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
      un-p="x-0.5 y-1"
      un-select-none
      un-touch-none
      un-bg="black/50"
      un-w="2.5"
    >
      <ScrollAreaThumb class="bg-rainbow" un-rounded-full />
    </ScrollAreaScrollbar>
    <!-- Scroll corner -->
    <ScrollAreaCorner />
  </ScrollAreaRoot>
</template>
