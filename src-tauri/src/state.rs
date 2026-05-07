use std::sync::atomic::AtomicBool;

/// 保存桌面端窗口与退出行为相关状态。
pub struct AppState {
    pub is_quitting: AtomicBool,
    pub close_to_tray_enabled: AtomicBool,
}

impl AppState {
    /// 创建应用共享状态。
    pub fn new() -> Self {
        Self {
            is_quitting: AtomicBool::new(false),
            close_to_tray_enabled: AtomicBool::new(false),
        }
    }
}
