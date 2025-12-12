use std::fs;
use std::path::PathBuf;

use skrifa::charmap::Charmap;
use skrifa::{FontRef, MetadataProvider};

use crate::error::CoordinateError;

fn codepoints_under_limit(charmap: Charmap<'_>, limit: u32) -> Vec<u32> {
    charmap
        .mappings()
        .filter_map(|(cp, _)| (cp < limit).then_some(cp))
        .collect()
}

pub fn coverage(font_paths: Vec<PathBuf>, limit: u32) -> Result<Vec<Vec<u32>>, CoordinateError> {
    let mut coverage = Vec::with_capacity(font_paths.len());
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let cps = codepoints_under_limit(font.charmap(), limit);
        coverage.push(cps);
    }
    Ok(coverage)
}
