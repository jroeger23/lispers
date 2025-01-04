use lispers::raytracer::lisp::mk_raytrace;
use lispers_core::lisp::environment::EnvironmentLayer;
use lispers_core::lisp::prelude::mk_prelude;
use lispers_core::lisp::{eval, Environment};
use lispers_core::parser::ExpressionStream;

fn main() {
    let programs = [
        "(vadd (vector 1 2 3) (vector 4 5 6))",
        "(set 'blue (material (color 0 0 1) (color 0 0 1) (color 0 0 0.6) 50 0.25))",
        "(set 'green (material (color 0 1 0) (color 0 1 0) (color 0 0.6 0) 50 0.25))",
        "(set 'white (material (color 1 1 1) (color 1 1 1) (color 0.6 0.6 0.6) 100 0.5))",
        "(set 'black (material (color 0 0 0) (color 0 0 0) (color 0.6 0.6 0.6) 100 0.5))",
        "(set 's1 (sphere (point 0 1 0) 1 blue))",
        "(set 's2 (sphere (point 2 0.5 2) 0.5 green))",
        "(set 'p1 (checkerboard (point 0 0 0) (vector 0 1 0) black white 0.5 (vector 0.5 0 1)))",
        "(set 'l1 (light (point 3 10 5) (color 1 1 1)))",
        "(set 'l2 (light (point 2 10 5) (color 1 1 1)))",
        "(set 'scn (scene (color 0.1 0.1 0.1) '(s1 s2 p1) '(l1 l2)))",
        "(set 'cam (camera (point 0 3 6) (point 0 0 0) (vector 0 1 0) 40 1920 1080))",
        "(render cam scn 5 4 \"rt-lisp-demo.png\")",
    ];

    let mut layer = EnvironmentLayer::new();
    mk_prelude(&mut layer);
    mk_raytrace(&mut layer);

    let environment = Environment::from_layer(layer);

    for r in ExpressionStream::from_char_stream(programs.iter().map(|p| p.chars()).flatten()) {
        match r {
            Err(err) => {
                println!("ParserError: {:?}", err);
                break;
            }
            Ok(expr) => {
                println!("Evaluating: {}", expr.clone());
                match eval(&environment, expr) {
                    Ok(e) => println!("=> {}", e),
                    Err(e) => println!("Error: {}", e),
                }
            }
        }
    }

    println!("Interpreter Done!");
}
