import sub_processor
teacher = sub_processor.Teacher("LOL","physics")
teacher.add_period(1,2,'C')

print(teacher)
print(teacher.get_sub())
print(teacher.name)

# school = sub_processor.School()
# school.add_teacher(teacher)
# del teacher
# print(school.teacher_by_subjects("physics"))