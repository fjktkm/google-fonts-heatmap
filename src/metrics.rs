use std::fs;
use std::path::PathBuf;

use skrifa::raw::TableProvider;
use skrifa::{FontRef, MetadataProvider, Tag};

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
        let instances = font.named_instances();
        if instances.is_empty() {
            values.push(upem);
        } else {
            values.extend(std::iter::repeat(upem).take(instances.len()));
        }
    }
    Ok(values)
}

pub fn weight_classes(font_paths: Vec<PathBuf>) -> Result<Vec<u16>, CoordinateError> {
    let mut values = Vec::with_capacity(font_paths.len());
    for path in font_paths {
        let data = fs::read(&path).map_err(|err| CoordinateError::Io(path.to_path_buf(), err))?;
        let font =
            FontRef::new(&data).map_err(|err| CoordinateError::Read(path.to_path_buf(), err))?;
        let instances = font.named_instances();
        let default_weight =
            font.attributes().weight.value().round().clamp(1.0, 1000.0) as u16;
        if instances.is_empty() {
            values.push(default_weight);
        } else {
            let axes = font.axes();
            let wght_tag = Tag::new(b"wght");
            let wght_index = axes.iter().position(|axis| axis.tag() == wght_tag);
            for instance in instances.iter() {
                let weight = wght_index
                    .and_then(|index| instance.user_coords().nth(index))
                    .unwrap_or(default_weight as f32)
                    .round()
                    .clamp(1.0, 1000.0) as u16;
                values.push(weight);
            }
        }
    }
    Ok(values)
}
