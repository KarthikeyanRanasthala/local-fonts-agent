use crate::fonts;

pub fn build_cache(dir: &std::path::PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let fonts_meta = fonts::get_fonts_meta();
    let fonts_preview = fonts::get_fonts_preview(fonts_meta.clone());

    // Create static directory if it doesn't exist
    std::fs::create_dir_all(dir)?;

    let fonts_meta_path = dir.join("fonts-meta.json");
    let fonts_meta_json = serde_json::to_string_pretty(&fonts_meta)?;
    std::fs::write(fonts_meta_path, fonts_meta_json)?;

    let fonts_preview_path = dir.join("fonts-preview.json");
    let fonts_preview_json = serde_json::to_string_pretty(&fonts_preview)?;
    std::fs::write(fonts_preview_path, fonts_preview_json)?;

    Ok(())
}
