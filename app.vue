<script lang="ts" setup>
// https://unocss.dev/guide/style-reset
import '@unocss/reset/tailwind.css'
import '~/assets/rainbow.css'
</script>

<template>
  <!-- https://nuxt.com/docs/guide/directory-structure/layouts -->
  <NuxtLayout>
    <NuxtPage />
  </NuxtLayout>
</template>

<style lang="postcss">
/* Layout should be full screen */
html, body, #__nuxt {
  @apply m-0 h-full;
}
/* Background is transparent */
/* Default text is tight and white */
body {
  @apply antialiased bg-gray-800/75 font-sans font-400 text-sm text-white;
}
/* https://nuxt.com/docs/getting-started/transitions */
.page-enter-active,
.page-leave-active {
  transition: all 0.4s;
}
.page-enter-from,
.page-leave-to {
  opacity: 0;
  filter: blur(1rem);
}
</style>
