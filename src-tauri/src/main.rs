// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(handle: tauri::AppHandle, _name: &str) -> String {
    let resource_path = handle
        .path_resolver()
        .resolve_resource("resources/audio.wav")
        .expect("failed to resolve resource");
    use soloud::*;
    let sl = Soloud::default().unwrap();
    let mut wav = audio::Wav::default();
    wav.load(resource_path).expect("Error al cargar archivo");
    sl.play(&wav);
    while sl.voice_count() > 0 {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    format!("")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
