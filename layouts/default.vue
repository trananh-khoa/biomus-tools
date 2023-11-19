<script lang="ts" setup>

</script>

<template>
  <!-- Layout should always be full screen -->
  <div un-flex="~ col" un-h-full>
    <LayoutTitleBar />
    <div un-flex un-overflow-hidden un-flex-grow>
      <LayoutSideBar />
      <LayoutMainContent un-flex-grow>
        <slot />
      </LayoutMainContent>
    </div>
  </div>
</template>
