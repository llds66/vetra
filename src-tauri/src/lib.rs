mod commands;
mod desktop;
mod state;

use commands::set_close_to_tray_enabled;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
/// 启动 Tauri 应用并初始化桌面端能力。
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::new())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_close_to_tray_enabled])
        .setup(|app| {
            desktop::window::setup_main_window(app)?;
            desktop::tray::setup_tray(app)?;
            Ok(())
        })
        .on_window_event(desktop::window::handle_window_event)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
