#[tauri::command]
// ！！！！！剧阴：不加async 新窗口有问题！
pub async fn create_main_window(app: tauri::AppHandle) -> Result<(), String> {
    tauri::WebviewWindowBuilder::new(
        &app,
        "main",
        tauri::WebviewUrl::App("/mainLayout/home".into()),
    )
    .title("tolbox")
    .inner_size(1070.0, 630.0)
    .center()
    .resizable(false)
    .maximizable(false)
    .decorations(false)
    .transparent(true)
    .fullscreen(false)
    .build()
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn create_login_window(app: tauri::AppHandle) -> Result<(), String> {
    tauri::WebviewWindowBuilder::new(&app, "login", tauri::WebviewUrl::App("/login".into()))
        .title("tolbox")
        .inner_size(420.0, 330.0)
        .center()
        .resizable(false)
        .maximizable(false)
        .decorations(false)
        .transparent(true)
        .fullscreen(false)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}
