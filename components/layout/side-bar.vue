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
  <UiScrollArea un-min-w-56 as="div" un-bg="black/10">
    <!-- Navigation -->
    <NavigationMenuRoot orientation="vertical" un-px-6>
      <NavigationMenuList un-list-none un-mt-4>
        <div v-for="{ category, items } in sections" :key="category" un-mb-4 un-flex="~ col gap-1">
          <div un-mb-1 un-ml-2 un-text="xs neutral-400">
            {{ category }}
          </div>
          <NavigationMenuItem v-for="{ icon, label, to } in items" :key="to">
            <NavigationMenuLink
              un-bg="hover:zinc-700" un-border="1 solid transparent hover:[--text-colour]"
              un-transition="colors" as-child un-rounded-md un-px-2 un-py-1 un-w-full
            >
              <NuxtLink :to="to" un-flex un-select-none un-gap-2 un-items-center un-text-sm>
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
  border-color: var(--text-colour);
  font-weight: 700;
}
</style>
