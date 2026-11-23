use tauri::async_runtime::Mutex;
use tauri::State;
use crate::commands::AppState;

pub async fn handle_calculators(app_state: &State<'_, Mutex<AppState>>, user_input: String) -> Vec<String> {
    // this function is handed the user's input and runs it against all scanners/parsers
    // then, the return values of the parsers are collected and displayed on dynamically adjusted window size
    let mut results: Vec<String> = Vec::new();

    //TODO call scanners/parsers

    results.push(user_input.clone());

    // to set an action when hitting the enter key
    //app_state.lock().await.enter_action = "test".to_string();

    results
}