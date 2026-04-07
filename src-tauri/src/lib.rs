use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};
use window_vibrancy::apply_mica;

const SHOW_WINDOW_MENU_ID: &str = "show_window";
const QUIT_APP_MENU_ID: &str = "quit_app";

struct AppState {
    is_quitting: AtomicBool,
    close_to_tray_enabled: AtomicBool,
}

fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

#[tauri::command]
fn set_close_to_tray_enabled(app: tauri::AppHandle, enabled: bool) {
    app.state::<AppState>()
        .close_to_tray_enabled
        .store(enabled, Ordering::Relaxed);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState {
            is_quitting: AtomicBool::new(false),
            close_to_tray_enabled: AtomicBool::new(false),
        })
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![set_close_to_tray_enabled])
        .setup(|app| {
            let window = app
                .get_webview_window("main")
                .expect("main window not found");
            let _ = apply_mica(&window, None);

            let show_window =
                MenuItem::with_id(app, SHOW_WINDOW_MENU_ID, "显示主窗口", true, None::<&str>)?;
            let quit_app = MenuItem::with_id(app, QUIT_APP_MENU_ID, "退出", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_window, &quit_app])?;

            let mut tray_builder = TrayIconBuilder::with_id("main-tray")
                .menu(&tray_menu)
                .tooltip("Vetra")
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    SHOW_WINDOW_MENU_ID => show_main_window(app),
                    QUIT_APP_MENU_ID => {
                        app.state::<AppState>()
                            .is_quitting
                            .store(true, Ordering::Relaxed);
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        show_main_window(tray.app_handle());
                    }
                });

            if let Some(icon) = app.default_window_icon().cloned() {
                tray_builder = tray_builder.icon(icon);
            }

            tray_builder.build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event {
                let state = window.state::<AppState>();
                if state.is_quitting.load(Ordering::Relaxed) {
                    return;
                }

                if !state.close_to_tray_enabled.load(Ordering::Relaxed) {
                    return;
                }

                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
