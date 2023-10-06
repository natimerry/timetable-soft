use pyo3::prelude::*;
mod school;
mod teacher;
#[allow(unused_macros)]

/// fucking autistic syntax to expose basic functions, so i use
/// a macro to shorten it in case i need it
macro_rules! mod_add_func {
    ($m:ident, $f:ident) => {
        $m.add_function(wrap_pyfunction!($f, $m)?)?;
    };
}

/// shit exposed to python modul
///
/// make sure to add bindings to sub_processor/sub_processor.pyi
/// to make sure type annotations work
#[pymodule]
fn sub_processor(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<teacher::Teacher>()?;
    module.add_class::<school::School>()?;
    module.add_class::<school::Class>()?;
    Ok(())
}
