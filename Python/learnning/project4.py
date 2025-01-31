class Customer:
    def __init__(self,name,balance):

        self.name = name
        self.balance = balance


    def getDeposit(self,amount):

        self.balance += amount
        return self.balance
    

    def getwithdraw(self,amount):

        self.balance -= amount
        return self.balance
    

    def getbalance(self):
        return self.balance
    

print("----- create account -----")
FS = input("create user name: ")
Pw = input("create password: ")

print("----- login -----")
logf = input("Enter your user name: ")
logp = input("Enter your password: ")

if logf == FS:
    ctr1 = Customer(FS,0)

    while True :

        option = int(input("0- To Deposite\n"
                        "1- To Whithdraw\n"
                        "2- To Check balance\n"
                        "3- Exit\n"
                        "Enter Your Choice: "))

        if option == 0:
        
            deposet_amount = float(input("Enter your deposit amount: "))
            current_balance = ctr1.getDeposit(deposet_amount)
            print(f"your carrent balance is {current_balance}")


        elif option == 1:
    
            withdraw_amount = float(input("Enter Your withdraw amount: "))
            if withdraw_amount > ctr1.getbalance():
                print("Enter withdraw amount less than or equal the current balance")
            else:
                current_balance = ctr1.getwithdraw(withdraw_amount)
                print(f"your carrent balance is {current_balance}")


        elif option == 2:
            current_balance = ctr1.getbalance()
            print(f"your carrent balance is {current_balance}")

    
        elif option == 3:
            break



        else:
            print("Enter correct choice")

else:
    print("Faild login")