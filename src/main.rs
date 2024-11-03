mod parser;

fn main() {
    let mut test = "(add 10 (sub 1.1 200.5)) (concat-if true \"true\" 'nil (a . b))".chars();

    let mut tkns = parser::tokenizer::tokenize(&mut test);

    while let Some(tk) = tkns.next() {
        println!("{:?}", tk);
    }
}
