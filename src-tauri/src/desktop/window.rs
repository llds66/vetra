use std::sync::atomic::Ordering;

use tauri::{App, AppHandle, Manager, Window, WindowEvent};
use window_vibrancy::apply_mica;

use crate::state::AppState;

#[cfg(windows)]
const INACTIVE_TITLEBAR_COLOR: u32 = 0x002A1D18;

/// 显示并激活主窗口。
pub fn show_main_window(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.unminimize();
        let _ = window.set_focus();
    }
}

/// 初始化主窗口的 Mica 与标题栏效果。
pub fn setup_main_window(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app
        .get_webview_window("main")
        .expect("main window not found");
    let _ = apply_mica(&window, None);
    sync_titlebar_backdrop(&window, true);
    Ok(())
}

/// 处理主窗口的关闭与焦点事件。
pub fn handle_window_event(window: &Window, event: &WindowEvent) {
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

    if let WindowEvent::Focused(focused) = event {
        sync_titlebar_backdrop(window, *focused);
    }
}

#[cfg(windows)]
/// 根据窗口焦点状态同步标题栏和边框的系统背景色。
fn sync_titlebar_backdrop(window: &impl raw_window_handle::HasWindowHandle, focused: bool) {
    let Ok(handle) = window.window_handle() else {
        return;
    };

    let raw_window_handle::RawWindowHandle::Win32(handle) = handle.as_raw() else {
        return;
    };

    let hwnd = handle.hwnd.get() as windows_sys::Win32::Foundation::HWND;
    let color = if focused {
        windows_sys::Win32::Graphics::Dwm::DWMWA_COLOR_DEFAULT
    } else {
        INACTIVE_TITLEBAR_COLOR
    };

    unsafe {
        let _ = windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
            hwnd,
            windows_sys::Win32::Graphics::Dwm::DWMWA_CAPTION_COLOR as u32,
            &color as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );

        let _ = windows_sys::Win32::Graphics::Dwm::DwmSetWindowAttribute(
            hwnd,
            windows_sys::Win32::Graphics::Dwm::DWMWA_BORDER_COLOR as u32,
            &color as *const _ as _,
            std::mem::size_of::<u32>() as u32,
        );
    }
}

#[cfg(not(windows))]
/// 非 Windows 平台无需同步标题栏背景效果。
fn sync_titlebar_backdrop<T>(_window: &T, _focused: bool) {}
