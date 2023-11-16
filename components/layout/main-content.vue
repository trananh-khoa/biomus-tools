<script lang="ts" setup>
import { ScrollAreaRoot, ScrollAreaScrollbar, ScrollAreaThumb, ScrollAreaViewport } from 'radix-vue'
</script>

<template>
  <!-- https://www.radix-vue.com/components/scroll-area.html -->
  <!-- Main (and scroll area root + viewport) content should fill all available space -->
  <!-- Flex takes care of this -->
  <ScrollAreaRoot un-bg-neutral-900 un-border="~ b-0 l-0 t-1 r-0 black solid" un-overflow-hidden>
    <!-- Content (horizontal padding to avoid vertical scrollbar) -->
    <!-- Need to set h-full manually -->
    <ScrollAreaViewport un-h-full un-px-6>
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
      un-bg="black/10 hover:black/20"
      un-p="x-0.75 y-1"
      un-select-none
      un-touch-none
      un-transition="colors duration-200"
      un-w="1.5"
    >
      <ScrollAreaThumb un-bg-neutral-600 un-rounded-full />
    </ScrollAreaScrollbar>
    <!-- Scroll corner -->
    <ScrollAreaCorner />
  </ScrollAreaRoot>
</template>
