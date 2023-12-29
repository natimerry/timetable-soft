use std::collections::HashSet;

use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Teacher {
    #[pyo3(get)]
    pub name: String,
    pub periods: HashSet<(i16, String)>,
    pub sub: Subjects,
    pub present: bool,
    pub reason_of_absentee: Option<String>,
}

#[pymethods]
impl Teacher {
    pub fn add_period(&mut self, period: i16, grade: String) -> PyResult<()> {
        self.periods.insert((period, grade));
        Ok(())
    }

    //TODO: will substitute with a getter trait later
    pub fn get_sub(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.sub))
    }

    fn __str__(&self) -> String {
        let mut periods_list: Vec<String> = vec![];
        self.periods.iter().for_each(|entry: &(i16, String)| {
            periods_list.push(format!("{}:{}", entry.0, entry.1));
        });

        format!(
            "Teacher:
    {{
        name:{} 
        periods: {:?} 
        subject: {:?}
        present: {:?}
    }}\n",
            self.name, periods_list, self.sub, self.present
        )
    }

    /// class constructor definition
    #[new]
    pub fn __new__(name: &str, sub: &str, present: bool) -> PyResult<Self> {
        let subject = match sub {
            "chemistry" => Subjects::Chemistry,
            "physics" => Subjects::Physics,
            "maths" => Subjects::Maths,
            "computer" => Subjects::Computer,
            "english" => Subjects::English,
            "bio" => Subjects::Biology,
            "P.E" => Subjects::PhysicalEdu,
            _ => return Err(PyErr::new::<PyTypeError, _>("Wrong subject")),
        };
        Ok(Teacher {
            name: name.to_string(),
            periods: HashSet::new(),
            sub: subject,
            present,
            reason_of_absentee: Some("Planned Absense".to_string()),
        })
    }
}
#[derive(Debug, Clone, Copy)]
pub enum Subjects {
    Chemistry,
    Physics,
    Maths,
    Computer,
    English,
    Biology,
    PhysicalEdu,
}
