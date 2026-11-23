use crate::adjust_height;
use tauri::{Error, State};
use tauri::async_runtime::Mutex;
use crate::calculator::handle_calculators;

#[derive(Default)]
pub(crate) struct AppState {
    pub(crate) enter_action: String
}

#[tauri::command]
pub async fn on_edit(
    app_state: State<'_, Mutex<AppState>>,
    window: tauri::WebviewWindow,
    user_input: String)
    -> Result<Vec<String>, Error> {
    
    let mut results = handle_calculators(&app_state, user_input).await;

    // Showing 6 results is reasonable, beyond that space is limited
    results.truncate(6);
    // make space for the lines
    adjust_height(&window, results.len()).expect("Failed to adjust height");
    Ok(results)
}

#[tauri::command]
pub fn on_enter() {
    // This function is for following weblinks for suitable results
    //TODO
}

#[tauri::command]
pub fn on_app_close() {
    // save input
    //TODO
}
