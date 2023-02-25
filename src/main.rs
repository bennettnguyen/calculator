mod calculator;

fn main() {
    let tokens = calculator::Calculator::parse("2 * 2 + 48 / 4");
    println!("{:?}", tokens);
    let expr = calculator::Calculator::expression(tokens.unwrap());
    println!("{:?}", expr);
    let value = calculator::Calculator::evaluate(expr);
    println!("{}", value.unwrap());
}
