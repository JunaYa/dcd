import path from 'node:path'
import vue from '@vitejs/plugin-vue'
import UnoCSS from 'unocss/vite'
import { defineConfig } from 'vitest/config'

const host = process.env.TAURI_DEV_HOST

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue(), UnoCSS()],

  resolve: {
    alias: {
      '~': path.resolve(import.meta.dirname, 'src'),
    },
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1422,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: 'ws',
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
  test: {
    include: ['tests/**/*.test.mjs'],
    environment: 'node',
  },
  build: {
    rolldownOptions: {
      input: {
        main: path.resolve(import.meta.dirname, './main.html'),
        task: path.resolve(import.meta.dirname, './task.html'),
        setting: path.resolve(import.meta.dirname, './setting.html'),
        preview: path.resolve(import.meta.dirname, './preview.html'),
        startup: path.resolve(import.meta.dirname, './startup.html'),
      },
    },
  },
}))
