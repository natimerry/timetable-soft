use pyo3::prelude::*;
mod school;
mod teacher;

use school::*;

#[pymodule]
fn substitution_gen_lib_rs(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<teacher::Teacher>()?;
    module.add_class::<school::School>()?;
    module.add_class::<school::Class>()?;
    module.add_function(wrap_pyfunction!(register_period, module)?)?;
    Ok(())
}
