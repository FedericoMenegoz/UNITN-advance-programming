use std::ops::Add;

enum Result {
    Ok(i32),
    Error(String),
}
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Expression {
    Number(i32),
    Operation(Box<Expression>, Operation, Box<Expression>),
}

pub fn es10() {
    let exp: Expression = Expression::Operation(
        Box::new(Expression::Operation(
            Box::new(Expression::Number(23)),
            Operation::Add,
            Box::new(Expression::Number(12)),
        )),
        Operation::Sub,
        Box::new(Expression::Operation(
            Box::new(Expression::Number(45)),
            Operation::Mul,
            Box::new(Expression::Operation(
                Box::new(Expression::Number(0)),
                Operation::Div,
                Box::new(Expression::Number(1)),
            )),
        )),
    );
    print_exp(&exp);
    println!();
    println!(
        "{}",
        match evaluate_expression(exp) {
            Result::Ok(n) => n.to_string(),
            Result::Error(s) => s,
        }
    );
}
fn print_exp(exp: &Expression) {
    match exp {
        Expression::Number(val) => {
            print!("{}", val)
        }
        Expression::Operation(exp1, op, exp2) => {
            print_exp(exp1);
            match op {
                Operation::Add => print!(" + "),
                Operation::Div => print!(" \\ "),
                Operation::Mul => print!(" * "),
                Operation::Sub => print!(" - "),
            }
            print_exp(exp2);
        }
    }
}
fn evaluate_expression(exp: Expression) -> Result {
    match exp {
        Expression::Number(number) => Result::Ok(number),
        Expression::Operation(left, op, right) => {
            match (evaluate_expression(*left), evaluate_expression(*right)) {
                (Result::Ok(left_val), Result::Ok(right_val)) => match op {
                    Operation::Add => Result::Ok(left_val + right_val),
                    Operation::Div => {
                        if right_val != 0 {
                            Result::Ok(left_val / right_val)
                        } else {
                            Result::Error("Can not divide per 0!".to_string())
                        }
                    }
                    Operation::Mul => Result::Ok(left_val * right_val),
                    Operation::Sub => Result::Ok(left_val - right_val),
                },
                (Result::Ok(_), Result::Error(s)) => Result::Error(s),
                (Result::Error(s), Result::Ok(_)) => Result::Error(s),
                (Result::Error(s), Result::Error(t)) => Result::Error(s.add(" ").add(&t)),
            }
        }
    }
}
