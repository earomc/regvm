use crate::{
    Instruction::{self},
    Value,
};

#[derive(Debug)]
enum Token {
    Marker(String),
    Number(usize),
    Go,
    Wrt,
    Cpy,
    Add,
    Sub,
    Prt,
}

pub fn parse(code: String) -> Vec<Instruction> {
    // LEXING:
    let value = &mut String::new();
    let mut tokens = Vec::new();

    #[derive(Debug)]
    enum LexingState {
        String,
        Number,
        Comment,
        None,
    }
    let mut state = LexingState::None;
    for char in code.chars() {
        //println!("char: {:?}, state: {:?}", char, &state);
        if let LexingState::Comment = state {
            if !matches!(char, '\n' | '\r') {
                continue;
            } else {
                state = LexingState::None;
            }
        }
        match char {
            'a'..='z' | 'A'..='Z' | '_' => {
                state = LexingState::String;
                value.push(char);
            }
            '0'..='9' => {
                state = LexingState::Number;
                value.push(char);
            }
            ';' => {
                state = LexingState::Comment;
            }
            ' ' | '\t' | '\n' | '\r' => {
                match state {
                    LexingState::String => match value.to_ascii_lowercase().as_str() {
                        "go" => tokens.push(Token::Go),
                        "cpy" => tokens.push(Token::Cpy),
                        "add" => tokens.push(Token::Add),
                        "prt" => tokens.push(Token::Prt),
                        "wrt" => tokens.push(Token::Wrt),
                        "sub" => tokens.push(Token::Sub),
                        _ => {
                            tokens.push(Token::Marker(value.clone()));
                        }
                    },
                    LexingState::Number => {
                        let number = value.parse().expect("Expected integer");
                        tokens.push(Token::Number(number));
                    }
                    LexingState::None | LexingState::Comment => {
                        continue;
                    }
                }
                value.clear();
                state = LexingState::None;
                continue;
            }
            _ => {
                panic!("invalid character")
            }
        }
    }

    // PARSING:
    let mut program = Vec::new();
    let mut tokens = tokens.into_iter();
    loop {
        let Some(token) = tokens.next() else {
            break;
        };
        dbg!(&token);
        match token {
            Token::Marker(marker) => {
                program.push(Instruction::Marker(marker));
                continue;
            }
            Token::Go => {
                if let Some(Token::Number(cond)) = tokens.next() {
                    if let Some(Token::Marker(marker1)) = tokens.next() {
                        if let Some(Token::Marker(marker2)) = tokens.next() {
                            program.push(Instruction::Go { marker1, marker2, cond });
                        }
                    }
                }
            }
            Token::Prt => {
                if let Some(Token::Number(number)) = tokens.next() {
                    program.push(Instruction::Prt { reg_src: number });
                }
            }
            Token::Cpy => {
                if let Some(Token::Number(reg_src)) = tokens.next() {
                    if let Some(Token::Number(reg_target)) = tokens.next() {
                        program.push(Instruction::Cpy {
                            reg_src,
                            reg_target,
                        });
                    }
                }
            }
            Token::Add => {
                if let Some(Token::Number(reg_a)) = tokens.next() {
                    if let Some(Token::Number(reg_b)) = tokens.next() {
                        if let Some(Token::Number(reg_target)) = tokens.next() {
                            program.push(Instruction::Add {
                                reg_a,
                                reg_b,
                                reg_target,
                            });
                        }
                    }
                }
            }
            Token::Sub => {
                if let Some(Token::Number(reg_a)) = tokens.next() {
                    if let Some(Token::Number(reg_b)) = tokens.next() {
                        if let Some(Token::Number(reg_target)) = tokens.next() {
                            program.push(Instruction::Sub {
                                reg_a,
                                reg_b,
                                reg_target,
                            });
                        }
                    }
                }
            }
            Token::Wrt => {
                if let Some(Token::Number(reg_target)) = tokens.next() {
                    if let Some(Token::Number(value)) = tokens.next() {
                        program.push(Instruction::Wrt {
                            reg_target,
                            value: Value(value),
                        });
                    }
                }
            }
            Token::Number(_) => panic!("unexpected number"),
        }
    }
    program
}
