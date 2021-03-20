use clap::{App, Arg};
use std::fs;
use std::panic::{self, catch_unwind};
use std::process::{self};
use tracing::debug;
use sch_interface::{self, interface};
use sch_symbol_table::{self, SymbolTable};

/// Exit status code used for successful compilation and help output.
pub const EXIT_SUCCESS: i32 = 0;

/// Exit status code used for compilation failures and invalid flags.
pub const EXIT_FAILURE: i32 = 1;

pub struct RunCompiler {
    // @TODO keep track of the compiler state
    buffer: Vec<char>,
    symbol_table: SymbolTable
}

impl RunCompiler {
    pub fn new(input: String) -> Self {
        let mut buffer: Vec<char> = Vec::new();
        
        for byte in input.as_bytes().iter() {
            buffer.push(*byte as char);
        }

        Self {
            symbol_table: SymbolTable::new(),
            buffer
        }
    }
    pub fn run(self) -> interface::Result<()> {
        run_compiler(self.buffer, self.symbol_table)
    }
}

// @TODO implement the compilation logic
fn run_compiler(buffer: Vec<char>, mut symbol_table: SymbolTable) -> interface::Result<()> {
    // Run the lexer to get a token stream
    let mut analyzer = sch_lexer::LexicalAnalyzer::new(buffer);

    let id = symbol_table.add_entry(String::from("awesome"));
    let entry = symbol_table.get_symbol(&id);

    match entry {
        Ok(e) => println!("{:?}", e),
        Err(_) => println!("Error occured"),
    }

    println!("{:?}", analyzer.get_next_token());
    println!("{:?}", analyzer.get_next_token());

    return Ok(());
}

#[cfg(unix)]
pub fn set_sigpipe_handler() {
    unsafe {
        // Set the SIGPIPE signal handler, so that an EPIPE
        // will cause sch to terminate, as expected.
        assert_ne!(libc::signal(libc::SIGPIPE, libc::SIG_DFL), libc::SIG_ERR);
    }
}

#[cfg(windows)]
pub fn set_sigpipe_handler() {}

pub fn catch_fatal_errors<F: FnOnce() -> R, R>(function: F) -> Result<R, interface::ErrorReported> {
    catch_unwind(panic::AssertUnwindSafe(function)).map_err(|value| {
        if value.is::<interface::FatalErrorMarker>() {
            interface::ErrorReported
        } else {
            panic::resume_unwind(value);
        }
    })
}

pub fn catch_with_exit_code(function: impl FnOnce() -> interface::Result<()>) -> i32 {
    let result = catch_fatal_errors(function).and_then(|result| result);

    match result {
        Ok(()) => EXIT_SUCCESS,
        Err(_) => EXIT_FAILURE,
    }
}

pub fn main() -> ! {
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

    let exit_code = catch_with_exit_code(|| {
        if let Some(input_file) = matches.value_of("INPUT") {
            let contents = fs::read_to_string(input_file)
                .expect("Could not read input file");
            RunCompiler::new(contents).run()
        } else {
            Err(interface::ErrorReported)
        }
    });

    process::exit(exit_code);
}