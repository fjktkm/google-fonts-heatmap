use std::fs;
use std::path::PathBuf;

use skrifa::raw::TableProvider;
use skrifa::FontRef;

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
