// windi.config.js
import { defineConfig } from 'windicss/helpers'

export default defineConfig({
    extract: {
        include: ['src/**/*.{html,vue,jsx,tsx,svelte}'],
    },
    safelist: 'rotate-45 rotate-0 font-semibold',
})
