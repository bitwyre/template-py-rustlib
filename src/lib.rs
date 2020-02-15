use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::{PyResult, Python};

#[no_mangle]
#[pyfunction]
#[text_signature = "(name, /)"]
pub unsafe extern "C" fn say_hello_to(name: &str) -> String {
    format!("Hello {}! I'm \u{1F980}  !", name)
}

#[pymodule]
fn py_rustlib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello_to))?;
    Ok(())
}
