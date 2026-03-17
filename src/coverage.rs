use std::fs;
use std::path::PathBuf;

use skrifa::{FontRef, MetadataProvider};

use crate::error::CoordinateError;

pub fn coverage(font_paths: Vec<PathBuf>, limit: u32) -> Result<Vec<Vec<u32>>, CoordinateError> {
    let mut coverage = Vec::with_capacity(font_paths.len());
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let cps: Vec<u32> = font
            .charmap()
            .mappings()
            .filter_map(|(cp, _)| (cp < limit).then_some(cp))
            .collect();
        let count = font.named_instances().len().max(1);
        coverage.extend(std::iter::repeat_n(cps, count));
    }
    Ok(coverage)
}
