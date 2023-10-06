use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;
#[derive(Debug, Clone, Copy)]
enum Subjects {
    Chemistry,
    Physics,
    Maths,
    Computer,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Teacher {
    #[pyo3(get)]
    pub name: String,
    periods: Vec<(i64, (i64, char))>,
    sub: Subjects,
}

#[pymethods]
impl Teacher {
    pub fn add_period(&mut self, period: i64, grade: i64, section: char) -> PyResult<()> {
        Ok(self.periods.push((period, (grade, section))))
    }

    //TODO: will substitute with a getter trait later
    pub fn get_sub(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.sub))
    }

    fn __str__(&self) -> String {
        let mut periods_list: Vec<(i64, String)> = vec![];
        self.periods.iter().for_each(|entry| {
            periods_list.push((entry.0, format!("{}{}", entry.1 .0, entry.1 .1)));
        });
        format!(
            "Teacher:
    {{
        name:{} 
        periods: {:?} 
        subject: {:?}
    }}\n",
            self.name, periods_list, self.sub
        )
    }

    /// class constructor definition
    #[new]
    pub fn __new__(name: &str, sub: &str) -> PyResult<Self> {
        let subject = match sub {
            "chemistry" => Subjects::Chemistry,
            "physics" => Subjects::Physics,
            "maths" => Subjects::Maths,
            "computer" => Subjects::Computer,
            _ => return Err(PyErr::new::<PyTypeError, _>("Wrong subject")),
        };
        Ok(Teacher {
            name: name.to_string(),
            periods: vec![],
            sub: subject,
        })
    }
}
