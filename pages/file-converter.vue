<script lang="ts" setup>
import { open } from '@tauri-apps/api/dialog'
import { Separator } from 'radix-vue'
import { downloadDir } from '@tauri-apps/api/path'

const files: Ref<string[]> = ref([])
async function handleSelectFiles() {
  const selected = await open({
    filters: [{ extensions: ['zrf'], name: 'ART.LAB' }],
    multiple: true,
  })
  if (Array.isArray(selected))
    files.value = selected
}

const saveLocation: Ref<string> = ref(await downloadDir())
async function handleSaveLocation() {
  const selected = await open({
    directory: true,
    multiple: false,
  })
  if (typeof selected === 'string')
    saveLocation.value = selected
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
        <UiScrollArea v-if="files.length > 0" un-flex-grow un-rounded-md un-bg-zinc-800 un-pr-3 un-pb-3>
          <ul un-text-neutral-400>
            <li v-for="f in files" :key="f">
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
          <UIButton un-w-full :class="{ 'bg-rainbow': files.length > 0 }" :disabled="files.length === 0">
            Convert {{ files.length }} Selected File(s)
          </UIButton>
        </div>
      </div>
    </div>
  </LayoutPageContent>
</template>

<style>

</style>
