use std::io;
use std::f64;
//для решения квадратного
fn solve_quadratic(a: f64, b: f64, c: f64) -> (Option<f64>, Option<f64>) {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        (None, None) // Нет действительных корней
    } else if discriminant == 0.0 {
        let y1 = -b / (2.0 * a);
        (Some(y1), None) // Один действительный корень
    } else {
        let sqrt_d = discriminant.sqrt();
        let y1 = (-b + sqrt_d) / (2.0 * a);
        let y2 = (-b - sqrt_d) / (2.0 * a);
        (Some(y1), Some(y2)) // Два действительных корня
    }
}

fn solve_biquadratic(a: f64, b: f64, c: f64) {
    let (y1, y2) = solve_quadratic(a, b, c);

    if let Some(y) = y1 {
        if y >= 0.0 {
            let x1 = y.sqrt();
            let x2 = -x1;
            println!("x1 = {}", x1);
            println!("x2 = {}", x2);
        }
    }

    if let Some(y) = y2 {
        if y >= 0.0 {
            let x3 = y.sqrt();
            let x4 = -x3;
            println!("x3 = {}", x3);
            println!("x4 = {}", x4);
        }
    }

    if y1.is_none() && y2.is_none() {
        println!("Нет действительных корней");
    }
}

fn read_coefficient(name: &str) -> f64 {
    loop {
        println!("Введите коэффициент {}:", name);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не удалось прочитать строку");

        match input.trim().parse::<f64>() {
            Ok(value) => return value,
            Err(_) => println!("Пожалуйста, введите правильное число."),
        }
    }
}

fn main() {
    // Ввод коэффициентов пользователем
    let a = read_coefficient("a");
    let b = read_coefficient("b");
    let c = read_coefficient("c");

    // Решаем биквадратное уравнение
    solve_biquadratic(a, b, c);
}