use lispers::lisp::{eval, Environment};
use lispers::parser::ExpressionStream;

fn main() {
    let programs = [
        "((lambda (x y) (+ (if (< x 10) (* x 11) x) y)) 2 20)",
        "(set 'myvar \"hello world!\")",
        "(print myvar) (print 'myvar)",
        "(car (cons 'a 'b)) (cdr (cons 'c 'd)) (cons 'a 'b)",
        "(eval (car (cons 'myvar 'b)))",
        "(set 'pow (lambda (a b) (if (= b 0) 1 (* a (pow a (- b 1))))))",
        "pow",
        "(pow 2 10)",
        "(let '((fib . (lambda (n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2))))))) (fib 10))",
        "(let '((a . (vec3 1 2 3)) (b . (vec3 4 5 6))) (vec3-dot (vec3-norm (vec3-add a b)) a))",
    ];

    let environment = Environment::default();

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
