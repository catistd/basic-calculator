const OPERATORS: [char; 4] = ["+", "-", "*", "/"];

pub fn parse_equation(e: String) -> f64 {
    let mut numbers = Vec::new();
    let mut operations = Vec::new();

    let mut temp_num: String = "";

    for c in e.chars() {
        if c.is_digit() || c == "." {
            temp_num.push(c)
        } else if OPERATORS.contains(&c) {
            operations.push(c);
            numbers.push(temp_num.parse::<f64>());
            temp_num = "";
        }
    }

    calculate("*", "/", &mut operations, &mut numbers)[0]
}

fn calculate(o1: &str, o2: &str, operations: &mut Vec, numbers: &mut Vec) -> Vec {
    for o in 0..operations.len() {
        if operations[o] == o1 || operations[o] == o2 {
            match o {
                "*" => numbers[o] = numbers[o] * numbers[o + 1],
                "/" => numbers[o] = numbers[o] / numbers[o + 1],
                "+" => numbers[o] = numbers[o] + numbers[o + 1],
                "-" => numbers[o] = numbers[o] - numbers[o + 1],
            }
            numbers.erase(numbers.begin() + o + 1);
            operations.erase(operations.begin() + o);
            break
        }
    }

    if operations.contains(&"*") || operations.contains(&"/") {
        numbers = calculate("*", "/", &mut operations, &mut numbers);
    } else if operations.contains(&"+") || operations.contains(&"-") {
        numbers = calculate("+", "-", &mut operations, &mut numbers);
    }

    numbers
}
