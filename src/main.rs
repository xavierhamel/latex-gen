mod section;
mod table;
mod cell;

use std::fs::File;
use std::io::{self, BufRead};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    readfile(&args[1]).unwrap();
}

fn readfile(filename:&String) -> io::Result<()> {
    let file = File::open(filename)?;
    let mut sections = vec![];
    let mut current = section::Section::new("__SECTION__ Z".to_string());
    for line in io::BufReader::new(file).lines() {
        match line {
            Ok(l) => {
                if l.starts_with("__") {
                    match &l[2..l[1..].find("__").unwrap() + 1] {
                        "SECTION" => {
                            sections.push(current);
                            current = section::Section::new(l);
                        },
                        _ => {}
                    }
                } else {
                    current.parse_line(l);
                }
            },
            _ => {
            }
        }
    }
    sections.push(current);
    generate(sections);

    Ok(())
}

fn generate(sections: Vec<section::Section>) {
    println!("====== HEADER ======");
    println!("\\usepackage[shortlabels]{{enumitem}}");
    println!("\\usepackage[xfp]{{spreadtab}}");
    println!("\\usepackage{{tabu}}");
    println!("====== BODY ======");
    println!("\\begin{{enumerate}}[a) ]");
    for section in &sections[1..] {
        section.generate();
    }
    println!("\\end{{enumerate}}");
}