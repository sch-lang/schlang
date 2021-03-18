use clap::{App, Arg};

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
        println!("Ran the compiler with file: {}", input_file);
    }
}
