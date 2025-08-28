use serde::Deserialize;
use serde::Serialize;
use std::sync::Mutex;
use tauri::Manager;
use tauri::State;
use tauri_plugin_http::reqwest;

#[derive(Default)]
struct AppState {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TrackData {
    title: String,
    artist: String,
    album: String,
}

#[tauri::command]
fn set_url(state: State<'_, Mutex<AppState>>, new_url: String) -> String {
    let mut state = state.lock().unwrap();
    state.url = new_url.clone();
    state.url.clone()
}

#[tauri::command]
fn get_url(state: State<'_, Mutex<AppState>>) -> String {
    let state = state.lock().unwrap();
    state.url.clone()
}

#[tauri::command]
async fn get_track(state: State<'_, Mutex<AppState>>) -> Result<TrackData, String> {
    let url = {
        let state = state.lock().unwrap();
        state.url.clone()
    };

    let res = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let body = res.text().await.map_err(|e| e.to_string())?;
        let track: TrackData = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        Ok(track)
    } else {
        Err(format!("Failed with status: {}", res.status()))
    }
}

#[tauri::command]
async fn test_url(state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    match get_track(state).await {
        Ok(_) => Ok("URL is reachable".into()),
        Err(e) => Err(e),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            set_url, get_url, get_track, test_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
