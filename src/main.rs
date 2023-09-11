use chrono::Local;
use clap::{Arg, Command};
use regex::{Captures, Regex};
use std::fs;

pub fn time() -> String {
    let date = Local::now();
    date.format("%Y-%m-%d").to_string()
}

// GET ALL MD FILE THAT INCLUDE ONE OF THE STRING INPUTED YOU PUT MULTIPLE STRING AS INPUT AS ONCE
pub fn find_docs(terms: rhai::Array) -> rhai::Array {
    let terms: Vec<String> = terms.iter().map(|t| t.clone().cast::<String>()).collect();
    let mut found: rhai::Array = Vec::new();

    for entry in glob::glob("./**/*.md")
        .unwrap()
        .filter(|p| !p.as_ref().unwrap().to_str().unwrap().ends_with(".comp.md"))
    {
        if let Ok(path) = entry {
            let contents = std::fs::read_to_string(&path).unwrap();

            for term in &terms {
                if contents.contains::<&String>(term) {
                    found.push(path.to_str().unwrap().into());
                    break;
                }
            }
        }
    }

    found.into()
}

pub fn find_link_references(current_file: &str) -> rhai::Array {
    let re = Regex::new(r"\[[^]]*\]\(([^)]*)\)").unwrap();

    let mut referenced_from: rhai::Array = Vec::new();

    for entry in glob::glob("./**/*.md")
        .unwrap()
        .filter(|p| !p.as_ref().unwrap().to_str().unwrap().ends_with(".comp.md"))
    {
        if let Ok(path) = entry {
            let content = fs::read_to_string(&path).unwrap();

            for cap in re.captures_iter(&content) {
                let link = cap[1].to_string();
                if link == current_file {
                    referenced_from.push(path.to_str().unwrap().into());
                }
            }
        }
    }

    referenced_from
}

fn substitue_code(markdown: &str, current_file: &str, filename: &str) -> String {
    let re = Regex::new(r"```mcomp\s*([\s\S]*?)\s*```").unwrap();
    re.replace_all(markdown, |caps: &Captures| {
        let code = caps.get(1).unwrap().as_str();
        // execute code and get result
        let result = execute_code(code, current_file, filename);
        result
    })
    .to_string()
}

fn process_file(input_path: &str) -> std::io::Result<()> {
    let output_path = input_path.trim_end_matches(".comp.md").to_owned() + ".md";

    let input = fs::read_to_string(input_path)?.replace("\r\n", "\n");

    let output = substitue_code(
        &input,
        std::path::PathBuf::from(output_path.clone())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap(),
        std::path::PathBuf::from(output_path.clone())
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    fs::File::create(&output_path)?.set_len(0)?; // truncate file
    fs::write(&output_path, &output)?;

    Ok(())
}

// CLI
use rhai::{Engine, Scope};

fn execute_code(code: &str, current_file: &str, filename: &str) -> String {
    let mut engine = Engine::new();
    let mut scope = Scope::new();
    scope.push_constant("current_file", current_file.to_owned().clone());
    scope.push_constant("filename", filename.to_owned().clone());
    // register any modules or functions
    engine.register_fn("find_backlink", find_link_references);
    engine.register_fn("time", time);
    engine.register_fn("find_docs", find_docs);

    let result = engine.eval_with_scope::<String>(&mut scope, code).unwrap();

    result
}
fn main() {
    let matches = Command::new("mcomp")
        .arg(
            Arg::new("input")
                .help("Input markdown file")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();

    process_file(&input_file).unwrap();
}
