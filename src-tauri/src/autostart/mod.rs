use tauri::{AppHandle, Runtime};
use tauri_plugin_autostart::ManagerExt;

/// 提供给前端调用：切换开机自启
#[tauri::command]
pub async fn change_autostart<R: Runtime>(app: AppHandle<R>, open: bool) -> Result<(), String> {
    let mgr = app.autolaunch();
    if open {
        mgr.enable().map_err(|e| e.to_string())
    } else {
        mgr.disable().map_err(|e| e.to_string())
    }
}
