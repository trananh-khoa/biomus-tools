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
  <div un-pt-4 un-px-2>
    <!-- Title -->
    <h1 class="text-rainbow-gradient" un-font-700 un-mb-4 un-text="3xl">
      Biomedical Ultrasound Tools
    </h1>
    <!-- Description -->
    <p un-mb-8 un-text-neutral-400>
      BiomUS-Tools is a collection of useful programs built for the <span un-font-700>Biomedical Ultrasound Laboratory</span>
      at <span un-font-700>Carleton University</span>.
    </p>
    <!-- Features -->
    <div un-grid="~ gap-4 sm:cols-2 md:cols-3" un-text-left>
      <div
        v-for="{ description, icon, title } in features"
        :key="title"
        un-bg-neutral-800
        un-p-4
        un-rounded-xl
      >
        <div un-bg-neutral-600 un-rounded-md un-inline-block un-mb-2 un-p-2>
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
  </div>
</template>
