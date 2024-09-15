from lab_python_oop.rectangle import Rectangle
from lab_python_oop.circle import Circle
from lab_python_oop.square import Square
from colorama import Fore, Back, Style


N = 7

def main():
    rectangle = Rectangle(N, N, "blue")
    circle = Circle(N, "green")
    square = Square(N, "red")

    # print('{}\n {}\n {}\n'\
    # .format(rectangle,circle,square))
    print(Fore.RED + Back.WHITE+'{}\n {}\n {}'\
    .format(rectangle,circle,square))
    print(Style.RESET_ALL)



if __name__ == "__main__":
    main()