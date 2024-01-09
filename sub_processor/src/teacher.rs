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
    #[pyo3(get,set)]
    pub present: bool,
    #[pyo3(get,set)]
    pub reason_of_absentee: String,
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
        let sub = sub.to_lowercase();
        let subject = match sub.as_str() {
            "chemistry" => Subjects::Chemistry,
            "physics" => Subjects::Physics,
            "maths" => Subjects::Maths,
            "computer" => Subjects::Computer,
            "english" => Subjects::English,
            "bio" => Subjects::Biology,
            "biology" => Subjects::Biology,
            "P.E" => Subjects::PhysicalEdu,
            "sanskrit" => Subjects::Sanskrit,
            "urdu" => Subjects::Urdu,
            "geography" => Subjects::Geo,
            "accountancy" => Subjects::Acc,
            "history" => Subjects::History,
            "civics" => Subjects::Civics,
            "business" => Subjects::Business,
            "art" => Subjects::Arts,
            "games" => Subjects::Games,
            "economics" => Subjects::Economics,
            "hindi" => Subjects::Hindi,
            "bengali" => Subjects::Bengali,
            "finfance" => Subjects::Finance,
            _ => return Err(PyErr::new::<PyTypeError, _>(format!("Wrong subject {sub}",))),
        };
        Ok(Teacher {
            name: name.to_string(),
            periods: HashSet::new(),
            sub: subject,
            present,
            reason_of_absentee: "Planned Absense".to_string(),
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
    Sanskrit,
    Urdu,
    Geo,
    Acc,
    History,
    Civics,
    Business,
    Arts,
    Games,
    Economics,
    Hindi,
    Bengali,
    Finance
}
