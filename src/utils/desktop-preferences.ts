import { invoke } from '@tauri-apps/api/core'

// 当前使用的关闭行为存储键。
const CLOSE_BEHAVIOR_KEY = 'vetra.closeBehavior'
// 兼容旧版布尔配置，迁移后会清理。
const LEGACY_CLOSE_TO_TRAY_KEY = 'vetra.closeToTrayEnabled'

export type CloseBehavior = 'exit' | 'tray'

const DEFAULT_CLOSE_BEHAVIOR: CloseBehavior = 'exit'

export function getCloseBehavior(): CloseBehavior {
  const rawValue = window.localStorage.getItem(CLOSE_BEHAVIOR_KEY)
  if (rawValue === 'exit' || rawValue === 'tray') return rawValue

  const legacyValue = window.localStorage.getItem(LEGACY_CLOSE_TO_TRAY_KEY)
  if (legacyValue === 'true') return 'tray'
  if (legacyValue === 'false') return 'exit'

  return DEFAULT_CLOSE_BEHAVIOR
}

export function setCloseBehavior(value: CloseBehavior) {
  window.localStorage.setItem(CLOSE_BEHAVIOR_KEY, value)
  window.localStorage.removeItem(LEGACY_CLOSE_TO_TRAY_KEY)
}

// 同步到 Tauri 后端，让关闭行为立即生效。
export async function syncCloseBehavior(value = getCloseBehavior()) {
  try {
    await invoke('set_close_to_tray_enabled', { enabled: value === 'tray' })
  } catch (error) {
    console.error('Failed to sync close behavior setting', error)
  }
}
