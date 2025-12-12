mod coverage;
mod error;
mod metrics;
mod outline;
mod pens;

use std::path::PathBuf;

use numpy::ndarray::Array2;
use numpy::{IntoPyArray, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;

fn concatenate(all_points: Vec<[f32; 2]>, py: Python<'_>) -> PyResult<Py<PyArray2<f32>>> {
    let total = all_points.len();
    let mut flat = Vec::with_capacity(total * 2);
    for point in all_points {
        flat.push(point[0]);
        flat.push(point[1]);
    }
    let array = Array2::from_shape_vec((total, 2), flat)
        .map_err(|err| PyValueError::new_err(err.to_string()))?;
    Ok(array.into_pyarray(py).unbind())
}

#[pyfunction]
fn glyph_outline_coordinates(
    py: Python<'_>,
    font_paths: Vec<PathBuf>,
) -> PyResult<Py<PyArray2<f32>>> {
    let points = py.detach(move || outline::outline_coordinates(font_paths))?;
    concatenate(points, py)
}

#[pyfunction]
fn units_per_em(py: Python<'_>, font_paths: Vec<PathBuf>) -> PyResult<Vec<u16>> {
    let values = py.detach(move || metrics::units_per_em(font_paths))?;
    Ok(values)
}

#[pyfunction]
fn glyph_command_counts(py: Python<'_>, font_paths: Vec<PathBuf>) -> PyResult<Vec<u32>> {
    let counts = py.detach(move || outline::glyph_command_counts(font_paths))?;
    Ok(counts)
}

#[pyfunction]
fn weight_classes(py: Python<'_>, font_paths: Vec<PathBuf>) -> PyResult<Vec<u16>> {
    let weights = py.detach(move || metrics::weight_classes(font_paths))?;
    Ok(weights)
}

#[pyfunction]
fn coverage_bmp(py: Python<'_>, font_paths: Vec<PathBuf>, limit: u32) -> PyResult<Vec<Vec<u32>>> {
    let coverage = py.detach(move || coverage::coverage(font_paths, limit))?;
    Ok(coverage)
}

#[pymodule]
fn _skrifa(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add("__doc__", "Rust-powered outline helpers built with Skrifa")?;
    module.add_function(wrap_pyfunction!(glyph_outline_coordinates, module)?)?;
    module.add_function(wrap_pyfunction!(units_per_em, module)?)?;
    module.add_function(wrap_pyfunction!(glyph_command_counts, module)?)?;
    module.add_function(wrap_pyfunction!(weight_classes, module)?)?;
    module.add_function(wrap_pyfunction!(coverage_bmp, module)?)?;
    Ok(())
}
