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
  @apply antialiased bg-neutral-800/80 font-sans font-400 text-white
}
</style>
