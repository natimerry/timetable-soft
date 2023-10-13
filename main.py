from substitution_gen_lib_rs import * 


school = School()

#artificial routine construction
teacher_phy1 = Teacher(f"Ashutosh",f"physics",False)
teacher_phy2 = Teacher(f"Arnab",f"physics",True)


teacher_chem1 = Teacher(f"Chem1",f"chemistry",True)
teacher_chem2 = Teacher(f"Chem2",f"chemistry",True)

teacher_cs1 = Teacher(f"Cs1",f"computer",True)
teacher_cs2 = Teacher(f"Cs2",f"computer",False)

teacher_maths2 = Teacher(f"maths2",f"maths",False)
teacher_maths1 = Teacher(f"maths1",f"maths",True)


Class1 = Class("11A")
Class2 =  Class("11B")
Class3 =  Class("11C")


#class1
register_period(teacher_phy1,1,school,Class1)
register_period(teacher_chem2,2,school,Class1)
register_period(teacher_maths2,3,school,Class1)

register_period(teacher_phy1 ,1,school,Class2)
register_period(teacher_maths1,2,school,Class2)
# register_period(teacher_cs1,3,school,Class2)
register_period(teacher_phy1,4,school,Class2)

register_period(teacher_cs2,1,school,Class3)
register_period(teacher_phy2,2,school,Class3)
register_period(teacher_phy1,3,school,Class3)


school.add_class(Class1)
school.add_class(Class2)
school.add_class(Class3)


# school.add_class(Class2)


# school.generate_substitutions()

print(school.generate_time_table())
print(school)

# print(school.generate_substitutions())
# print(Class1)