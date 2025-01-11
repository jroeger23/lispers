use std::env;

use lispers::raytracer::lisp::mk_raytrace;
use lispers_core::lisp::environment::EnvironmentLayer;
use lispers_core::lisp::prelude::mk_prelude;
use lispers_core::lisp::{eval, Environment};
use lispers_core::parser::ExpressionStream;

fn main() {
    let program_paths: Vec<_> = env::args().skip(1).collect();
    let programs: Vec<_> = program_paths
        .iter()
        .map(|path| std::fs::read_to_string(path).unwrap())
        .collect();

    let mut layer = EnvironmentLayer::new();
    mk_prelude(&mut layer);
    mk_raytrace(&mut layer);

    let environment = Environment::from_layer(layer);

    for (i, r) in
        ExpressionStream::from_char_stream(programs.iter().map(|p| p.chars()).flatten()).enumerate()
    {
        match r {
            Err(err) => {
                println!("ParserError in Expression {}: {:?}", i + 1, err);
                break;
            }
            Ok(expr) => match eval(&environment, expr) {
                Ok(_) => {}
                Err(e) => println!("Error evaluating Expression {}: {}", i + 1, e),
            },
        }
    }

    println!("Interpreter Done!");
}
