// windi.config.js
import { defineConfig } from 'windicss/helpers'

export default defineConfig({
    extract: {
        include: ['src/**/*.{html,vue,jsx,tsx,svelte}'],
    },
    // Utilities used with svelte's 'class:' syntax can't be scanned by windicss
    safelist: 'rotate-45 rotate-0 font-semibold bg-blue-200 border-dotted border-3 border-gray-800 bg-gray-200"',
})
