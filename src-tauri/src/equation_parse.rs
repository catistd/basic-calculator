const OPERATORS: [char; 4] = ['+', '-', '*', '/'];

pub fn parse_equation(e: String) -> Result<f64, String> {
    let mut numbers: Vec<f64> = Vec::new();
    let mut operations: Vec<char> = Vec::new();

    let mut temp_num: String = "".to_string();

    if OPERATORS.contains(&e.chars().nth(e.len()-1).unwrap()) {
        return Err("error invalid syntax".to_string())
    }

    for c in e.chars() {
        if c.is_digit(10) || c == '.' {
            temp_num.push(c)
        } else if OPERATORS.contains(&c) {
            if temp_num.is_empty() {
                return Err("error invalid syntax".to_string())
            }
            if c == '/' && temp_num.parse::<f64>().unwrap() == 0.0 {
                return Err("error divide by 0".to_string());
            }
            operations.push(c);
            numbers.push(temp_num.parse::<f64>().unwrap());
            temp_num = "".to_string();
        }
    }

    if !temp_num.is_empty() {
        numbers.push(temp_num.parse::<f64>().unwrap());
    }

    Ok(calculate('*', '/', operations, numbers)[0])
}

fn calculate(o1: char, o2: char, mut operations: Vec<char>, mut numbers: Vec<f64>) -> Vec<f64> {
    for o in 0..operations.len() {
        if operations[o] == o1 || operations[o] == o2 {
            match operations[o] {
                '*' => numbers[o] = numbers[o] * numbers[o + 1],
                '/' => numbers[o] = numbers[o] / numbers[o + 1],
                '+' => numbers[o] = numbers[o] + numbers[o + 1],
                '-' => numbers[o] = numbers[o] - numbers[o + 1],
                _ => panic!("error, invalid operator")
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
