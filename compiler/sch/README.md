### `sch`

This is the entrypoint of the Schlang compiler.

The `sch` package will initiate the `sch_driver` which is responsible for the entire compilation process.
`sch` will provide OS bindings so that the compiler can abort gracefully with proper clean up.