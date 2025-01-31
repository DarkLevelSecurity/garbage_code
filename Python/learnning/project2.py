#!/bin/python3
import math

def getFactorial(number):
    result = math.factorial(number)
    return result

def getPrime(number):
    if number <= 1:
        print(f"The number {number} is not Prim")
    else:
        for num in range(2,number):
            if number % num == 0:
                print(f"The number {number} is not Prim")
                break
        else:
            print(f"The number {number} is Prim")

def getPN(number):
    if float((number)) >= 0:
        print(f"The number {number} is Positive")
    else:
        print(f"The number {number} is negetive")

def OddEven(number):
    if number %2 == 0:
        print(f"the number {number} is Even")
    else:
        print(f"the number {number} is Odd")


number = int(input("Enter Number: "))

while True:
    option = int(input("1- Chicking prime \n"
          "2- Chicking Factorial \n"
          "3- Chicking positive or negitive \n"
          "4- Chicking Even or Odd \n"
          "5- Exit \n"
          "Enter your chois: "))  
    if option == 1:
        getPrime(number)
    elif option == 2:
        getFactorial(number)
        res = getFactorial(number)
        print(f"The factorial of the number {number} is {res}")
    elif option == 3:
        getPN(number)
    elif option == 4: 
        OddEven(number)
    elif option == 5:
            break
    else:
        print("Wrong")
