import {
  defineConfig,
  presetAttributify,
  presetIcons,
  presetUno,
  presetWebFonts,
  transformerDirectives,
  transformerVariantGroup,
} from 'unocss'

export default defineConfig({
  presets: [
    presetUno(),
    presetAttributify({
      prefix: 'un-',
      prefixedOnly: true,
    }),
    presetIcons({}),
    presetWebFonts({
      fonts: {
        sans: 'Inter:400,700',
      },
    }),
  ],
  shortcuts: [],
  transformers: [
    transformerDirectives(),
    transformerVariantGroup(),
  ],
})
