import { resolve } from 'node:path'
import vue from '@vitejs/plugin-vue'
import MotionResolver from 'motion-v/resolver'
import UnoCSS from 'unocss/vite'
import AutoImport from 'unplugin-auto-import/vite'
import { NaiveUiResolver } from 'unplugin-vue-components/resolvers'
import Components from 'unplugin-vue-components/vite'
import { defineConfig } from 'vite'
import VueRouter from 'vue-router/vite'

const host = process.env.TAURI_DEV_HOST

// https://vite.dev/config/
export default defineConfig(async () => ({
  plugins: [
    VueRouter({
      dts: 'src/route-map.d.ts',
      routesFolder: 'src/pages',
      exclude: ['**/components/**'],
    }),
    vue(),
    AutoImport({
      imports: [
        'vue',
        {
          'naive-ui': ['useDialog', 'useMessage', 'useNotification', 'useLoadingBar'],
        },
      ],
    }),
    Components({
      dts: true,
      resolvers: [NaiveUiResolver(), MotionResolver()],
    }),
    UnoCSS(),
  ],

  // 别名配置
  resolve: {
    alias: {
      '@': resolve(__dirname, 'src'),
      '@pages': resolve(__dirname, 'src/pages'),
      '@layouts': resolve(__dirname, 'src/layouts'),
      '@components': resolve(__dirname, 'src/components'),
      '@composables': resolve(__dirname, 'src/common/composables'),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent Vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
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
      // 3. tell Vite to ignore watching `src-tauri`
      ignored: ['**/src-tauri/**'],
    },
  },
}))
