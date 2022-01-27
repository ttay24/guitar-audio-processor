#[tauri::command]
pub fn test_command() -> String {
    println!("this is a test command");
    "this is from rust".into()
}