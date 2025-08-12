use tauri::{Manager};
use std::path::{PathBuf,Path};
use std::process::Command;


#[tauri::command]
pub async fn quick_open(path: Option<String>, app: Option<String>) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(move || {
        if let Some(app_path) = app {
            let mut cmd = Command::new(&app_path);

            if let Some(target) = &path {
                // 对路径参数进行特殊处理（Windows需要）
                #[cfg(target_os = "windows")]
                {
                    // 使用原始字符串字面量避免转义问题
                    cmd.arg(target.replace('/', "\\"));
                }
                #[cfg(not(target_os = "windows"))]
                {
                    cmd.arg(target);
                }
            }

            cmd.spawn()
                .map_err(|e| format!("启动应用失败: {} (路径: {:?})", e, app_path))?;
        } else if let Some(target) = path {
            // 使用 opener 库处理路径（它会自动处理空格）
            opener::open(target)
                .map_err(|e| format!("打开失败: {}", e))?;
        }
        Ok::<(), String>(())
    })
        .await
        .map_err(|e| format!("异步任务失败: {}", e))?
}
