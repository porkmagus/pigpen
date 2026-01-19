mod search;
use search::{SearchEngine, SearchResult};
use std::sync::Mutex;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, Runtime, State,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};
use window_vibrancy::{apply_acrylic, apply_mica};

struct DbState(Mutex<Option<SearchEngine>>);

#[tauri::command]
fn search_vault(state: State<'_, DbState>, query: String) -> Result<Vec<SearchResult>, String> {
    let db_lock = state.0.lock().unwrap();
    if let Some(engine) = db_lock.as_ref() {
        engine.search(&query).map_err(|e| e.to_string())
    } else {
        Err("Search engine not initialized".to_string())
    }
}

#[tauri::command]
fn index_vault(state: State<'_, DbState>, path: String) -> Result<usize, String> {
    let db_lock = state.0.lock().unwrap();
    if let Some(engine) = db_lock.as_ref() {
        engine.index_vault(path).map_err(|e| e.to_string())
    } else {
        Err("Search engine not initialized".to_string())
    }
}

fn toggle_hud<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("hud") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            let _ = window.hide();
        } else {
            let _ = window.show();
            let _ = window.set_focus();
        }
    }
}

fn apply_vibrancy<R: Runtime>(window: &tauri::WebviewWindow<R>) {
    #[cfg(target_os = "macos")]
    let _ = window_vibrancy::apply_vibrancy(
        window,
        NSVisualEffectMaterial::AppearanceBased,
        None,
        None,
    );

    #[cfg(target_os = "windows")]
    {
        let _ = apply_mica(window, None);
        let _ = apply_acrylic(window, Some((0, 0, 0, 0)));
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DbState(Mutex::new(None)))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize DB
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("failed to get app data dir");
            std::fs::create_dir_all(&app_data_dir).expect("failed to create app data dir");
            let db_path = app_data_dir.join("pigpen.db");

            let engine = SearchEngine::new(db_path).expect("failed to init search engine");
            *app.state::<DbState>().0.lock().unwrap() = Some(engine);

            // Create Tray Menu
            let quit_i = MenuItem::with_id(app, "quit", "Quit Pigpen", true, None::<&str>)?;
            let show_hud_i = MenuItem::with_id(app, "show_hud", "Show HUD", true, None::<&str>)?;
            let show_dash_i =
                MenuItem::with_id(app, "show_dash", "Open Dashboard", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_hud_i, &show_dash_i, &quit_i])?;

            // Build Tray
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        app.exit(0);
                    }
                    "show_hud" => {
                        toggle_hud(app);
                    }
                    "show_dash" => {
                        if let Some(window) = app.get_webview_window("dashboard") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        toggle_hud(app);
                    }
                })
                .build(app)?;

            // Register Hotkey: Alt+Space
            let ctrl_space = Shortcut::new(
                Some(tauri_plugin_global_shortcut::Modifiers::ALT),
                tauri_plugin_global_shortcut::Code::Space,
            );

            app.global_shortcut()
                .on_shortcut(ctrl_space, |app, _shortcut, event| {
                    if event.state() == ShortcutState::Pressed {
                        toggle_hud(app);
                    }
                })?;

            // Apply Vibrancy
            if let Some(window) = app.get_webview_window("dashboard") {
                apply_vibrancy(&window);
            }
            if let Some(window) = app.get_webview_window("hud") {
                apply_vibrancy(&window);

                // Hide HUD on blur (losing focus)
                let w = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = w.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![search_vault, index_vault])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
