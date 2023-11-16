// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  app: {
    pageTransition: {
      mode: 'out-in',
      name: 'page',
    },
  },
  devtools: { enabled: true },
  modules: [
    '@unocss/nuxt',
    'radix-vue/nuxt',
  ],
  ssr: false,
})
