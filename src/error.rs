use std::path::PathBuf;

use pyo3::exceptions::{PyIOError, PyRuntimeError};
use pyo3::PyErr;
use skrifa::outline::DrawError;
use skrifa::raw::ReadError;

#[derive(Debug)]
pub enum CoordinateError {
    Io(PathBuf, std::io::Error),
    Read(PathBuf, ReadError),
    Draw(PathBuf, DrawError),
}

impl CoordinateError {
    pub fn context(self) -> PyErr {
        match self {
            CoordinateError::Io(path, err) => {
                PyIOError::new_err(format!("Failed to read font {}: {err}", path.display()))
            }
            CoordinateError::Read(path, err) => {
                PyRuntimeError::new_err(format!("Failed to parse font {}: {err}", path.display()))
            }
            CoordinateError::Draw(path, err) => PyRuntimeError::new_err(format!(
                "Failed to extract outlines from {}: {err}",
                path.display()
            )),
        }
    }
}

impl From<CoordinateError> for PyErr {
    fn from(value: CoordinateError) -> Self {
        value.context()
    }
}
