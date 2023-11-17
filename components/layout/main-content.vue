<script lang="ts" setup>
</script>

<template>
  <!-- https://www.radix-vue.com/components/scroll-area.html -->
  <!-- Main (and scroll area root + viewport) content should fill all available space -->
  <!-- Flex takes care of this -->
  <UiScrollArea un-bg-zinc-900 un-border="~ b-0 l-0 t-1 r-0 black solid">
    <slot />
  </UiScrollArea>
</template>
