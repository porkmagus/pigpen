use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager, Runtime,
};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
use window_vibrancy::{apply_acrylic, apply_mica};

fn toggle_hud<R: Runtime>(app: &tauri::AppHandle<R>) {
    if let Some(window) = app.get_webview_window("hud") {
        let is_visible = window.is_visible().unwrap_or(false);
        if is_visible {
            window.hide().unwrap();
        } else {
            window.show().unwrap();
            window.set_focus().unwrap();
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
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

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
                .on_shortcut(ctrl_space, |app, _shortcut, _event| {
                    toggle_hud(app);
                })?;

            // Apply Vibrancy
            if let Some(window) = app.get_webview_window("dashboard") {
                apply_vibrancy(&window);
            }
            if let Some(window) = app.get_webview_window("hud") {
                apply_vibrancy(&window);
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
