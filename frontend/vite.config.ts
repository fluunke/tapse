import { svelte } from '@sveltejs/vite-plugin-svelte'
import WindiCSS from 'vite-plugin-windicss'
import { defineConfig } from 'vite'

export default defineConfig({
    plugins: [
        WindiCSS(),
        svelte(),
    ],
})
