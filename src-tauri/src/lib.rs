use std::{
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use tauri::{AppHandle, Emitter, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn start_precise_timestamp_emitter(
    app: AppHandle,
    state: tauri::State<'_, Mutex<AppData>>,
) -> Result<(), String> {
    let mut state = state.lock().unwrap();
    if state.emitter_started {
        return Err("Emitter already started".to_string());
    }
    let app_handle = Arc::new(app);
    tokio::spawn(async move {
        loop {
            emit_precise_timestamp(app_handle.clone());
            tokio::time::sleep(std::time::Duration::from_nanos(1)).await;
        }
    });
    state.emitter_started = true;
    Ok(())
}

fn emit_precise_timestamp(app: Arc<AppHandle>) {
    let epoch_nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    // emit to frontend
    app.emit_to("main", "precise_timestamp", epoch_nanos)
        .unwrap();
}

#[derive(Default)]
struct AppData {
    emitter_started: bool,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData {
                emitter_started: false,
            }));
            Ok(())
        })
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![start_precise_timestamp_emitter])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
