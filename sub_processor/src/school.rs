use crate::teacher::{self, Teacher};
use pyo3::{
    prelude::*,
    types::{PyList, PySequence},
};
use rayon::prelude::*;
use std::collections::HashSet;
use std::{fmt::format, sync::Arc};

#[pyclass]
#[derive(Debug, Clone)]
pub struct Class {
    pub list_of_periods: Vec<(Arc<Teacher>, i64)>,
}

#[pyclass]
pub struct School {
    list_of_teachers: Vec<Arc<Teacher>>,
    list_of_classes: Vec<Arc<Class>>,
}

#[pymethods]
impl School {
    #[new]
    pub fn new() -> Self {
        School {
            list_of_teachers: vec![],
            list_of_classes: vec![],
        }
    }
    pub fn add_class(&mut self, class: &Class) {
        self.list_of_classes.push(Arc::new(class.clone()));
    }

    // move this outside, try to get the namelist hashset from 
    // python implementation as it reduces computation time
    pub fn collect_teachers(&mut self) {
        let mut name_list: HashSet<String> = HashSet::new();
        self.list_of_classes.iter().for_each(|class| {
            class.list_of_periods.iter().for_each(|teacher| {
                let teacher = &teacher.0;
                if !name_list.contains(&teacher.name) {
                    self.list_of_teachers.push(teacher.clone());
                } else {
                    name_list.insert(teacher.name.clone());
                }
            });
        });
    }

    fn __str__(&self) -> String {
        format!("{:#?} {:#?}", self.list_of_teachers, self.list_of_classes)
    }
}
