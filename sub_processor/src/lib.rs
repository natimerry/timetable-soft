use pyo3::prelude::*;
mod teacher;
mod school;
#[allow(unused_macros)]
macro_rules! mod_add_func {
    ($m:ident, $f:ident) => {
        $m.add_function(wrap_pyfunction!($f, $m)?)?;
    };
}

/// A Python module implemented in Rust.
#[pymodule]
fn sub_processor(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add_class::<teacher::Teacher>()?;
    m.add_class::<school::School>()?;

    Ok(())
}