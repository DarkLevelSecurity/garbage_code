class Studetnt:
    def __init__(self,first_name,last_name):
        self.first_name = first_name
        self.last_name = last_name
        self.email = first_name + "." + last_name + "@universty.com"

FN = input("Enter your first name:  ")
FL = input("Enter your last name:  ")


stu1 = Studetnt(FN,FL)
print(stu1.email)