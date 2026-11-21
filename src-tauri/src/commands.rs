#[tauri::command]
pub fn handle_text(user_input: String) -> Vec<String> {
    // this function is handed the user's input and runs it against all scanners/parsers
    // then, the return values of the parsers is collected and displayed on dynamically adjusted window size
    vec!["moin".to_string()]
}

pub fn handle_button() {
    
}