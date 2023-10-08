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
    pub periods: Vec<(i16, (i16, char))>,
    sub: Subjects,
    pub present: bool,
}

#[pymethods]
impl Teacher {
    pub fn add_period(&mut self, period: i16, grade: i16, section: char) -> PyResult<()> {
        Ok(self.periods.push((period, (grade, section))))
    }

    //TODO: will substitute with a getter trait later
    pub fn get_sub(&self) -> PyResult<String> {
        Ok(format!("{:?}", self.sub))
    }

    fn __str__(&self) -> String {
        let mut periods_list: Vec<(i16, String)> = vec![];
        self.periods.iter().for_each(|entry: &(i16, (i16, char))| {
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
    pub fn __new__(name: &str, sub: &str, present: bool) -> PyResult<Self> {
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
            present,
        })
    }

    pub fn swap_periods(&mut self, period: i16, data: (i16, char)) {
        for i in 0..self.periods.len() {
            if self.periods[i].0 == period {
                let _ = self.periods[i] == (period, (data));
            }
        }
    }
}
