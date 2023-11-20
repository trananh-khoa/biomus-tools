<script lang="ts" setup>
import { ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport } from 'radix-vue'
</script>

<template>
  <!-- https://www.radix-vue.com/components/scroll-area.html -->
  <ScrollAreaRoot un-overflow-hidden>
    <!-- Content (horizontal padding to avoid vertical scrollbar) -->
    <!-- Need to set h-full manually -->
    <ScrollAreaViewport un-h-full as-child>
      <slot />
    </ScrollAreaViewport>
    <!-- Horizontal scroll -->
    <ScrollAreaScrollbar
      orientation="horizontal"
      un-bg="black/50"
      un-h="2.5"
      un-select-none un-touch-none un-flex un-p-0.5
    >
      <ScrollAreaThumb class="bg-rainbow" un-rounded-full />
    </ScrollAreaScrollbar>
    <!-- Vertical scroll -->
    <ScrollAreaScrollbar
      orientation="vertical"
      un-p-0.5
      un-select-none
      un-touch-none
      un-bg="black/50"
      un-w="2.5"
    >
      <ScrollAreaThumb class="bg-rainbow" un-rounded-full />
    </ScrollAreaScrollbar>
    <!-- Scroll corner -->
    <ScrollAreaCorner un-h-0.25 un-w-0.25 un-bg="black/50" />
  </ScrollAreaRoot>
</template>
