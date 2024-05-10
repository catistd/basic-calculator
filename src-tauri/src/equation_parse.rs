const OPERATORS: [char; 4] = ['+', '-', '*', '/'];

pub fn parse_equation(e: String) -> f64 {
    let mut numbers: Vec<f64> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    let mut temp_num: String = "".to_string();

    for c in e.chars() {
        if c.is_digit(10) || c == '.' {
            temp_num.push(c)
        } else if OPERATORS.contains(&c) {
            operations.push(c);
            numbers.push(temp_num.parse::<f64>().unwrap());
            temp_num = "".to_string();
        }
    }

    calculate('*', '/', operations, numbers)[0]
}

fn calculate(o1: char, o2: char, mut operations: Vec<char>, mut numbers: Vec<f64>) -> Vec<f64> {
    for o in 0..operations.len() {
        if operations[o] == o1 || operations[o] == o2 {
            match operations[o] {
                '*' => numbers[o] = numbers[o] * numbers[o + 1],
                '/' => numbers[o] = numbers[o] / numbers[o + 1],
                '+' => numbers[o] = numbers[o] + numbers[o + 1],
                '-' => numbers[o] = numbers[o] - numbers[o + 1],
                _ => numbers[o] = numbers[o] + numbers[o + 1]
            }
            numbers.remove(o + 1);
            operations.remove(o);
            break
        }
    }

    if operations.contains(&'*') || operations.contains(&'/') {
        numbers = calculate('*', '/', operations, numbers);
    } else if operations.contains(&'+') || operations.contains(&'-') {
        numbers = calculate('+', '-', operations, numbers);
    }

    numbers
}
