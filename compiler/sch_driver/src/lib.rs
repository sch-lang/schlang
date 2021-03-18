use std::panic::{self, catch_unwind};
use std::process::{self};
use std::time::Instant;
use tracing::debug;

/* @FIX Local declarations to be moved later */
// Useful type to use with `Result<>` indicate that an error has already
// been reported to the user, so no need to continue checking.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct FatalErrorMarker;

pub mod interface;
/* End of local declarations */

/// Exit status code used for successful compilation and help output.
pub const EXIT_SUCCESS: i32 = 0;

/// Exit status code used for compilation failures and invalid flags.
pub const EXIT_FAILURE: i32 = 1;

pub struct RunCompiler {
    // @TODO keep track of the compiler state
}

impl RunCompiler {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(self) -> interface::Result<()> {
        run_compiler()
    }
}

// @TODO implement the compilation logic
fn run_compiler() -> interface::Result<()> {
    debug!("Successfully ran the `run_compiler` function");
    
    // return Err(ErrorReported);
    return Ok(());
}

#[cfg(unix)]
pub fn set_sigpipe_handler() {
    unsafe {
        // Set the SIGPIPE signal handler, so that an EPIPE
        // will cause rustc to terminate, as expected.
        assert_ne!(libc::signal(libc::SIGPIPE, libc::SIG_DFL), libc::SIG_ERR);
    }
}

#[cfg(windows)]
pub fn set_sigpipe_handler() {}

pub fn catch_fatal_errors<F: FnOnce() -> R, R>(function: F) -> Result<R, interface::ErrorReported> {
    catch_unwind(panic::AssertUnwindSafe(function)).map_err(|value| {
        if value.is::<FatalErrorMarker>() {
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
    let start_time = Instant::now();
    let exit_code = catch_with_exit_code(|| {
        RunCompiler::new().run()
    });

    debug!("Started at: {:?}", start_time);
    println!("Everything is ay okay!");

    process::exit(exit_code);
}