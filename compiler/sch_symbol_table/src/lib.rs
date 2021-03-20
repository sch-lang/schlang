use std::collections::HashMap;
use sch_interface::{self, interface};

/*
We need to define a symbol table that can store information
about the individual tokens. This is an example of what the
symbol table will need to keep track of. 
This can of course change over time.
*/

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug, Hash)]
pub struct SymbolIdentifier(usize);

/*enum DataType {
    INT,
    BOOL,
    STRING,
    FLOAT,
    NONE
}

struct SymbolEntry {
    value: String,
    data_type: DataType, 
    line: u32, // what line is this in the source code
    column_index: u32, // where in the column does the word start
    column_offset: u8 // how long is the word
}*/

#[derive(Copy, Clone, Debug)]
pub struct SymbolEntry {}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: HashMap<SymbolIdentifier, SymbolEntry>
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new()
        }
    }

    pub fn add_entry(&mut self, value: String) -> SymbolIdentifier {
        let id = self.symbols.len();
        let mut characters: Vec<char> = Vec::new();

        for i in value.as_bytes().iter() {
            characters.push(*i as char);
        }

        self.symbols.insert(SymbolIdentifier(id), SymbolEntry {
        });

        return SymbolIdentifier(id);
    }

    pub fn get_symbol(&self, identifier: &SymbolIdentifier) -> Result<SymbolEntry, interface::ErrorReported> {
        if let Some(entry) = self.symbols.get(&identifier) {
            return Ok(*entry);
        } else {
            return Err(interface::ErrorReported);
        }
    }
}