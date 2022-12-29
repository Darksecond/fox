use std::collections::HashMap;
use crate::parser::Stmt;
use fox_bytecode::*;
use fox_bytecode::memory::RESET_VECTOR;

struct Reference {
    label: String,
    index: usize,
}

pub struct Assembler {
    data: Vec<u8>,
    index: usize,
    length: usize,
    labels: HashMap<String, u32>,
    references: Vec<Reference>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            index: RESET_VECTOR as _,
            length: 0,
            labels: HashMap::new(),
            references: Vec::new(),
        }
    }

    pub fn assemble(&mut self, ast: &[Stmt]) {
        self.parse(ast);

        // Resolve references
        for reference in &self.references {
            let index = self.labels[&reference.label];

            let [a,b,c,d] = index.to_le_bytes();
            self.data[reference.index + 0] = a;
            self.data[reference.index + 1] = b;
            self.data[reference.index + 2] = c;
            self.data[reference.index + 3] = d;
        }

        eprintln!("Assembled in {} bytes, {} labels", self.length, self.labels.len());
    }

    fn parse(&mut self, ast: &[Stmt]) {
        for stmt in ast {
            match stmt {
                Stmt::OriginAbsolute(value) => {
                    self.index = *value as _;
                },
                Stmt::LiteralWord(value) => {
                        self.push_u8(OP_LITW);
                        self.push_u32(*value);
                },
                Stmt::LabelAbsolute(value) => {
                        self.labels.insert(value.to_string(), self.index as _);
                },
                Stmt::ReferenceAbsolute(value) => {
                        self.push_u8(OP_LITW);

                        self.references.push(Reference {
                            label: value.to_string(),
                            index: self.index,
                        });

                        self.push_u32(0);
                },
                Stmt::Operation(value) => {
                    self.push_u8(*value as _);
                },
                Stmt::String(value) => {
                    for ch in value.bytes() {
                        self.push_u8(ch);
                    }
                },
                Stmt::RawByte(value) => {
                    self.push_u8(*value);
                },
                Stmt::OriginRelative(value) => {
                    self.index += *value as usize;
                }
            }
        }
    }

    pub fn data(&self) -> &[u8] {
        let start = RESET_VECTOR as usize;
        let end = start + self.length;
        &self.data[start..end]
    }

    fn push_u8(&mut self, value: u8) {
        if self.data.len() < self.index + 1 {
            self.data.resize(self.index + 1, 0);
        }
        self.data[self.index] = value;
        self.index += 1;
        self.length = self.index - RESET_VECTOR as usize;
    }

    fn push_u32(&mut self, value: u32) {
        let [a,b,c,d] = value.to_le_bytes();
        self.push_u8(a);
        self.push_u8(b);
        self.push_u8(c);
        self.push_u8(d);
    }
}
