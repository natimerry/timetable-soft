use std::{sync::Arc, fmt::format};
use std::collections::HashSet;
use crate::teacher::{Teacher, self};
use pyo3::{prelude::*, types::{PyList, PySequence}};
use rayon::prelude::*;


#[pyclass]
#[derive(Debug,Clone)]
pub struct Class{
    pub list_of_periods: Vec<(Arc<Teacher>,i64)>,
}

#[pyclass]
pub struct School{
    list_of_teachers: Vec<Arc<Teacher>>, 
    list_of_classes: Vec<Arc<Class>>
}


#[pymethods]
impl School{
    #[new]
    pub fn new() -> Self{
        School {list_of_teachers:vec![], list_of_classes: vec![] }
    }
    pub fn add_class(&mut self,class:&Class){
        self.list_of_classes.push(Arc::new(class.clone()));
    }
    pub fn collect_teachers(&mut self){
        let mut name_list:HashSet<String> = HashSet::new();
        self.list_of_classes.iter().for_each(|class|{
            class.list_of_periods.iter().for_each(|teacher|{
                let teacher=&teacher.0;
                if !name_list.contains(&teacher.name){
                    self.list_of_teachers.push(teacher.clone());
                }
                else {
                    name_list.insert(teacher.name.clone());
                }
            });
        });
    }

    fn __str__(&self) -> String {
        format!("{:#?} {:#?}",self.list_of_teachers,self.list_of_classes)
    }
}