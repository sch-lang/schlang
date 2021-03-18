fn main() {
    /*
    @TODO check whether or not to use jemalloc (https://github.com/jemalloc/jemalloc).
    It's being used by the Rust compiler so might make sense for us to look into it as well,
    but it will require a bit of research.
    */
    
    sch_driver::set_sigpipe_handler();
    sch_driver::main();
}