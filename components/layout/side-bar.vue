<script setup lang="ts">
import {
  NavigationMenuIndicator,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuRoot,
} from 'radix-vue'
</script>

<template>
  <!-- Scroll area -->
  <UiScrollArea un-border="~ b-0 l-0 r-1 t-1 black solid" un-w-48>
    <!-- Navigation -->
    <NavigationMenuRoot orientation="vertical">
      <NavigationMenuList un-list-none un-p-0>
        <NavigationMenuItem>
          <NavigationMenuLink as-child>
            <NuxtLink to="/">
              Home
            </NuxtLink>
          </NavigationMenuLink>
        </NavigationMenuItem>
        <NavigationMenuItem>
          <NavigationMenuLink as-child>
            <NuxtLink to="/artlab/converter">
              Converter
            </NuxtLink>
          </NavigationMenuLink>
        </NavigationMenuItem>
        <NavigationMenuIndicator />
      </NavigationMenuList>
    </NavigationMenuRoot>
  </UiScrollArea>
</template>

<style lang="postcss" scoped>
.category {
  @apply font-700 text-neutral-500 text-xs tracking-wide;
}
</style>
