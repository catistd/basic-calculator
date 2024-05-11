const OPERATORS: [char; 4] = ['+', '-', '*', '/'];

pub fn parse_equation(e: String) -> Result<f64, String> {
    //create empty variables
    let mut numbers: Vec<f64> = Vec::new();
    let mut operations: Vec<char> = Vec::new();
    let mut temp_num: String = "".to_string();

    //check for trailing operator
    if OPERATORS.contains(&e.chars().nth(e.len()-1).unwrap()) {
        return Err("error invalid syntax".to_string())
    }

    //split the equation into operators and numbers and put then in their respective vectors
    for c in e.chars() {
        if c.is_digit(10) || c == '.' {
            temp_num.push(c)
        } else if OPERATORS.contains(&c) {
            //check for leading operator
            if temp_num.is_empty() {
                return Err("error invalid syntax".to_string())
            }

            operations.push(c);

            //check for division by 0
            if operations.len() > 1 && operations[operations.len() - 1] == '/' && temp_num.parse::<f64>().unwrap() == 0.0 {
                return Err("error divide by 0".to_string())
            }

            //parse temp_num into a float and add it to numbers then clear for next number
            numbers.push(temp_num.parse::<f64>().unwrap());
            temp_num = "".to_string();
        }
    }

    if operations.is_empty() {
        return Ok(temp_num.parse::<f64>().unwrap())
    }

    //get last number
    if !temp_num.is_empty() {
        numbers.push(temp_num.parse::<f64>().unwrap());
        //check for division by 0
        if operations[operations.len() - 1] == '/' && temp_num.parse::<f64>().unwrap() == 0.0 {
            return Err("error divide by 0".to_string())
        }
    }

    //return answer
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

    //handle order of operations
    if operations.contains(&'*') || operations.contains(&'/') {
        numbers = calculate('*', '/', operations, numbers);
    } else if operations.contains(&'+') || operations.contains(&'-') {
        numbers = calculate('+', '-', operations, numbers);
    }

    numbers
}
