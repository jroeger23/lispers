mod lisp;
mod parser;
use lisp::expression::{eval_prelude, Expression};

fn main() {
    let mut test = "(add 10 (sub 1.1 200.5)) (concat-if true \"true\" 'nil (a . b))".chars();

    let mut tkns = parser::tokenizer::tokenize(&mut test);

    while let Some(tk) = tkns.next() {
        println!("{:?}", tk);
    }

    let expr: Expression = vec![
        vec![
            Expression::Symbol("lambda".to_string()),
            vec![
                Expression::Symbol("x".to_string()),
                Expression::Symbol("y".to_string()),
            ]
            .into(),
            vec![
                Expression::Symbol("if".to_string()),
                vec![
                    Expression::Symbol("==".to_string()),
                    Expression::Symbol("x".to_string()),
                    Expression::Integer(5),
                ]
                .into(),
                vec![
                    Expression::Symbol("add".to_string()),
                    Expression::Symbol("x".to_string()),
                    Expression::Symbol("y".to_string()),
                ]
                .into(),
                Expression::String("x is not 5".to_string()),
            ]
            .into(),
        ]
        .into(),
        Expression::Integer(5),
        vec![
            Expression::Symbol("let".to_string()),
            vec![Expression::Cell(
                Box::new(Expression::Symbol("y".to_string())),
                Box::new(Expression::Integer(7)),
            )]
            .into(),
            Expression::Symbol("y".to_string()),
        ]
        .into(),
    ]
    .into();

    println!("{:?} evaluates to {:?}", expr.clone(), eval_prelude(expr));
}
