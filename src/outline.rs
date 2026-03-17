use std::fs;
use std::path::{Path, PathBuf};

use skrifa::instance::{LocationRef, Size};
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

    fn next_f64(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
    }

    fn keep(&mut self, rate: f64) -> bool {
        self.next_f64() < rate
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
        let instances = font.named_instances();
        if instances.is_empty() {
            let mut per_font =
                collect_points_from_font(&path, &font, LocationRef::default(), sample_rate, &mut rng)?;
            accumulated.append(&mut per_font);
        } else {
            for instance in instances.iter() {
                let location = instance.location();
                let mut per_font = collect_points_from_font(
                    &path,
                    &font,
                    LocationRef::from(&location),
                    sample_rate,
                    &mut rng,
                )?;
                accumulated.append(&mut per_font);
            }
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
        let instances = font.named_instances();
        if instances.is_empty() {
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
        } else {
            for instance in instances.iter() {
                let location = instance.location();
                let outlines = font.outline_glyphs();
                for (_, glyph) in outlines.iter() {
                    let mut pen = CommandCountPen::default();
                    glyph
                        .draw(
                            DrawSettings::unhinted(Size::unscaled(), LocationRef::from(&location)),
                            &mut pen,
                        )
                        .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                    counts.push(pen.into_count());
                }
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
        let instances = font.named_instances();
        if instances.is_empty() {
            let outlines = font.outline_glyphs();
            for (_, glyph) in outlines.iter() {
                let mut pen = CommandStatsPen::default();
                glyph
                    .draw(
                        DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
                        &mut pen,
                    )
                    .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                counts.push(pen.into_counts());
            }
        } else {
            for instance in instances.iter() {
                let location = instance.location();
                let outlines = font.outline_glyphs();
                for (_, glyph) in outlines.iter() {
                    let mut pen = CommandStatsPen::default();
                    glyph
                        .draw(
                            DrawSettings::unhinted(Size::unscaled(), LocationRef::from(&location)),
                            &mut pen,
                        )
                        .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                    counts.push(pen.into_counts());
                }
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
        let collection = font.outline_glyphs();
        let label = match collection.format() {
            Some(OutlineGlyphFormat::Glyf) => "TrueType",
            Some(OutlineGlyphFormat::Cff) => "CFF",
            Some(OutlineGlyphFormat::Cff2) => "CFF2",
            None => "Unknown",
        };
        let instances = font.named_instances();
        if instances.is_empty() {
            formats.push(label.to_string());
        } else {
            formats.extend(std::iter::repeat_n(label.to_string(), instances.len()));
        }
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
        let instances = font.named_instances();
        if instances.is_empty() {
            let outlines = font.outline_glyphs();
            for (_, glyph) in outlines.iter() {
                let mut pen = CommandBreakdownPen::default();
                glyph
                    .draw(
                        DrawSettings::unhinted(Size::unscaled(), LocationRef::default()),
                        &mut pen,
                    )
                    .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                let counts = pen.counts();
                for (total, value) in totals.iter_mut().zip(counts) {
                    *total = total.saturating_add(value);
                }
                glyph_count = glyph_count.saturating_add(1);
            }
        } else {
            for instance in instances.iter() {
                let location = instance.location();
                let outlines = font.outline_glyphs();
                for (_, glyph) in outlines.iter() {
                    let mut pen = CommandBreakdownPen::default();
                    glyph
                        .draw(
                            DrawSettings::unhinted(
                                Size::unscaled(),
                                LocationRef::from(&location),
                            ),
                            &mut pen,
                        )
                        .map_err(|err| CoordinateError::Draw(path.to_path_buf(), err))?;
                    let counts = pen.counts();
                    for (total, value) in totals.iter_mut().zip(counts) {
                        *total = total.saturating_add(value);
                    }
                    glyph_count = glyph_count.saturating_add(1);
                }
            }
        }
    }
    Ok((totals, glyph_count))
}
