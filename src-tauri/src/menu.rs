use tauri::menu::{CheckMenuItem, MenuItem};
use tauri_plugin_autostart::ManagerExt;

pub fn setup(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let name = app.package_info().name.clone();
    let version = format!("{} v{}", name, app.package_info().version.to_string());

    let autostart = app.autolaunch();
    let is_enabled = autostart.is_enabled().unwrap_or(false);

    let menu = tauri::menu::Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "version", &version, false, None::<&str>)?,
            &CheckMenuItem::with_id(
                app,
                "autostart",
                "Start on Login",
                true,
                is_enabled,
                None::<&str>,
            )?,
            &MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?,
        ],
    )?;

    tauri::tray::TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip(name)
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "autostart" => {
                let autostart = app.autolaunch();
                let is_enabled = autostart.is_enabled().unwrap_or(false);

                if is_enabled {
                    if let Err(e) = autostart.disable() {
                        tracing::error!("Failed to disable autostart: {}", e);
                    } else {
                        tracing::info!("Autostart disabled");
                    }
                } else {
                    if let Err(e) = autostart.enable() {
                        tracing::error!("Failed to enable autostart: {}", e);
                    } else {
                        tracing::info!("Autostart enabled");
                    }
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
