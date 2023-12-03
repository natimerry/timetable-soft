
use crate::teacher::Teacher;
use pyo3::prelude::*;
use std::collections::HashMap;

use std::sync::Arc;
use std::sync::Mutex;
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
                .unwrap_or_else(|_| {
                    panic!("Mutex poisoned. Irrecoverable.\n{:#?}", teacher_in_hashmap)
                })
                .add_period(period, grade.clone());
            class.list_of_periods.push((teacher_in_hashmap.clone(), period));
            
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
        let sub = teacher.lock().unwrap().get_sub().unwrap();
        match hashtable.get_mut(&sub) {
            Some(t) => if teacher.lock().unwrap().present{t.push(teacher.clone())},
            None => {
                if teacher.lock().unwrap().present{
                    hashtable.insert(sub, vec![teacher.clone()]);
                }else {
                    hashtable.insert(sub, vec![]);
                    
                }
            }
        };
    });

    hashtable
}


#[pymethods]
impl School {
    pub fn add_teacher(&mut self,teacher:&Teacher){
        match self.name_list_teacher.get(&teacher.name) {
            Some(_t) => {},
            None => {
                let new_teacher = Arc::new(Mutex::new(teacher.clone()));
                self.list_of_teachers.push(new_teacher.clone()); // add to list of teacher
                self
                    .name_list_teacher
                    .insert(teacher.name.clone(), new_teacher.clone()); // add to hashmap
            },
        }   
    }

    pub fn generate_time_table(&mut self) -> PyResult<String>{
        let to_print = Arc::new(Mutex::new(String::from("Teacher,Period,Subject,Substitution,Classroom,Reason\n")));
        let failure_log = Arc::new(Mutex::new(String::new()));
        let benchmark_logs = Arc::new(Mutex::new(String::new()));
        Python::with_gil(|_py: Python<'_>|{
            let start = Instant::now();
            let hashtable: HashMap<String, Vec<Arc<Mutex<Teacher>>>> = build_hashtable(self);
            self.list_of_classes.iter_mut().for_each(|class| {
                let grade = class.lock().unwrap().class_name.clone();
                let mut subbed_map:HashMap<String, (bool,i32)> = HashMap::new();
                class.lock().unwrap().list_of_periods.iter_mut().for_each(|period: &mut (Arc<Mutex<Teacher>>, i16)| {
                        let teacher = period.0.clone();
                        if !teacher.lock().unwrap().present{
                            let name = teacher.lock().unwrap().name.clone();

                            match subbed_map.get(&name) {
                                Some(_) => {},
                                None => {subbed_map.insert(name, (false,0));}
                            }

                            let period_num:i32 = period.1 as i32;
                            let sub = teacher.lock().expect("Unable to lock mutex").get_sub().expect("Unable to get subject");
                            let reason = teacher.lock().expect("Unable to lock mutex").reason_of_absentee.clone().expect("Unable to get subject");

                            let teacher_sub_list: Option<&Vec<Arc<Mutex<Teacher>>>> = hashtable.get(&sub);

                            let mut sorted_list: Vec<Arc<Mutex<Teacher>>> = teacher_sub_list.unwrap().clone();
                            sorted_list.sort_by(|a: &Arc<Mutex<Teacher>>,b: &Arc<Mutex<Teacher>>|a.lock().unwrap().periods.len()
                                                    .cmp(&b.lock().unwrap().periods.len()));

                            // match teacher_sub_list{
                                // Some(teacher_from_sub) => {
                            for new_teacher in sorted_list{
                                if subbed_map.get(&teacher.lock().unwrap().name).unwrap().1 == period_num{
                                    continue;
                                }
                                let new_teacher_period = new_teacher.lock().unwrap().periods.iter().map(|period|{
                                    period.0
                                }).collect::<Vec<i16>>();
                                if !new_teacher_period.contains(&period_num.try_into().unwrap()){
                                    let _ = new_teacher.clone().lock().unwrap().add_period(period_num.try_into().unwrap(), grade.clone());
                                    to_print.lock().unwrap().push_str(&format!("{},{},{},{},{},{}\n", 
                                                                            teacher.lock().unwrap().name.clone(),
                                                                            period_num,
                                                                            sub,
                                                                            new_teacher.lock().unwrap().name.clone(),
                                                                            grade.clone(),reason));
                                    subbed_map.insert(teacher.lock().unwrap().name.clone(), (true,period_num));
                                    break;
                                }
                            }
                            if !(subbed_map.get(&teacher.lock().unwrap().name).unwrap().0){
                                let mut found: bool= false;
                                for new_teacher in &self.list_of_teachers{
                                    if !new_teacher.lock().unwrap().present{
                                        continue;
                                    } 
                                    let new_teacher_period: Vec<i16> = new_teacher.lock().unwrap().periods.iter().map(|period|{
                                        period.0
                                    }).collect::<Vec<i16>>();
                                    if !new_teacher_period.contains(&period_num.try_into().unwrap()){
                                        let _ = new_teacher.clone().lock().unwrap().add_period(period_num.try_into().unwrap(), grade.clone());
                                        to_print.lock().unwrap().push_str(&format!("{},{},{},{},{},{}\n", 
                                                                                teacher.lock().unwrap().name.clone(),
                                                                                period_num,
                                                                                sub,
                                                                                new_teacher.lock().unwrap().name.clone(),
                                                                                grade.clone(),reason));
                                        subbed_map.insert(teacher.lock().unwrap().name.clone(), (true,period_num));
                                        found=true;
                                        break;
                                    }
                                }
                                if !found{
                                    failure_log.lock().unwrap().push_str(&format!("Couldnt find a substitution for {} at {}-{:?}\n",
                                                    teacher.lock().unwrap().name,period_num,grade.clone()));
                                }
                            }
                                // },
                                // None => to_print.push_str(&format!("unable to operate on teacher {}\n",teacher.lock().unwrap().name)),

                            }

                        // }
                        
                    });
            });
            let duration = start.elapsed();
            benchmark_logs.lock().unwrap().push_str(&format!("Time taken to generate subs: {:?}",duration));
        });
        let to_print = to_print.lock().unwrap().clone();
        let benchmark_logs = benchmark_logs.lock().unwrap().clone();

        Ok(format!("{to_print}\n{:?}\n{:?}",failure_log.lock(),benchmark_logs))
    }
    #[new]
    pub fn new() -> Self {
        School {
            ..Self::default()
        }
    }
    pub fn add_class(&mut self, class: &Class) {
        self.list_of_classes
            .push(Arc::new(Mutex::new(class.clone())));

        self.list_of_teachers.sort_by(|a, b| {
            a.lock()
                .unwrap()
                .periods
                .len()
                .cmp(&b.lock().unwrap().periods.len())
        });
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
