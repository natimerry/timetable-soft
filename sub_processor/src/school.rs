use crate::teacher;
use crate::teacher::Teacher;
use pyo3::prelude::*;
use pyo3::types::PyList;
use std::collections::HashSet;
use std::sync::Arc;
use std::sync::Mutex;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Class {
    pub class_name: String,
    pub list_of_periods: Vec<(Arc<Mutex<Teacher>>, i16)>,
}

#[pyclass]
pub struct School {
    name_list_teacher: HashSet<String>,
    list_of_teachers: Vec<Arc<Mutex<Teacher>>>,
    list_of_classes: Vec<Arc<Mutex<Class>>>,
    // teacher_hashmap: HashMap<String,i16>,
}

pub fn collect_teachers(school: &mut School) {
    // let mut name_list: HashSet<String> = HashSet::new();
    school.list_of_classes.iter().for_each(|class| {
        let class = class.lock().unwrap();
        class.list_of_periods.iter().for_each(|teacher| {
            let teacher: &Arc<Mutex<Teacher>> = &teacher.0;
            if !school.name_list_teacher.contains(&teacher.lock().unwrap().name) {
                school.list_of_teachers.push(teacher.clone());
                school.name_list_teacher.insert(teacher.clone().lock().unwrap().name.clone());
            }
        });
    });
}

#[pymethods]
impl School {
    #[new]
    pub fn new() -> Self {
        School {
            list_of_teachers: vec![],
            list_of_classes: vec![],
            name_list_teacher: HashSet::new(),
        }
    }
    pub fn add_class(&mut self, class: &Class) {
        self.list_of_classes
            .push(Arc::new(Mutex::new(class.clone())));
        collect_teachers(self);
    }

    // move this outside, try to get the namelist hashset from
    // python implementation as it reduces computation time
    fn __str__(&mut self) -> String {
        let mut pretty_list: Vec<Class> = vec![];
        self.list_of_classes.iter().for_each(|class| {
            pretty_list.push(class.lock().unwrap().clone());
        });

        let mut pretty_list: Vec<Teacher> = vec![];
        self.list_of_teachers.iter().for_each(|teacher| {
            pretty_list.push(teacher.lock().unwrap().clone());
        });
        // format!("{:#?} {:#?}", self.list_of_teachers, pretty_list)
        format!("{:?}", self.list_of_teachers)
    }

    pub fn generate_substitutions(&mut self) -> PyResult<()> {
        // check sorting
        self.list_of_teachers.sort_by(|a, b| {
            a.lock()
                .unwrap()
                .periods
                .len()
                .cmp(&b.lock().unwrap().periods.len())
        });

        Ok(())
    }
}

#[pymethods]
impl Class {
    #[new]
    pub fn __new__(name: String) -> Self {
        Class {
            class_name: name,
            list_of_periods: vec![],
        }
    }

    pub fn add_teacher(&mut self, teacher: Teacher, period: i16) {
        let mut teacher = teacher.clone();
        let grade = str::parse::<i16>(&self.class_name[0..2]).unwrap();
        let section = self.class_name.chars().last().unwrap();
        let _ = teacher.add_period(period, grade, section);
        self.list_of_periods
            .push((Arc::new(teacher.into()), period));
    }

    pub fn __str__(&self) -> String {
        let mut pretty_list: Vec<(Teacher, i16)> = vec![];
        self.list_of_periods.iter().for_each(|period| {
            pretty_list.push((period.0.lock().unwrap().clone(), period.1));
        });
        format!("{:#?} {:#?}", self.class_name, pretty_list)
    }
}
