use calculator::Calculator;
use plotters::prelude::*;
use std::io;
use meval::eval_str;
use open;

mod calculator;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        println!("Enter an expression: (Type 'graph' to graph a function)");
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                let tokens = Calculator::parse(trimmed_input);

                if trimmed_input == "graph" {
                    prompt_and_graph()?;
                } else {
                    let error = tokens.as_ref().err();
                    if let Some(err) = error {
                        if let calculator::Error::Equation = err  {
                            prompt_and_graph()?;
                            continue;
                        } else if let calculator::Error::BadeToken('x') = err {
                            println!("Did you mean to graph?");
                            prompt_and_graph()?;
                            continue;
                        }
                        println!("{:?}", tokens.err().unwrap());
                        continue;
                    }
                    let expr = Calculator::expression(tokens?);
                    if let Some(v) = Calculator::evaluate(expr) {
                        println!("{}", v);
                    }
                }
            },
            Err(error) => println!("error: {}", error)
        }
    }
}

fn prompt_and_graph() -> Result<(), Box<dyn std::error::Error>> {
    println!("Enter the expression to graph: (Simply type the equation without y, f(x), etc. e.g. x + 2)");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression)?;
    let mut graph_type: Option<String> = None;
    loop {
        println!("Choose the type of graph:");
        println!("1. Regular graph");
        println!("2. Graph with integral");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice)?;
        let choice = choice.trim();

        if choice == "1" || choice == "2" {
            graph_type = Some(choice.to_string());
            break;
        } else {
            println!("Invalid choice, please try again.");
        }
    }

    match graph_type {
        Some(ref t) if t == "1" => graph(expression, None, None),
        Some(ref t) if t == "2" => {
            println!("Enter the lower bound of the integral:");
            let mut a = String::new();
            io::stdin().read_line(&mut a)?;
            let a = a.trim().parse::<f64>()?;

            println!("Enter the upper bound of the integral:");
            let mut b = String::new();
            io::stdin().read_line(&mut b)?;
            let b = b.trim().parse::<f64>()?;

            graph(expression, Some(a), Some(b))
        }
        _ => unreachable!(),
    }
}

fn graph(equation: String, a: Option<f64>, b: Option<f64>) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("graph.png", (WIDTH, HEIGHT)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Graph of the function", ("Arial", 24).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f64..10.0f64, 0f64..10.0f64)?;

    chart.configure_mesh().draw()?;

    let f = |x: f64| eval_str(&equation.replace("x", &x.to_string())).unwrap();

    chart.draw_series(LineSeries::new(
        (0..=100)
            .map(|i| -10.0 + i as f64 / 5.0)
            .map(|x| (x, f(x))),
        &BLUE,
    ))?;

    if let (Some(a), Some(b)) = (a, b) {
        let integral_area = AreaSeries::new(
            (-100..=100)
                .map(|i| -10.0 + i as f64 / 5.0)
                .filter(|&x| x >= a && x <= b)
                .map(|x| (x, f(x))),
            0.0,
            &RED.mix(0.3),
        );
        chart.draw_series(integral_area)?;
    }

    root.present().unwrap();
    open::that("graph.png")?;

    Ok(())
}
