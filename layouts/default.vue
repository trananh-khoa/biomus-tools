<script lang="ts" setup>

</script>

<template>
  <!-- Layout should always be full screen -->
  <div un-flex="~ col" un-h-full>
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
/* Background is transparent */
/* Default text is tight and white */
body {
  @apply bg-slate-950/60 font-sans text-white tracking-tight
}
</style>
