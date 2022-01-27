#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use test_rust::test_rust;

fn main() {
  // test running a function from library
  test_rust();

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
