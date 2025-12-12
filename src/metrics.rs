use std::fs;
use std::path::PathBuf;

use skrifa::raw::TableProvider;
use skrifa::{FontRef, MetadataProvider};

use crate::error::CoordinateError;

pub fn units_per_em(font_paths: Vec<PathBuf>) -> Result<Vec<u16>, CoordinateError> {
    let mut values = Vec::with_capacity(font_paths.len());
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let upem = font
            .head()
            .ok()
            .map(|head| head.units_per_em())
            .unwrap_or(1000);
        values.push(upem);
    }
    Ok(values)
}

pub fn weight_classes(font_paths: Vec<PathBuf>) -> Result<Vec<u16>, CoordinateError> {
    let mut values = Vec::with_capacity(font_paths.len());
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let weight = font.attributes().weight.value().round().clamp(1.0, 1000.0) as u16;
        values.push(weight);
    }
    Ok(values)
}
