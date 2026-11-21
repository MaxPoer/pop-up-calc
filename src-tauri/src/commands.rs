use crate::adjust_height;

#[tauri::command]
pub fn on_edit(window: tauri::WebviewWindow, user_input: String) -> Vec<String> {
    // this function is handed the user's input and runs it against all scanners/parsers
    // then, the return values of the parsers are collected and displayed on dynamically adjusted window size

    let mut results: Vec<String> = Vec::new();

    // test logic
    let ans: &str = match user_input.as_str() {
        "1+1" => "2",
        "69" => "nice",
        _ => "",
    };
    if ans != "" {
        results.push(ans.to_string());
    }

    // Showing 6 results is reasonable, beyond that space is limited
    results.truncate(6);
    // make space for the lines
    adjust_height(&window, results.len()).expect("Failed to adjust height");
    results
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
