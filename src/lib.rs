mod coverage;
mod error;
mod metrics;
mod outline;
mod pens;

use std::path::PathBuf;

use numpy::ndarray::Array2;
use numpy::{Element, IntoPyArray, PyArray2};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyModule;
use pyo3::Bound;

fn into_array2<T: Element + Copy>(
    pairs: Vec<[T; 2]>,
    py: Python<'_>,
) -> PyResult<Py<PyArray2<T>>> {
    let total = pairs.len();
    let flat: Vec<T> = pairs.into_iter().flatten().collect();
    let array = Array2::from_shape_vec((total, 2), flat)
        .map_err(|err| PyValueError::new_err(err.to_string()))?;
    Ok(array.into_pyarray(py).unbind())
}

#[pyfunction]
fn glyph_outline_coordinates(
    py: Python<'_>,
    font_paths: Vec<PathBuf>,
    sample_rate: f64,
) -> PyResult<Py<PyArray2<f32>>> {
    let points = py.detach(move || outline::outline_coordinates(font_paths, sample_rate))?;
    into_array2(points, py)
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
fn glyph_command_and_path_counts(
    py: Python<'_>,
    font_paths: Vec<PathBuf>,
) -> PyResult<Py<PyArray2<u32>>> {
    let counts = py.detach(move || outline::glyph_command_and_path_counts(font_paths))?;
    into_array2(counts, py)
}

#[pyfunction]
fn outline_formats(py: Python<'_>, font_paths: Vec<PathBuf>) -> PyResult<Vec<String>> {
    let formats = py.detach(move || outline::outline_formats(font_paths))?;
    Ok(formats)
}

#[pyfunction]
fn outline_command_breakdown(
    py: Python<'_>,
    font_paths: Vec<PathBuf>,
) -> PyResult<(Vec<u64>, u64)> {
    let (totals, glyphs) = py.detach(move || outline::command_breakdown(font_paths))?;
    Ok((totals.to_vec(), glyphs))
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
    module.add_function(wrap_pyfunction!(glyph_command_and_path_counts, module)?)?;
    module.add_function(wrap_pyfunction!(outline_formats, module)?)?;
    module.add_function(wrap_pyfunction!(outline_command_breakdown, module)?)?;
    module.add_function(wrap_pyfunction!(weight_classes, module)?)?;
    module.add_function(wrap_pyfunction!(coverage_bmp, module)?)?;
    Ok(())
}
