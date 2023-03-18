use calculator::{Calculator};
use plotters::prelude::*;
use std::{io, str::FromStr};
use meval::{eval_str, Expr};

mod calculator;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed_input = input.trim();
                if input.trim() == "graph" {
                    println!("Enter the expression to graph:");
                    let mut expression = String::new();
                    io::stdin().read_line(&mut expression)?;
                    graph(expression)?;
                } else {
                    let tokens = Calculator::parse(trimmed_input);
                    if tokens.is_err() {
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

fn graph(equation: String) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}
