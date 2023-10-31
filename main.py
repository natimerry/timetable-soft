from substitution_gen_lib_rs import * 
import csv
import os
import argparse
from pathlib import Path
import pandas as pd
DIR = ""

school = School()

list_of_teachers = {}
def read_teachers():
    global school,DIR,list_of_teachers

    with open(Path(DIR+"teacher_list.csv")) as file:
          teacher_data = csv.DictReader(file)

          for lines in teacher_data:
            teacher = Teacher(lines["teacher"],lines["subject"],
                              lines["present"].lower() in ['true', '1', 't', 'y', 'yes', 'yeah', 'yup'])
            
            school.add_teacher(teacher)
            list_of_teachers[lines['teacher']] = teacher


def get_periods(day:str):
    read_teachers()
    global DIR,list_of_teachers,school
    path = Path(DIR+day)
    for cls in os.listdir(path):
        with open(Path(str(path)+'/'+cls)) as file:
            cur_class = Class(cls.removesuffix(".csv"))
            class_data = csv.DictReader(file)
            for row in class_data:
                register_period(list_of_teachers[row['teacher']],int(row['period']),school,cur_class)
            
            school.add_class(cur_class)

        

if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--directory",help="Directory containing input data",required=True)
    parser.add_argument("--day",help="Day of the week to generate substitutions for",required=True)
    args = parser.parse_args()

    DIR = args.directory
    get_periods(args.day)
    with open("output.csv","w+") as output_csv:
        new_timetable = school.generate_time_table()
        print(new_timetable)
        output_csv.write(new_timetable.split("\n\n")[0])
    
    data_format = pd.read_csv('output.csv')
    html_table = data_format.to_html()
    with open("output.html","w+") as html:
        html.write(html_table)
