use std::collections::HashMap;
use std::path::Path;

use lispers::raytracer::lisp::mk_raytrace;
use lispers_core::lisp::environment::EnvironmentLayer;
use lispers_core::lisp::prelude::mk_prelude;
use lispers_core::lisp::{eval, Environment};
use lispers_core::parser::ExpressionStream;

const SCENES_DIR: &str = env!("SCENES_DIR");

fn main() {
    println!("Loading scenes from directory: {}", SCENES_DIR);

    let mut scenes = HashMap::new();
    for e in std::fs::read_dir(Path::new(SCENES_DIR)).expect("Failed to read scenes directory") {
        let e = e.expect("Failed to read scene file");
        let t = e.file_type().expect("Failed to read scene file type");
        let n = e
            .file_name()
            .into_string()
            .expect("Failed to read scene file name");
        if t.is_file() && n.starts_with("demo-") && n.ends_with(".lisp") {
            scenes.insert(n, e);
        }
    }

    let mut layer = EnvironmentLayer::new();
    mk_prelude(&mut layer);
    mk_raytrace(&mut layer);
    let environment = Environment::from_layer(layer);

    let args: Vec<_> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <scene-file.lisp>", args[0]);
        println!("Available scene files:");
        for name in scenes.keys() {
            println!("  {}", name);
        }
        return;
    }

    for r in ExpressionStream::from_char_stream(
        std::fs::read_to_string(scenes.get(&args[1]).expect("Scene file not found").path())
            .expect("Failed to read scene file")
            .chars(),
    ) {
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
