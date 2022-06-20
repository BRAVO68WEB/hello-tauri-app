#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn greet() -> String {
  format!(" {}!",whoami::username())
}

#[tauri::command]
fn systeminfo() -> String {
  format!("Running {} on {}",whoami::platform(), whoami::distro())
  
}

fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet,systeminfo])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
