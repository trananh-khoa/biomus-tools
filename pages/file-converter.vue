<script lang="ts" setup>
import { open } from '@tauri-apps/api/dialog'
import { Separator } from 'radix-vue'

const files: Ref<string[]> = ref([])
async function handleSelectFiles() {
  const selected = await open({
    filters: [{ extensions: ['zrf'], name: 'ART.LAB' }],
    multiple: true,
  })
  if (Array.isArray(selected))
    files.value = selected
}
</script>

<template>
  <LayoutPageContent>
    <template #title>
      File Converter
    </template>
    <div un-grid="~ sm:cols-2 gap-6" un-flex-grow un-h-0>
      <!-- File selection -->
      <div un-flex="~ col" un-overflow-hidden>
        <!-- Buttons and controls -->
        <div un-flex="~ gap-2" un-mb-4>
          <UIButton @click="handleSelectFiles">
            Select Files
          </UIButton>
        </div>
        <!-- File list -->
        <UiScrollArea v-if="files.length > 0" un-flex-grow un-text-sm un-rounded-md un-bg-zinc-800 un-pr-3 un-pb-3>
          <ul un-text-neutral-400>
            <li v-for="f in files" :key="f" un-pos-relative>
              <div un-px-2 un-py-1>
                {{ f }}
              </div>
              <Separator un-h-0.25 un-bg-zinc-700 />
            </li>
          </ul>
        </UiScrollArea>
        <div v-else un-flex="~ col" un-flex-grow un-rounded-md un-bg-zinc-800 un-items-center un-justify-center un-text="sm neutral-400">
          <div>No files selected</div>
        </div>
      </div>
      <!-- Conversion settings -->
      <div>Conversion settings</div>
    </div>
  </LayoutPageContent>
</template>

<style>

</style>
