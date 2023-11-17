<script lang="ts" setup>
import {
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuRoot,
} from 'radix-vue'

// Manually specify links to pages
const sections = [
  {
    category: 'General',
    items: [
      { icon: 'i-ph-house', label: 'Home', to: '/' },
    ],
  },
  {
    category: 'Tools',
    items: [
      { icon: 'i-ph-arrows-clockwise', label: 'File Converter', to: '/file-converter' },
      { icon: 'i-ph-images', label: 'Image Extraction', to: '/image-extraction' },
    ],
  },
]
</script>

<template>
  <!-- Scroll area -->
  <UiScrollArea un-border="~ b-0 l-0 r-1 t-1 black solid" un-min-w-48 as="div">
    <!-- Navigation -->
    <NavigationMenuRoot orientation="vertical">
      <NavigationMenuList un-list-none un-mt-4>
        <div v-for="{ category, items } in sections" :key="category" un-mb-8 un-flex="~ col gap-1">
          <div un-mb-1 un-ml-2 un-text="xs neutral-400">
            {{ category }}
          </div>
          <NavigationMenuItem v-for="{ icon, label, to } in items" :key="to">
            <NavigationMenuLink un-bg="hover:neutral-700" un-transition="colors" un-rounded-md un-px-2 un-py-1 as-child un-w-full>
              <NuxtLink :to="to" un-flex un-select-none un-decoration-none un-gap-2 un-items-center un-text="sm white">
                <div :class="icon" />
                {{ label }}
              </NuxtLink>
            </NavigationMenuLink>
          </NavigationMenuItem>
        </div>
      </NavigationMenuList>
    </NavigationMenuRoot>
  </UiScrollArea>
</template>

<style lang="postcss" scoped>
/* Style active link */
a.router-link-active {
  background-color: var(--bg-colour);
  font-weight: 700;
}
</style>
