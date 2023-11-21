import { defineStore } from 'pinia'

export const useToastStore = defineStore('toast', () => {
  const open = ref(false)
  const title = ref('Title')
  const description = ref('Description')
  const timer = ref(0)

  return { description, open, timer, title }
})
