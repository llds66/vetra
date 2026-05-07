use std::sync::atomic::Ordering;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    App, Manager,
};

use crate::state::AppState;

use super::window::show_main_window;

const SHOW_WINDOW_MENU_ID: &str = "show_window";
const QUIT_APP_MENU_ID: &str = "quit_app";

/// 初始化系统托盘及其菜单行为。
pub fn setup_tray(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
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
}
