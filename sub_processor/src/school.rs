use crate::teacher::Teacher;
use pyo3::prelude::*;
use std::collections::HashMap;

use std::sync::Arc;
use std::sync::Mutex;
#[allow(clippy::new_without_default)]
#[pyclass]
#[derive(Debug, Clone)]
pub struct Class {
    pub class_name: String,
    pub list_of_periods: Vec<(Arc<Mutex<Teacher>>, i16)>,
}

#[pyclass]
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
    let grade = str::parse::<i16>(&class.class_name[0..2]).unwrap();
    let section = class.class_name.chars().last().expect("Couldnt get section");
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
                .add_period(period, grade, section);
            class.list_of_periods.push((teacher_in_hashmap.clone(), period));
            
        }
        None => {
            // let new_teacher = Arc::new(Mutex::new(teacher.clone()));
            // school.list_of_teachers.push(new_teacher.clone()); // add to list of teacher
            // school
            //     .name_list_teacher
            //     .insert(teacher.name.clone(), new_teacher.clone()); // add to hashmap

            // let _ = new_teacher
            //     .clone()
            //     .lock() // clone increments reference count and returns a reference, then lock the mutex
            //     .unwrap_or_else(|_| panic!("Mutex poisoned. Irrecoverable.\n{:#?}", new_teacher))
            //     .add_period(period, grade, section);
            // class.list_of_periods.push((new_teacher.clone(), period));
            panic!("Teacher not found in existing list. Please add to 'teacher_list.csv'")
        }
    }
    Ok(())
    //                 }
}
// pub fn collect_teachers(school: &mut School){
//     // let mut name_list: HashSet<String> = HashSet::new();
//     Python::with_gil(|py|{
//         py.allow_threads(move ||{
//             for class in &school.list_of_classes {
//                 let class_locked = class.lock().unwrap();
//                 for i in 0..class_locked.list_of_periods.len() {
//                     // let _ = py.run("print('period')", None, None); print to stdout
//                     let teacher: &Arc<Mutex<Teacher>> = &class_locked.list_of_periods[i].0;
//                     let periods = teacher.lock().unwrap().periods.clone().into_iter().next().unwrap(); // period for current iteration
//                     let name = teacher.lock().unwrap().name.clone();

//                     match school.name_list_teacher.get(&name) {
//                         Some(teacher_in_hashmap) => {
//                             teacher.clone().lock().unwrap().periods.insert(periods);
//                             teacher_in_hashmap.clone().lock().unwrap().periods.insert(periods);
//                         }
//                         None => {
//                             school.list_of_teachers.push(teacher.clone());
//                             school.name_list_teacher.insert(name, teacher.clone());
//                         }
//                     }
//                 }
//             }
//         });
//     });

// }

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
        let mut to_print = String::from("Teacher,Period,Subject,Substitution,Classroom\n");
        let mut failure_log = String::new();
        Python::with_gil(|_py|{
            let hashtable = build_hashtable(self);
            self.list_of_classes.iter().for_each(|class| {
                let grade = str::parse::<i16>(&class.lock().unwrap().class_name[0..2]).unwrap();
                let section = class.lock().unwrap().class_name.chars().last().expect("Couldnt get section");

                let mut subbed_map:HashMap<String, bool> = HashMap::new();
                class.lock().unwrap().list_of_periods.iter_mut().for_each(|period| {
                        let teacher = period.0.clone();
                        if !teacher.lock().unwrap().present{
                            let name = teacher.lock().unwrap().name.clone();
                            match subbed_map.get(&name) {
                                Some(_) => {},
                                None => {subbed_map.insert(name, false);}
                            }
                            let period_num = period.1;
                            let sub = teacher.lock().expect("Unable to lock mutex").get_sub().expect("Unable to get subject");
                            let teacher_sub_list = hashtable.get(&sub);
                            match teacher_sub_list{
                                Some(teacher_vec) => {
                                    // py.run("print(absent_teacher_found)", None, None);   
                                    for new_teacher in teacher_vec{
                                        let new_teacher_period = new_teacher.lock().unwrap().periods.iter().map(|period|{
                                            period.0
                                        }).collect::<Vec<i16>>();
                                        if !new_teacher_period.contains(&period_num){
                                            let _ = new_teacher.clone().lock().unwrap().add_period(period_num, grade, section);
                                            to_print.push_str(&format!("{},{},{},{},{}{}\n", 
                                                                                    teacher.lock().unwrap().name.clone(),
                                                                                    period_num,
                                                                                    sub,
                                                                                    new_teacher.lock().unwrap().name.clone(),
                                                                                    grade,section));
                                            subbed_map.insert(teacher.lock().unwrap().name.clone(), true);
                                            break;
                                        }
                                    }
                                    if !(*subbed_map.get(&teacher.lock().unwrap().name).unwrap()){
                                        failure_log.push_str(&format!("Couldnt find a substitution for {} at {}-{:?}\n",teacher.lock().unwrap().name,period_num,(grade,section)));
                                    }
                                },
                                None => to_print.push_str(&format!("unable to operate on teacher {}\n",teacher.lock().unwrap().name)),
                            }

                        }
                        
                    });
            });
        });
        Ok(format!("{to_print}\n{failure_log}"))
    }
    #[new]
    pub fn new() -> Self {
        School {
            list_of_teachers: vec![],
            list_of_classes: vec![],
            name_list_teacher: HashMap::new(),
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
        // collect_teachers(self);
        // force_teachers(self);
    }

    fn __str__(&mut self) -> String {
        let hashtable = build_hashtable(self);
        format!(
            "List of teachers: {:#?}\nList of classes:{:#?}\nTeacher_hashtable {:?}",
            self.list_of_teachers, self.list_of_classes, hashtable
        )
        // format!("{:?}", self.list_of_teachers)
        // format!("{:?}",build_hashtable(self))
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
