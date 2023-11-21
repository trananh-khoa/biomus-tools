<script lang="ts" setup>
import { open } from '@tauri-apps/api/dialog'
import { Separator } from 'radix-vue'
import { downloadDir } from '@tauri-apps/api/path'
import { invoke } from '@tauri-apps/api/tauri'
import { useToastStore } from '@/stores/toast'

// Toasts
const { description, open: toastOpen, timer, title } = storeToRefs(useToastStore())

// Visual feedback
const conversionStatus: Ref<string> = ref('WAITING')
const convertButtonIcon = computed(() => {
  switch (conversionStatus.value) {
    case 'READY': return 'i-ph-files'
    case 'BUSY': return 'i-ph-hourglass text-yellow-500 animate-spin pointer-events-none'
    case 'COMPLETE': return 'i-ph-check-circle text-green-500'
    case 'ERROR': return 'i-ph-x-circle text-red-500'
    default: return 'i-ph-warning-circle'
  }
})

// File selection
const files: Ref<string[]> = ref([])
async function handleSelectFiles() {
  const selected = await open({
    filters: [{ extensions: ['zrf'], name: 'ART.LAB' }],
    multiple: true,
  })
  if (Array.isArray(selected)) {
    files.value = selected
    conversionStatus.value = 'READY'
  }
}

// Save folder location
const saveLocation: Ref<string> = ref(await downloadDir())
async function handleSaveLocation() {
  const selected = await open({
    directory: true,
    multiple: false,
  })
  if (typeof selected === 'string')
    saveLocation.value = selected
}

// File conversion
async function handleConvertSelectedFiles() {
  conversionStatus.value = 'BUSY'
  await invoke('convert', { files: files.value, saveLocation: saveLocation.value })
  // await invoke('convert', { files: files.value, saveLocation: 'hello' })
    .then(() => {
      conversionStatus.value = 'COMPLETE'
    })
    .catch((err) => {
      conversionStatus.value = 'ERROR'

      toastOpen.value = false
      title.value = 'Error occurred when converting selected files'
      description.value = err
      window.clearTimeout(timer.value)
      timer.value = window.setTimeout(() => {
        toastOpen.value = true
      }, 100)
    })
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
        <UiScrollArea v-if="files.length > 0" un-flex-grow un-rounded-md un-bg-zinc-800 un-pb-3>
          <ul un-text-neutral-400>
            <li v-for="f in files" :key="f">
              <div un-py-1 un-pl-2 un-pr-4>
                {{ f }}
              </div>
              <Separator un-h-0.25 un-bg-zinc-700 />
            </li>
          </ul>
        </UiScrollArea>
        <div v-else un-flex="~ col gap-2" un-flex-grow un-rounded-md un-bg-zinc-800 un-items-center un-justify-center un-text="sm neutral-400">
          <div class="i-ph-warning-circle" />
          <div>No files selected</div>
        </div>
      </div>
      <!-- Conversion settings -->
      <div un-flex="~ col gap-3">
        <!-- Label -->
        <div>
          <div un-font-700 un-mb-2>
            Output Settings
          </div>
          <Separator un-h-0.25 un-bg-zinc-700 />
        </div>
        <!-- Save folder -->
        <div>
          <div un-text="sm neutral-400" un-mb-1>
            Save Location
          </div>
          <div un-flex="~ gap-x-2">
            <div un-border="~ solid zinc-500" un-rounded-md un-flex-grow un-px-1 un-py-0.5 un-truncate>
              {{ saveLocation }}
            </div>
            <UIButton @click="handleSaveLocation">
              ...
            </UIButton>
          </div>
        </div>
        <div>
          <UIButton un-w-full :disabled="files.length === 0" un-justify-between @click="handleConvertSelectedFiles">
            <div>Convert {{ files.length }} Selected File(s)</div>
            <div :class="convertButtonIcon" />
          </UIButton>
        </div>
      </div>
    </div>
  </LayoutPageContent>
</template>

<style>

</style>
