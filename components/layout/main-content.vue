<script lang="ts" setup>
</script>

<template>
  <!-- https://www.radix-vue.com/components/scroll-area.html -->
  <!-- Main (and scroll area root + viewport) content should fill all available space -->
  <!-- Flex takes care of this -->
  <UiScrollArea un-bg="black/55">
    <div un-h-full un-p="x-6 t-3 b-6">
      <slot />
    </div>
  </UiScrollArea>
</template>
