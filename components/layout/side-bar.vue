<script lang="ts" setup>
import {
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuRoot,
} from 'radix-vue'

const sections = [
  {
    category: 'General',
    items: [
      { icon: 'i-ph-house', label: 'Home', to: '/' },
    ],
  },
  {
    category: 'ART.LAB',
    items: [
      { icon: 'i-ph-arrows-clockwise', label: 'File Converter', to: '/artlab/converter' },
    ],
  },
]
</script>

<template>
  <!-- Scroll area -->
  <UiScrollArea un-border="~ b-0 l-0 r-1 t-1 black solid" un-w-48>
    <!-- Navigation -->
    <NavigationMenuRoot orientation="vertical">
      <NavigationMenuList un-list-none>
        <div v-for="{ category, items } in sections" :key="category" un-mb-4>
          <div un-mb-1 un-ml-2 un-text="xs neutral-500">
            {{ category }}
          </div>
          <NavigationMenuItem v-for="{ icon, label, to } in items" :key="to" un-flex>
            <NavigationMenuLink un-bg="hover:neutral-700" un-transition="colors" un-px-2 un-py-1 as-child un-rounded-md un-w-full>
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
  @apply bg-blue-800
}
</style>
