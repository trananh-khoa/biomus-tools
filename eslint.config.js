import antfu from '@antfu/eslint-config'
import unocss from '@unocss/eslint-config/flat'

export default antfu(
  {},
  unocss,
  {
    rules: {
      'perfectionist/sort-array-includes': 'error',
      'perfectionist/sort-objects': 'error',
      'perfectionist/sort-vue-attributes': 'error',
    },
  },
)
