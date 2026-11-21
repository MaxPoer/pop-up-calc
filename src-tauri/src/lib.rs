mod commands;

use tauri::{LogicalSize, Manager, PhysicalPosition};
use tauri_plugin_global_shortcut::{Code, Modifiers, Shortcut, ShortcutState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_shortcut(Shortcut::new(
                    Some(Modifiers::CONTROL | Modifiers::ALT),
                    Code::KeyN,
                ))
                .expect("Couldn't grab Hotkey!")
                .with_handler(|app, _shortcut, event| {
                    if event.state != ShortcutState::Pressed {
                        return; // trigger only on press, not release
                    }
                    if let Some(win) = app.get_webview_window("main") {
                        if !win.is_visible().unwrap_or(false) {
                            place(&win).expect("Couldn't reposition popup");
                            win.show().expect("Couldn't show popup");
                            win.set_focus().expect("Couldn't focus popup");
                        }
                    }
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::on_edit,
            commands::on_enter,
            commands::on_app_close,
            hide_popup
        ])
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            place(&win)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

const WINDOW_WIDTH: f64 = 1000.0;
const MIN_HEIGHT: f64 = 90.0;
const LABEL_GAP: f64 = 8.0;
const ROW_HEIGHT: f64 = 60.0;

fn adjust_height(window: &tauri::WebviewWindow, lines: usize) -> tauri::Result<()> {
    window.set_size(LogicalSize::new(
        WINDOW_WIDTH,
        MIN_HEIGHT + ROW_HEIGHT * lines as f64,
    ))
}

fn place(win: &tauri::WebviewWindow) -> tauri::Result<()> {
    if let Some(mon) = win.current_monitor()? {
        let m_pos = mon.position();
        let m_size = mon.size();
        let w = win.outer_size()?;
        let scale = win.scale_factor()?;

        // distance from the window's top edge to the middle of the input
        let offset = ((ROW_HEIGHT * 1.5 + LABEL_GAP) * scale).round() as i32;

        let x = m_pos.x + (m_size.width as i32 - w.width as i32) / 2;
        let y = m_pos.y + m_size.height as i32 / 2 - offset;

        win.set_position(PhysicalPosition::new(x, y))?;
    }
    Ok(())
}

#[tauri::command]
fn hide_popup(window: tauri::WebviewWindow) -> Result<(), String> {
    window.hide().map_err(|e| e.to_string())
}
