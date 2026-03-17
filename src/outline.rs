use std::fs;
use std::path::{Path, PathBuf};

use skrifa::instance::{Location, LocationRef, Size};
use skrifa::outline::{DrawSettings, OutlineGlyphFormat};
use skrifa::raw::TableProvider;
use skrifa::{FontRef, MetadataProvider};

use crate::error::CoordinateError;
use crate::pens::{CommandBreakdownPen, CommandCountPen, CommandStatsPen, PointCollector};

struct Rng(u64);

impl Rng {
    fn new() -> Self {
        Self(0x517cc1b727220a95)
    }

    fn keep(&mut self, rate: f64) -> bool {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 * (1.0 / (1u64 << 53) as f64) < rate
    }
}

fn instance_locations(font: &FontRef<'_>) -> Vec<Option<Location>> {
    let instances = font.named_instances();
    if instances.is_empty() {
        vec![None]
    } else {
        instances.iter().map(|i| Some(i.location())).collect()
    }
}

fn collect_points_from_font(
    path: &Path,
    font: &FontRef<'_>,
    location: LocationRef<'_>,
    sample_rate: f64,
    rng: &mut Rng,
) -> Result<Vec<[f32; 2]>, CoordinateError> {
    let upem = font
        .head()
        .ok()
        .map(|head| head.units_per_em())
        .filter(|value| *value > 0)
        .unwrap_or(1000) as f32;
    let scale = 1.0 / upem;

    let outlines = font.outline_glyphs();
    let mut accumulated = Vec::new();
    let mut collector = PointCollector::new(scale);
    for (_, glyph) in outlines.iter() {
        collector.clear();
        glyph
            .draw(
                DrawSettings::unhinted(Size::unscaled(), location),
                &mut collector,
            )
            .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
        for &point in collector.points() {
            if rng.keep(sample_rate) {
                accumulated.push(point);
            }
        }
    }
    Ok(accumulated)
}

pub fn outline_coordinates(
    font_paths: Vec<PathBuf>,
    sample_rate: f64,
) -> Result<Vec<[f32; 2]>, CoordinateError> {
    let mut accumulated = Vec::new();
    let mut rng = Rng::new();
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        for loc in instance_locations(&font) {
            let location = loc
                .as_ref()
                .map_or(LocationRef::default(), LocationRef::from);
            accumulated.append(&mut collect_points_from_font(
                &path,
                &font,
                location,
                sample_rate,
                &mut rng,
            )?);
        }
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
        for loc in instance_locations(&font) {
            let location = loc
                .as_ref()
                .map_or(LocationRef::default(), LocationRef::from);
            for (_, glyph) in outlines.iter() {
                let mut pen = CommandCountPen::default();
                glyph
                    .draw(DrawSettings::unhinted(Size::unscaled(), location), &mut pen)
                    .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                counts.push(pen.into_count());
            }
        }
    }
    Ok(counts)
}

pub fn glyph_command_and_path_counts(
    font_paths: Vec<PathBuf>,
) -> Result<Vec<[u32; 2]>, CoordinateError> {
    let mut counts = Vec::new();
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let outlines = font.outline_glyphs();
        for loc in instance_locations(&font) {
            let location = loc
                .as_ref()
                .map_or(LocationRef::default(), LocationRef::from);
            for (_, glyph) in outlines.iter() {
                let mut pen = CommandStatsPen::default();
                glyph
                    .draw(DrawSettings::unhinted(Size::unscaled(), location), &mut pen)
                    .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                counts.push(pen.into_counts());
            }
        }
    }
    Ok(counts)
}

pub fn outline_formats(font_paths: Vec<PathBuf>) -> Result<Vec<String>, CoordinateError> {
    let mut formats = Vec::with_capacity(font_paths.len());
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let label = match font.outline_glyphs().format() {
            Some(OutlineGlyphFormat::Glyf) => "TrueType",
            Some(OutlineGlyphFormat::Cff) => "CFF",
            Some(OutlineGlyphFormat::Cff2) => "CFF2",
            None => "Unknown",
        };
        let count = font.named_instances().len().max(1);
        formats.extend(std::iter::repeat_n(label.to_string(), count));
    }
    Ok(formats)
}

pub fn command_breakdown(font_paths: Vec<PathBuf>) -> Result<([u64; 5], u64), CoordinateError> {
    let mut totals = [0u64; 5];
    let mut glyph_count = 0u64;
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let outlines = font.outline_glyphs();
        for loc in instance_locations(&font) {
            let location = loc
                .as_ref()
                .map_or(LocationRef::default(), LocationRef::from);
            for (_, glyph) in outlines.iter() {
                let mut pen = CommandBreakdownPen::default();
                glyph
                    .draw(DrawSettings::unhinted(Size::unscaled(), location), &mut pen)
                    .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                for (total, value) in totals.iter_mut().zip(pen.into_counts()) {
                    *total = total.saturating_add(value);
                }
                glyph_count = glyph_count.saturating_add(1);
            }
        }
    }
    Ok((totals, glyph_count))
}
