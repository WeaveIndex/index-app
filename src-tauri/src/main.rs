#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/*#[tauri::command] //broken state
fn launch_diagnoser() {
    let diagnoser_dir: &str = std::env::var("HOME") + ".weave/WeaveDiagnose.jar";

    println!("Spawn and clean shutdown");
    let _child = Command::new("java")
        .arg("-jar {}", diagnoser_dir)
        .spawn()
        .unwrap();
}*/

fn main() {
    tauri::Builder::default()
        //.invoke_handler(tauri::generate_handler![launch_diagnoser])
        .plugin(tauri_plugin_fs_watch::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
