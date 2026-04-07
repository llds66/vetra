import { createApp } from 'vue'
import { syncCloseBehavior } from '@/utils/desktop-preferences'
import App from './App.vue'
import router from './router'
import './styles.css'
import 'virtual:uno.css'

function setupScrollbarVisibility() {
  const root = document.documentElement
  let hasWindowFocus = document.hasFocus()
  let hasPointerInside = false

  const syncScrollbarState = () => {
    root.dataset.scrollbarActive = String(hasWindowFocus && hasPointerInside)
  }

  root.addEventListener('mouseenter', () => {
    hasPointerInside = true
    syncScrollbarState()
  })

  root.addEventListener('mouseleave', () => {
    hasPointerInside = false
    syncScrollbarState()
  })

  window.addEventListener('focus', () => {
    hasWindowFocus = true
    syncScrollbarState()
  })

  window.addEventListener('blur', () => {
    hasWindowFocus = false
    syncScrollbarState()
  })

  syncScrollbarState()
}

async function bootstrap() {
  await syncCloseBehavior()
  setupScrollbarVisibility()

  const app = createApp(App)
  app
    .use(router)
    .mount('#app')
}

void bootstrap()
