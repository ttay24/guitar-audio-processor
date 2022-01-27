#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use test_rust::test_rust;
mod vst_host;
use vst_host::test;

fn main() {
  // test running a function from library
  test_rust();

  //vst_host::test_vst();

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![vst_host::test::test_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
