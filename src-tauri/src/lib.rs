use tauri::Manager;

use crate::cache::build_cache;

mod cache;
mod fonts;
mod logging;
mod menu;
mod server;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default();

    builder
        .setup(|app| {
            let path = app.path();

            let log_dir = path.app_log_dir()?;

            logging::setup(&log_dir)?;

            let static_dir = path.app_cache_dir()?.join("static");

            tracing::info!("Static directory: {:?}", static_dir);

            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            menu::setup(app.handle())?;

            tauri::async_runtime::spawn(async move {
                if let Err(e) = build_cache(&static_dir) {
                    tracing::error!("Error building cache on startup: {}", e);
                }

                server::start(&static_dir).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
