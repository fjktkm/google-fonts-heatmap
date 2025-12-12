use std::fs;
use std::path::{Path, PathBuf};

use skrifa::instance::{LocationRef, Size};
use skrifa::outline::DrawSettings;
use skrifa::raw::TableProvider;
use skrifa::{FontRef, MetadataProvider};

use crate::error::CoordinateError;
use crate::pens::{CommandCountPen, PointCollector};

fn collect_points_from_font(path: &Path) -> Result<Vec<[f32; 2]>, CoordinateError> {
    let data = fs::read(path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
    let font = FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;

    let upem = font
        .head()
        .ok()
        .map(|head| head.units_per_em())
        .filter(|value| *value > 0)
        .unwrap_or(1000) as f32;
    let scale = 1.0 / upem;

    let outlines = font.outline_glyphs();
    let mut collector = PointCollector::new(scale);
    for (_, glyph) in outlines.iter() {
        glyph
            .draw(
                DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
                &mut collector,
            )
            .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
    }
    Ok(collector.into_points())
}

pub fn outline_coordinates(font_paths: Vec<PathBuf>) -> Result<Vec<[f32; 2]>, CoordinateError> {
    let mut accumulated = Vec::new();
    for path in font_paths {
        let mut per_font = collect_points_from_font(&path)?;
        accumulated.append(&mut per_font);
    }
    Ok(accumulated)
}

pub fn glyph_command_counts(font_paths: Vec<PathBuf>) -> Result<Vec<u32>, CoordinateError> {
    let mut counts = Vec::new();
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let outlines = font.outline_glyphs();
        for (_, glyph) in outlines.iter() {
            let mut pen = CommandCountPen::default();
            glyph
                .draw(
                    DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
                    &mut pen,
                )
                .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
            counts.push(pen.into_count());
        }
    }
    Ok(counts)
}
