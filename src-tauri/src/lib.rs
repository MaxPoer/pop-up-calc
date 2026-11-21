mod commands;

use tauri::{Manager, PhysicalPosition};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let win = app.get_webview_window("main").unwrap();
            place(&win)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![commands::handle_text])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

const LABEL_H: f64 = 60.0;
const LABEL_GAP: f64 = 8.0;
const ROW_H: f64 = 60.0;

fn place(win: &tauri::WebviewWindow) -> tauri::Result<()> {
    if let Some(mon) = win.current_monitor()? {
        let m_pos = mon.position();
        let m_size = mon.size();
        let w = win.outer_size()?;
        let scale = win.scale_factor()?;

        // distance from the window's top edge to the middle of the input
        let offset = ((LABEL_H + LABEL_GAP + ROW_H / 2.0) * scale).round() as i32;

        let x = m_pos.x + (m_size.width as i32 - w.width as i32) / 2;
        let y = m_pos.y + m_size.height as i32 / 2 - offset;

        win.set_position(PhysicalPosition::new(x, y))?;
    }
    Ok(())
}