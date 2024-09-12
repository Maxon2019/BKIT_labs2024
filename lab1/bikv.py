import sys

def corni(a, b, c):
    #считаем корни би-уравнения
    res = []
    D = b*b - 4*a*c
    if D == 0.0:
        co = -b / (2.0*a)
        res.append(co)
        
    elif D > 0.0:
        D_sq = D**0.5
        co1 = (-b + D_sq) / (2.0*a)
        co2 = (-b - D_sq) / (2.0*a)
        res.append(co1)
        res.append(co2)
    return res

def read_c(index, phrase):
    try:
        # Пробуем cчитать коэффициенты из командной строки
        coef_str = sys.argv[index]
    except:
        # Вводим с клавиатуры
        coef_str = input(phrase)
    

    coef = float(coef_str)
    return coef


def main():
    a = read_c(1, 'Введите коэффициент А:')
    b = read_c(2, 'Введите коэффициент B:')
    c = read_c(3, 'Введите коэффициент C:')
    # Вычисление корней
    corn = corni(a,b,c)

    # Выводим корни
    num_corni = len(corn)
    if num_corni == 0:
        print('Нет корней')
    elif num_corni == 1:
        print(f'Один корень: {corn[0]}')
    elif num_corni == 2:
        print(f'Два корня: {corn[0]} и {corn[1]}')
    


if __name__ == "__main__":
    main()