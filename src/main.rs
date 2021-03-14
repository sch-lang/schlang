use clap::{App, Arg};
use std::fs;

pub mod lexical_analysis;
pub mod lexical_analyzer;

fn main() {
    let matches = App::new("Schlang Compiler")
        .version("1.0.0")
        .author("Fredrik Stave <fredrik.stave@schibsted.com>")
        .about("A compiler for the Schlang programming language")
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file for the compiler")
                .required(true)
                .index(1)
        )
        .arg(
            Arg::new("verbose")
                .about("Sets the level of verbosity")
                .short('v')
                .long("verbose")
        )
        .get_matches();

    if let Some(input_file) = matches.value_of("INPUT") {
        let contents = fs::read_to_string(input_file)
            .expect("Could not read input file");

        let lexical_analyzer_instance = lexical_analyzer::LexicalAnalyzer::new(&contents);

        lexical_analyzer_instance.remove_comments();
    }
    
    if let Some(_) = matches.value_of("verbose") {
        println!("Verbosity is enabled");
    }
}
