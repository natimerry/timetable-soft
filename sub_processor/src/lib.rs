use pyo3::prelude::*;
mod school;
mod teacher;

use school::*;

/// fucking autistic syntax to expose basic functions, so i use
/// a macro to shorten it in case i need it
// macro_rules! mod_add_func {
//     ($m:ident, $f:ident) => {
//         $m.add_function(wrap_pyfunction!($f, $m)?)?;
//     };
// }

/// shit exposed to python modul
///
/// make sure to add bindings to sub_processor/sub_processor.pyi
/// to make sure type annotations work
#[pymodule]
fn substitution_gen_lib_rs(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<teacher::Teacher>()?;
    module.add_class::<school::School>()?;
    module.add_class::<school::Class>()?;
    module.add_function(wrap_pyfunction!(register_period,module)?)?;
    Ok(())
}
