use tauri::{AppHandle, Emitter};
use serde::{Serialize, Deserialize};
use tauri::Listener;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct DownloadStarted<'a>{
    url: &'a str,
    download_id: usize,
    content_length: usize,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct DownloadProgress{
    download_id: usize,
    chunk_length: usize,
}


#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct DownloadFinished{
    download_id: usize,
}


#[tauri::command]
fn download(app:AppHandle, url:String) -> String {
    let content_length = 1000;
    let download_id = 1;
    
    app.emit("download-started", DownloadStarted {
        url: &url,
        download_id,
        content_length,
    }).unwrap();
    
    for chunk_length in [1, 15, 50, 80, 100] {
        app.emit("download-progress", DownloadProgress {
            download_id,
            chunk_length,
        }).unwrap();
    }
    app.emit("download-completed", DownloadFinished { download_id }).unwrap();
    
    url.to_string()
}

// #[tauri::command]
// fn download(url: &str) -> String {
//     url.to_string()
// }

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.listen("download-progress", |event| {
                if let Ok(payload) = serde_json::from_str::<DownloadProgress>(&event.payload()) {
                    println!("downloading {}", payload.chunk_length)
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
