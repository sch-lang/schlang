# Schlang Compiler

This is the Schlang Programming Language compiler. It's build in Rust, and compiles you Schlang code to binary machine code that can run on all Unix-based systems.

### Compiler internals

```
+------------------+
|   Preprocessor   |
+------------------+
|     Compiler     |
+------------------+
|     Assembler    |
+------------------+
|   Linker/Loader  |
+------------------+
```

#### Preprocessor


