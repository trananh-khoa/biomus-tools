<script lang="ts" setup>
const features = [
  {
    description: 'Available for MacOS and Windows, use whichever one you like.',
    icon: 'i-ph-laptop',
    title: 'Cross Platform',
  },
  {
    description: 'Powered by the performant and efficient Rust programming language.',
    icon: 'i-ph-rocket-launch',
    title: 'Fast and Reliable',
  },
  {
    description: 'Beautifully crafted with Nuxt.js and UnoCSS.',
    icon: 'i-ph-paint-brush',
    title: 'Modern UI',
  },
]
</script>

<template>
  <LayoutPageContent>
    <!-- Title -->
    <template #title>
      Biomedical Ultrasound Tools
    </template>
    <!-- Description -->
    <template #description>
      BiomUS-Tools is a collection of useful programs built for the <span un-font-700>Biomedical Ultrasound Laboratory</span>
      at <span un-font-700>Carleton University</span>.
    </template>
    <!-- Features -->
    <div un-grid="~ gap-4 sm:cols-2 md:cols-3" un-text-left>
      <div
        v-for="{ description, icon, title } in features" :key="title"
        un-rounded-md un-h-full un-bg-zinc-800 un-p-4
      >
        <div un-rounded-md un-inline-block un-mb-2 un-p-2 un-bg-zinc-600>
          <div :class="icon" />
        </div>
        <div un-font-700 un-mb-2>
          {{ title }}
        </div>
        <p un-text="sm neutral-400">
          {{ description }}
        </p>
      </div>
    </div>
  </LayoutPageContent>
</template>
