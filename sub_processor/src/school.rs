use crate::teacher::Teacher;
use pyo3::prelude::*;
use std::collections::HashMap;

use parking_lot::Mutex;
use std::sync::Arc;
use std::time::Instant;

#[allow(clippy::new_without_default)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Class {
    pub class_name: String,
    pub list_of_periods: Vec<(Arc<Mutex<Teacher>>, i16)>,
}

#[pyclass]
#[derive(Default)]
pub struct School {
    name_list_teacher: HashMap<String, Arc<Mutex<Teacher>>>,
    list_of_teachers: Vec<Arc<Mutex<Teacher>>>,
    list_of_classes: Vec<Arc<Mutex<Class>>>,
    // teacher_hashmap: HashMap<String,i16>,
}

#[pyfunction]
pub fn register_period(
    teacher: &Teacher,
    period: i16,
    school: &mut School,
    class: &mut Class,
) -> PyResult<()> {
    let grade = &class.class_name;
    // let section = class.class_name.chars().last().expect("Couldnt get section");
    match school.name_list_teacher.get(&teacher.name) {
        Some(teacher_in_hashmap) => {
            class
                .list_of_periods
                .push((teacher_in_hashmap.clone(), period));

            let _ = teacher_in_hashmap
                .clone()
                .lock()
                .add_period(period, grade.clone());
            class
                .list_of_periods
                .push((teacher_in_hashmap.clone(), period));
        }
        None => {
            panic!("Teacher not found in existing list. Please add to 'teacher_list.csv'")
        }
    }
    Ok(())
    //                 }
}

pub fn build_hashtable(school: &mut School) -> HashMap<String, Vec<Arc<Mutex<Teacher>>>> {
    let mut hashtable: HashMap<String, Vec<Arc<Mutex<Teacher>>>> = HashMap::new();

    school.list_of_teachers.iter().for_each(|teacher| {
        let sub = teacher.lock().get_sub().unwrap();
        match hashtable.get_mut(&sub) {
            Some(t) => {
                if teacher.lock().present {
                    t.push(teacher.clone())
                }
            }
            None => {
                if teacher.lock().present {
                    hashtable.insert(sub, vec![teacher.clone()]);
                } else {
                    hashtable.insert(sub, vec![]);
                }
            }
        };
    });

    hashtable
}

#[pymethods]
impl School {
    pub fn add_teacher(&mut self, teacher: &Teacher) {
        match self.name_list_teacher.get(&teacher.name) {
            Some(_t) => {}
            None => {
                let new_teacher = Arc::new(Mutex::new(teacher.clone()));
                self.list_of_teachers.push(new_teacher.clone()); // add to list of teacher
                self.name_list_teacher
                    .insert(teacher.name.clone(), new_teacher.clone()); // add to hashmap
            }
        }
    }

    pub fn generate_substitutions(&mut self) -> PyResult<String> {
        let to_print = Arc::new(Mutex::new(String::from(
            "Teacher,Period,Subject,Substitution,Classroom,Reason\n",
        )));
        let failure_log = Arc::new(Mutex::new(String::new()));
        let benchmark_logs = Arc::new(Mutex::new(String::new()));
        Python::with_gil(|_py: Python<'_>| {
            let start = Instant::now();
            let hashtable: HashMap<String, Vec<Arc<Mutex<Teacher>>>> = build_hashtable(self);
            // iterate over classes
            self.list_of_classes.iter_mut().for_each(|class| {
                let grade = class.lock().class_name.clone();
                let mut subbed_map: HashMap<String, (bool, i32)> = HashMap::new();

                // iterate over the periods in a class
                class.lock().list_of_periods.iter_mut().for_each(
                    |period: &mut (Arc<Mutex<Teacher>>, i16)| {
                        let teacher = period.0.clone();
                        if !teacher.lock().present {
                            let name = teacher.lock().name.clone();

                            match subbed_map.get(&name) {
                                Some(_) => {}
                                None => {
                                    subbed_map.insert(name, (false, 0));
                                }
                            }

                            let period_num: i16 = period.1;
                            let sub = teacher.lock().get_sub().expect("Unable to get subject");
                            let reason = teacher
                                .lock()
                                .reason_of_absentee
                                .clone()
                                .expect("Unable to get subject");

                            let teacher_sub_list: Option<&Vec<Arc<Mutex<Teacher>>>> =
                                hashtable.get(&sub);

                            let mut sorted_list: Vec<Arc<Mutex<Teacher>>> =
                                teacher_sub_list.unwrap().to_vec();
                            sorted_list.sort_by(
                                |a: &Arc<Mutex<Teacher>>, b: &Arc<Mutex<Teacher>>| {
                                    a.lock().periods.len().cmp(&b.lock().periods.len())
                                },
                            );

                            for new_teacher in sorted_list {
                                if subbed_map.get(&teacher.lock().name).unwrap().1
                                    == period_num.into()
                                {
                                    continue;
                                }
                                let new_teacher_period = new_teacher
                                    .lock()
                                    .periods
                                    .iter()
                                    .map(|period| period.0)
                                    .collect::<Vec<i16>>();
                                if !new_teacher_period.contains(&period_num) {
                                    let _ = new_teacher
                                        .clone()
                                        .lock()
                                        .add_period(period_num, grade.clone());
                                    to_print.lock().push_str(&format!(
                                        "{},{},{},{},{},{}\n",
                                        teacher.lock().name.clone(),
                                        period_num,
                                        sub,
                                        new_teacher.lock().name.clone(),
                                        grade.clone(),
                                        reason
                                    ));
                                    subbed_map.insert(
                                        teacher.lock().name.clone(),
                                        (true, period_num.into()),
                                    );
                                    break;
                                }
                            }
                            if !(subbed_map.get(&teacher.lock().name).unwrap().0) {
                                let mut found: bool = false;
                                for new_teacher in &self.list_of_teachers {
                                    if !new_teacher.lock().present {
                                        continue;
                                    }
                                    let new_teacher_period: Vec<i16> = new_teacher
                                        .lock()
                                        .periods
                                        .iter()
                                        .map(|period| period.0)
                                        .collect::<Vec<i16>>();
                                    if !new_teacher_period.contains(&period_num) {
                                        let _ = new_teacher
                                            .clone()
                                            .lock()
                                            .add_period(period_num, grade.clone());
                                        to_print.lock().push_str(&format!(
                                            "{},{},{},{},{},{}\n",
                                            teacher.lock().name.clone(),
                                            period_num,
                                            sub,
                                            new_teacher.lock().name.clone(),
                                            grade.clone(),
                                            reason
                                        ));
                                        subbed_map.insert(
                                            teacher.lock().name.clone(),
                                            (true, period_num.into()),
                                        );
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    failure_log.lock().push_str(&format!(
                                        "Couldnt find a substitution for {} at {}-{:?}\n",
                                        teacher.lock().name,
                                        period_num,
                                        grade.clone()
                                    ));
                                }
                            }
                            // },
                            // None => to_print.push_str(&format!("unable to operate on teacher {}\n",teacher.lock().name)),
                        }

                        // }
                    },
                );
            });
            let duration = start.elapsed();
            benchmark_logs
                .lock()
                .push_str(&format!("Time taken to generate subs: {:?}", duration));
        });
        let to_print = to_print.lock().clone();
        let benchmark_logs = benchmark_logs.lock().clone();

        Ok(format!(
            "{to_print}\n{:?}\n{:?}",
            failure_log.lock(),
            benchmark_logs
        ))
    }
    #[new]
    pub fn new() -> Self {
        School { ..Self::default() }
    }
    pub fn add_class(&mut self, class: &Class) {
        self.list_of_classes
            .push(Arc::new(Mutex::new(class.clone())));

        self.list_of_teachers
            .sort_by(|a, b| a.lock().periods.len().cmp(&b.lock().periods.len()));
    }

    fn __str__(&mut self) -> String {
        let hashtable: HashMap<String, Vec<Arc<Mutex<Teacher>>>> = build_hashtable(self);
        format!(
            "List of teachers: {:#?}\nList of classes:{:#?}\nTeacher_hashtable {:?}",
            self.list_of_teachers, self.list_of_classes, hashtable
        )
    }

    pub fn add_to_hashmap(&mut self, name: String, teacher: Teacher) {
        self.name_list_teacher
            .insert(name, Arc::new(Mutex::new(teacher)));
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

    pub fn __str__(&self) -> String {
        format!("{:#?} {:#?}", self.class_name, self.list_of_periods)
    }
}
