mod autostart;
mod code_feeder;
mod get_by_bilibili;
mod tolbox_window;
mod translate;
mod quick_open;

use tauri::Manager;
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_sql::{Migration, MigrationKind};
use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build());
    #[cfg(desktop)]
    {
        // 配置程序以单例方式启动
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // if中的这3行缺一不可
            if let Some(window) = app.get_webview_window("main") {
                // 如果窗口最小化了，先恢复
                let _ = window.unminimize();
                // 显示window窗口
                let _ = window.show();
                // 然后将窗口激活、置顶
                let _ = window.set_focus();
            }
        }));
    }

    builder
        .plugin(
            tauri_plugin_sql::Builder::default()
            .add_migrations(
                "sqlite:tolbox.db", 
                vec![
                    Migration{
                        version: 1,
                        description: "create settings table",
                        sql: "CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT)",
                        kind: MigrationKind::Up,
                    }
                ],
            )
            .build()
        )
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            None,
        ))
        .plugin(tauri_plugin_persisted_scope::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            autostart::change_autostart,
            code_feeder::process_folder_command,
            translate::translate_word,
            translate::translate_sentence,
            get_by_bilibili::download_bilibili,
            tolbox_window::create_main_window,
            tolbox_window::create_login_window,
            quick_open::quick_open
        ])
        .setup(|app| {
            // 登录窗口
            tauri::WebviewWindowBuilder::new(app, "login", tauri::WebviewUrl::App("/login".into()))
                .title("tolbox")
                .inner_size(420.0, 330.0)
                .center()
                .resizable(false)
                .maximizable(false)
                //上方的菜单栏
                .decorations(false)
                .transparent(true)
                .fullscreen(false)
                .build()?;

            //系统托盘
            let menu_item_separator = tauri::menu::PredefinedMenuItem::separator(app)?;
            let menu_item_quick_open = tauri::menu::MenuItem::with_id(app, "quickOpen", "快速启动", true, None::<&str>)?;
            let menu_item_quit = tauri::menu::MenuItem::with_id(app, "quit", "关闭", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&menu_item_quick_open, &menu_item_separator, &menu_item_quit])?;

            tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                // .menu(&tauri::menu::Menu::with_items(app, &menu_items)?)
                .menu(&menu)

                .show_menu_on_left_click(false)
                .on_menu_event(|app,event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "quickOpen" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize(); // 解除最小化
                            let _ = window.show();      // 确保窗口显示
                            let _ = window.set_focus(); // 聚焦窗口
                            // 通知前端进行路由跳转
                            let _ = window.emit("navigate-to", "quickOpen");
                        } else if let Some(window) = app.get_webview_window("login") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray,event| match event {
                    tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize(); // 解除最小化
                            let _ = window.show();      // 确保窗口显示
                            let _ = window.set_focus(); // 聚焦窗口
                        }
                        else if let Some(window) = app.get_webview_window("login") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
