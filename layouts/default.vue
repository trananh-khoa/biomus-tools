<script lang="ts" setup>
import { useToastStore } from '@/stores/toast'

const { description, open, title } = storeToRefs(useToastStore())
</script>

<template>
  <!-- Layout should always be full screen -->
  <div un-h-full un-overflow-hidden>
    <ToastProvider>
      <!-- Page -->
      <div un-flex="~ col" un-h-full>
        <LayoutTitleBar />
        <div un-overflow-hidden un-flex un-flex-grow>
          <LayoutSideBar />
          <LayoutMainContent un-flex-grow>
            <slot />
          </LayoutMainContent>
        </div>
      </div>
      <!-- Toasts -->
      <ToastRoot v-model:open="open">
        <ToastTitle>
          {{ title }}
        </ToastTitle>
        <ToastDescription>
          {{ description }}
        </ToastDescription>
      </ToastRoot>
      <ToastViewport un-pos="fixed top-0 right-0" />
    </ToastProvider>
  </div>
</template>
