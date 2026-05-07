use std::sync::atomic::Ordering;

use tauri::{AppHandle, Manager};

use crate::state::AppState;

#[tauri::command]
/// 更新关闭窗口时是否最小化到托盘的开关。
pub fn set_close_to_tray_enabled(app: AppHandle, enabled: bool) {
    app.state::<AppState>()
        .close_to_tray_enabled
        .store(enabled, Ordering::Relaxed);
}
