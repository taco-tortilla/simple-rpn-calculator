use std::io;

fn main() {
    println!("============================");
    println!("Enter any expression in RPN");
    println!("ex) 1 2 + 3 *");
    println!("Exit with ctrl + c");
    println!("============================");

    let lines = io::stdin().lines();
    for line in lines{
        let mut stack = vec![];
        let mut error = false;

        if let Ok(s) = line {
            let words:Vec<_> = s.split(" ").collect();

            for word in words{
                if let Ok(parsed) = word.parse::<i32>(){
                    stack.push(parsed);
                } else {
                    match word {
                        "+" => add(&mut stack, &mut error),
                        "-" => sub(&mut stack, &mut error),
                        "*" => mul(&mut stack, &mut error),
                        "/" => div(&mut stack, &mut error),
                        _ => {
                            println!("Error: {word:?} is an invalid.");
                            error = true;
                        }
                    }
                }
                if error {
                    break;
                }
            }

            if error {
                println!("----------------------------");
                continue;
            }

            if stack.len() != 1 {
                println!("You did not do the calculations...");
                println!("Your input was {stack:?}");
            } else {
                println!("result: {}", stack[0])
            }
            println!("----------------------------")
        }
    }
}

fn add(stack: &mut Vec<i32>, error: &mut bool){
    if is_stack_valid(stack) { 
        *error = true;
        return;
    };
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    stack.push(left + right);
}

fn sub(stack: &mut Vec<i32>, error: &mut bool) {
    if is_stack_valid(stack) { 
        *error = true;
        return;
    };
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    stack.push(left - right);
}

fn div(stack: &mut Vec<i32>, error: &mut bool) {
    if is_stack_valid(stack) { 
        *error = true;
        return;
    };
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    if right == 0 {
        println!("Error: Division by zero");
        *error = true;
        return;
    }
    stack.push(left / right);
}

fn mul(stack: &mut Vec<i32>, error: &mut bool) {
    if is_stack_valid(stack) { 
        *error = true;
        return;
    };
    let right = stack.pop().unwrap();
    let left = stack.pop().unwrap();
    stack.push(left * right);
}

fn is_stack_valid(stack: &mut Vec<i32>) -> bool{
    if stack.len() < 2{
        println!("Error: Not following rpn description");
        return true;
    }else{
        return false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive() {
        let mut stack: Vec<i32> = vec![1, 2];
        let mut error = false;
        add(&mut stack, &mut error);
        assert_eq!(stack, vec![3]);
        assert!(!error);
    }

    #[test]
    fn test_add_nagative() {
        let mut stack: Vec<i32> = vec![-7, -2];
        let mut error = false;
        add(&mut stack, &mut error);
        assert_eq!(stack, vec![-9]);
        assert!(!error);
    }

    #[test]
    fn test_sub_positive() {
        let mut stack: Vec<i32> = vec![1, 2];
        let mut error = false;
        sub(&mut stack, &mut error);
        assert_eq!(stack, vec![-1]);
        assert!(!error);
    }

    #[test]
    fn test_sub_nagative() {
        let mut stack: Vec<i32> = vec![-7, 2];
        let mut error = false;
        sub(&mut stack, &mut error);
        assert_eq!(stack, vec![-9]);
        assert!(!error);
    }

    #[test]
    fn test_div_positive() {
        let mut stack: Vec<i32> = vec![4, 2];
        let mut error = false;
        div(&mut stack, &mut error);
        assert_eq!(stack, vec![2]);
        assert!(!error);
    }

    #[test]
    fn test_div_nagative() {
        let mut stack: Vec<i32> = vec![-5, -2];
        let mut error = false;
        div(&mut stack, &mut error);
        assert_eq!(stack, vec![2]);
        assert!(!error);
    }

    #[test]
    fn test_div_zero() {
        let mut stack: Vec<i32> = vec![5, 0];
        let mut error = false;
        div(&mut stack, &mut error);
        assert!(stack.is_empty());
        assert!(error);
    }

    #[test]
    fn test_mul_positive() {
        let mut stack: Vec<i32> = vec![4, 2];
        let mut error = false;
        mul(&mut stack, &mut error);
        assert_eq!(stack, vec![8]);
        assert!(!error);
    }

    #[test]
    fn test_mul_nagative() {
        let mut stack: Vec<i32> = vec![-5, -2];
        let mut error = false;
        mul(&mut stack, &mut error);
        assert_eq!(stack, vec![10]);
        assert!(!error);
    }
}