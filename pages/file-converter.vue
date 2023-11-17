<script lang="ts" setup>
import { open } from '@tauri-apps/api/dialog'

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
  <div un-flex="~ col" un-h-full un-px-2 un-pt-4>
    <!-- Title -->
    <h1 class="text-rainbow-gradient" un-mb-8 un-font-700 un-text="3xl">
      File Converter
    </h1>
    <!-- File conversion -->
    <div un-grid="~ sm:cols-2 gap-6" un-overflow-hidden un-flex-grow>
      <!-- File list -->
      <div un-flex="~ col" un-overflow-hidden>
        <!-- File selection -->
        <button un-rounded-md un-bg-neutral-600 un-mb-6 un-p-1 @click="handleSelectFiles">
          Select File(s)
        </button>
        <!-- Chosen files -->
        <UiScrollArea v-if="files.length > 0" un-flex-grow un-bg-neutral-800 un-rounded-xl>
          <!-- Show files here -->
          <div v-for="f in files" :key="f">
            {{ f }}
          </div>
        </UiScrollArea>
        <div v-else un-flex="~ col gap-2" un-flex-grow un-items-center un-justify-center un-mb-16>
          <div class="i-ph-warning" />
          <div>
            No files selected.
          </div>
        </div>
      </div>
      <!-- Options -->
      <div un-flex="~ col gap-4" un-bg-neutral-800 un-rounded-xl un-p-4>
        <!-- Output Folder -->
        <div un-text="sm neutral-400">
          Output Folder
        </div>
        <!-- Signal Settings -->
        <div un-text="sm neutral-400">
          Signal Settings
        </div>
      </div>
    </div>
  </div>
</template>

<style>

</style>
