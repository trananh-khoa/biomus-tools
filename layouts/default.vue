<script lang="ts" setup>

</script>

<template>
  <!-- Layout should always be full screen -->
  <div un-flex="~ col" un-h-full>
    <div un-flex un-flex-grow un-overflow-hidden>
      <LayoutSideBar />
      <LayoutMainContent un-flex-grow>
        <slot />
      </LayoutMainContent>
    </div>
    <LayoutStatusBar />
  </div>
</template>
