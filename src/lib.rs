use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

const VERSION: &str = env!("CARGO_PKG_VERSION");
#[pyfunction]
fn get_o3tree_version() -> &'static str {
    VERSION
}

/// A Python module implemented in Rust.
#[pymodule]
fn o3tree(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_o3tree_version, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
