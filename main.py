from substitution_gen_lib_rs import * 


school = School()

#artificial routine construction
teacher1 = Teacher(f"Ashutosh",f"physics",False)
teacher2 = Teacher(f"Tonoy",f"chemistry",True)
teacher3 = Teacher(f"Tamali",f"computer",True)
teacher4 = Teacher(f"Arnab",f"physics",True)

Class1 = Class("11A")
Class2 =  Class("11B")


Class1.add_teacher(teacher1,1) #phsyics
Class1.add_teacher(teacher2,2)
Class1.add_teacher(teacher3,3)

Class2.add_teacher(teacher3,1)
Class2.add_teacher(teacher1,2)
Class2.add_teacher(teacher4,3) #physics



school.add_class(Class1)
school.add_class(Class2)


# school.add_class(Class2)


school.generate_substitutions()

print(school)