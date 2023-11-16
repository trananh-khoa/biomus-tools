<script lang="ts" setup>

</script>

<template>
  <!-- Layout should always be full screen -->
  <div un-flex="~ col" un-h-full>
    <LayoutTitleBar />
    <LayoutMainContent un-flex-grow>
      <slot />
    </LayoutMainContent>
    <LayoutStatusBar />
  </div>
</template>

<style lang="postcss">
/* Layout should be full screen */
html, body, #__nuxt {
  @apply m-0 h-full;
}
</style>
