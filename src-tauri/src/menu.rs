use tauri::menu::MenuItem;

pub fn setup(app: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let name = app.package_info().name.clone();
    let version = format!("{} v{}", name, app.package_info().version.to_string());

    let menu = tauri::menu::Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "version", &version, false, None::<&str>)?,
            &MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?,
        ],
    )?;

    tauri::tray::TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip(name)
        .show_menu_on_left_click(true)
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .build(app)?;

    Ok(())
}
