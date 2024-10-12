use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn multiply(a: i32, b: i32, c: i32, d: i32, e: i32) -> PyResult<i32>{
    Ok(a*b*c*d*e)
}

/// A Python module implemented in Rust.
#[pymodule]
fn math_tools(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    Ok(())
}
