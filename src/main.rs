mod fun;
mod practice;
mod snippets;

use snippets::*;


use std::{env::args, collections::HashMap};


#[derive(PartialEq, Debug)]
enum ArgResult {
    Help,
    Snippet(String),
    Category(String),
}
use ArgResult::*;

fn main() -> Result<(), ()> {
    let args = get_args();
    let mut funcs = HashMap::new();
    let categories = HashMap::from([practice(&mut funcs), fun(&mut funcs)]);

    if args.len() == 0 {
        for (name, func) in funcs {
            run_func(&name, func);
        }
    } else {
        for arg in args {
            if arg == Help {
                println!(
                    "Syntax: beta [-opts]+
-h          \tShows this message
-c[category]\tExecutes all snippets of category [category]
[name]      \tExecutes snippet [name]

Categories:
{}",
                    categories
                        .iter()
                        .map(|(name, names)| format!(
                            "\t{}\n{}",
                            name,
                            names
                                .iter()
                                .map(|n| format!("\t\t{}", n))
                                .collect::<Vec<_>>()
                                .join("\n")
                        ))
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
                return Ok(());
            } else if let Snippet(ref name) = arg {
                if funcs.contains_key(name) {
                    run_func(name, funcs[name]);
                } else {
                    return Err(println!(
                        "Unknown snippet '{}'.
Valid Snippets:
{}",
                        name,
                        funcs
                            .keys()
                            .map(|k| format!("\t{}", k))
                            .collect::<Vec<_>>()
                            .join("\n")
                    ));
                }
            } else if let Category(ref category) = arg {
                if categories.contains_key(category) {
                    for name in &categories[category] {
                        run_func(name, funcs[name]);
                    }
                } else {
                    return Err(println!(
                        "Unknown category '{}'.
Valid Categories:
{}",
                        category,
                        categories
                            .keys()
                            .map(|k| format!("\t{}", k))
                            .collect::<Vec<_>>()
                            .join("\n")
                    ));
                }
            }
        }
    }

    Ok(())
}

fn get_args() -> Vec<ArgResult> {
    let args = args().into_iter().skip(1);
    if args.len() == 0 {
        return Vec::new();
    }

    args.map(|s| {
        if s == "-h" || s == "--help" {
            Help
        } else if s.starts_with("-c") {
            Category(s[2..].into())
        } else {
            Snippet(s)
        }
    })
    .collect()
}

fn run_func(name: &String, func: fn()) {
    println!("\n\nExecuting [{}]", name);
    func();
    println!("Finished: [{}]", name);
}
