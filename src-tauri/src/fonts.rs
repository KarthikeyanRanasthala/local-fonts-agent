use std::{collections::HashMap, error::Error, fs};

use font_kit::{error::FontLoadingError, handle::Handle, outline::OutlineSink};
use pathfinder_geometry::{line_segment::LineSegment2F, vector::Vector2F};

#[derive(Clone, serde::Serialize)]
pub struct FontMeta {
    pub family: String,
    pub full_name: Option<String>,
    pub postscript_name: String,
    pub is_monospace: bool,
    pub weight: f32,
    pub style: String,
    pub stretch: f32,
}

pub type FontsMeta = Vec<FontMeta>;

pub fn get_fonts_meta() -> FontsMeta {
    let source = font_kit::source::SystemSource::new();
    let families = source.all_families().unwrap_or_default();

    let mut fonts_meta = Vec::new();

    for family in families {
        if let Ok(family_handle) = source.select_family_by_name(&family) {
            if let Some(font_handle) = family_handle.fonts().first() {
                if let Ok(font) = font_handle.load() {
                    let properties = font.properties();
                    let postscript_name = font.postscript_name();

                    if let Some(postscript_name) = postscript_name {
                        fonts_meta.push(FontMeta {
                            family: font.family_name(),
                            full_name: Some(font.full_name()),
                            postscript_name,
                            is_monospace: font.is_monospace(),
                            weight: properties.weight.0,
                            style: format!("{:?}", properties.style),
                            stretch: properties.stretch.0,
                        });
                    }
                }
            }
        }
    }

    fonts_meta
}

struct SvgPathBuilder {
    path_data: String,
    offset: Vector2F,
    scale: f32,
}

impl SvgPathBuilder {
    fn new(offset: Vector2F, scale: f32) -> Self {
        Self {
            path_data: String::new(),
            offset,
            scale,
        }
    }
}

impl OutlineSink for SvgPathBuilder {
    fn move_to(&mut self, to: Vector2F) {
        // Scale from font units to pixels and add horizontal offset
        let point = to * self.scale + self.offset;
        // Negate Y to convert from font coordinates (Y-up) to SVG coordinates (Y-down)
        self.path_data
            .push_str(&format!("M {} {} ", point.x(), -point.y()));
    }

    fn line_to(&mut self, to: Vector2F) {
        let point = to * self.scale + self.offset;

        self.path_data
            .push_str(&format!("L {} {} ", point.x(), -point.y()));
    }

    fn quadratic_curve_to(&mut self, ctrl: Vector2F, to: Vector2F) {
        let ctrl_point = ctrl * self.scale + self.offset;
        let to_point = to * self.scale + self.offset;

        self.path_data.push_str(&format!(
            "Q {} {} {} {} ",
            ctrl_point.x(),
            -ctrl_point.y(),
            to_point.x(),
            -to_point.y()
        ));
    }

    fn cubic_curve_to(&mut self, ctrl: LineSegment2F, to: Vector2F) {
        let ctrl1_point = ctrl.from() * self.scale + self.offset;
        let ctrl2_point = ctrl.to() * self.scale + self.offset;
        let to_point = to * self.scale + self.offset;

        self.path_data.push_str(&format!(
            "C {} {} {} {} {} {} ",
            ctrl1_point.x(),
            -ctrl1_point.y(),
            ctrl2_point.x(),
            -ctrl2_point.y(),
            to_point.x(),
            -to_point.y()
        ));
    }

    fn close(&mut self) {
        self.path_data.push_str("Z ");
    }
}

fn generate_preview(font_handle: &Handle, text: &String) -> Result<String, FontLoadingError> {
    let font = font_handle.load()?;

    let metrics = font.metrics();
    let units_per_em = metrics.units_per_em as f32;
    let font_size = 12.0;
    let scale = font_size / units_per_em;

    // Use font metrics to get proper bounds
    let ascent = metrics.ascent * scale;
    let descent = metrics.descent * scale;

    let mut cursor_x = 0.0;
    let mut path_data = String::new();

    for char in text.chars() {
        if let Some(glyph_id) = font.glyph_for_char(char) {
            let offset = Vector2F::new(cursor_x, 0.0);
            let mut sink = SvgPathBuilder::new(offset, scale);

            if font
                .outline(glyph_id, font_kit::hinting::HintingOptions::None, &mut sink)
                .is_ok()
            {
                path_data.push_str(&sink.path_data);
            }

            // Advance cursor
            let advance = font.advance(glyph_id).unwrap_or_default();
            cursor_x += advance.x() * scale;
        }
    }

    // Font metrics: ascent is positive (above baseline), descent is negative (below baseline)
    // Since we negate Y coordinates in the path, descent becomes positive in SVG space
    let padding = 2.0;
    let width = cursor_x + padding * 2.0;
    let height = ascent - descent + padding * 2.0; // descent is negative, so this is ascent + |descent|
    let view_x = -padding;
    // In SVG space (Y-down), we start from negative ascent (top of text)
    let view_y = -ascent - padding;

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{} {} {} {}" width="{}" height="{}"><path d="{}" fill="currentColor"/></svg>"#,
        view_x,
        view_y,
        width,
        height,
        width,
        height,
        path_data.trim()
    );

    Ok(svg)
}

pub type FontsPreview = HashMap<String, String>;

pub fn get_fonts_preview(fonts_meta: FontsMeta) -> FontsPreview {
    let source = font_kit::source::SystemSource::new();

    let mut fonts_preview = HashMap::new();

    for font_meta in fonts_meta {
        if let Ok(font_handle) = source.select_by_postscript_name(&font_meta.postscript_name) {
            if let Ok(svg_string) = generate_preview(&font_handle, &font_meta.family) {
                fonts_preview.insert(font_meta.postscript_name.clone(), svg_string);
            }
        }
    }

    fonts_preview
}

pub fn get_font(postscript_name: &String) -> Result<(Vec<u8>, String), Box<dyn Error>> {
    let source = font_kit::source::SystemSource::new();

    let font_handle = source.select_by_postscript_name(&postscript_name)?;

    match &font_handle {
        Handle::Path { path, .. } => {
            let font_data = fs::read(path)?;

            let content_type = infer::get(&font_data)
                .map(|kind| kind.mime_type())
                .unwrap_or("application/octet-stream");

            Ok((font_data, content_type.to_string()))
        }
        Handle::Memory { bytes, .. } => {
            let content_type = infer::get(bytes)
                .map(|kind| kind.mime_type())
                .unwrap_or("application/octet-stream");

            Ok((bytes.to_vec(), content_type.to_string()))
        }
    }
}
