mod parser;

use std::collections::HashMap;

use crate::parser::parse;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        let program_path = &args[1];
        match std::fs::read_to_string(program_path) {
            Ok(code) => {
                let program = parse(code);
                let mut machine = Machine::<16>::default();
                machine.run(&program);
            },
            Err(err) => {
                eprintln!("Error reading program path: {}", err)
            },
        };
    } else {
        eprintln!("Expected a single path argument");
    }
}

pub struct Machine<'a, const N: usize> {
    registers: [Value; N],
    markers: HashMap<&'a str, usize>,
}

impl<'a, const N: usize> Default for Machine<'_, N> {
    fn default() -> Self {
        Self {
            registers: [Value::default(); N],
            markers: Default::default(),
        }
    }
}

impl<'a, const N: usize> Machine<'a, N> {
    fn run(&mut self, program: &'a [Instruction]) {
        self.collect_markers(program);
        let mut cursor = 0;
        while cursor < program.len() {
            let instruction = &program[cursor];
            match instruction {
                Instruction::Go {
                    cond,
                    marker1,
                    marker2,
                } => {
                    if self.registers[*cond].0 == 0 {
                        let Some(marker_location) = self.markers.get(marker1.as_str()) else {
                            panic!("No marker {} to jump to", marker1);
                        };
                        cursor = *marker_location;
                    } else {
                        let Some(marker_location) = self.markers.get(marker2.as_str()) else {
                            panic!("No marker {} to jump to", marker2);
                        };
                        cursor = *marker_location;
                    }
                }
                Instruction::Wrt { reg_target, value } => self.registers[*reg_target] = *value,
                Instruction::Cpy {
                    reg_src,
                    reg_target,
                } => self.registers[*reg_target] = self.registers[*reg_src],
                Instruction::Prt { reg_src } => println!("{}", self.registers[*reg_src].0),
                Instruction::Add {
                    reg_a,
                    reg_b,
                    reg_target,
                } => {
                    self.registers[*reg_target] =
                        Value(self.registers[*reg_a].0 + self.registers[*reg_b].0);
                }
                Instruction::Sub {
                    reg_a,
                    reg_b,
                    reg_target,
                } => {
                    self.registers[*reg_target] =
                        Value(self.registers[*reg_a].0 - self.registers[*reg_b].0)
                }
                Instruction::Marker(_) => {}
            }
            cursor += 1;
        }
    }

    fn collect_markers(&mut self, program: &'a [Instruction]) {
        let mut cursor = 0;
        while cursor < program.len() {
            let instruction = &program[cursor];
            if let Instruction::Marker(marker) = instruction {
                self.markers.insert(marker, cursor);
            }
            cursor += 1;
        }
    }
    pub fn registers_as_display_string(&self) -> String {
        format!("[{}]", self.registers.map(|r| r.0.to_string()).join(", "))
    }
}

impl From<Value> for usize {
    fn from(value: Value) -> Self {
        value.0
    }
}
#[derive(Copy, Debug, Clone, Default)]
pub struct Value(usize);

#[derive(Debug)]
pub enum Instruction {
    Marker(String),
    Go {
        cond: usize, // if register is 0, jump to marker1, else jump to marker2
        marker1: String,
        marker2: String,
    },
    Wrt {
        reg_target: usize,
        value: Value,
    },
    Cpy {
        reg_src: usize,
        reg_target: usize,
    },
    Prt {
        reg_src: usize,
    },
    Add {
        reg_a: usize,
        reg_b: usize,
        reg_target: usize,
    },
    Sub {
        reg_a: usize,
        reg_b: usize,
        reg_target: usize,
    },
}
