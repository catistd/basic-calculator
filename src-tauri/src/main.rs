// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use equation_parse::parse_equation;

pub mod equation_parse;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![calculate])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn calculate(equation: String) -> String {
    match parse_equation(equation) {
      Ok(num) => num.to_string(),
      Err(e) => e
    }
}