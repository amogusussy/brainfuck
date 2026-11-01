use std::fmt;

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum Instructions {
    RightPointer,
    LeftPointer,
    Incremement,
    Decrement,
    Output,
    Input,
    StartWhile,
    EndWhile,
}

impl fmt::Display for Instructions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instructions::RightPointer => {write!(f, "RightPointer")},
            Instructions::LeftPointer => {write!(f, "LeftPointer")},
            Instructions::Incremement => {write!(f, "Incremement")},
            Instructions::Decrement => {write!(f, "Decrement")},
            Instructions::Output => {write!(f, "Output")},
            Instructions::Input => {write!(f, "Input")},
            Instructions::StartWhile => {write!(f, "StartWhile")},
            Instructions::EndWhile => {write!(f, "EndWhile")},
        }
    }
}


pub fn lexer(program: &str) -> Vec<Instructions> {
    let mut output: Vec<Instructions> = Vec::new();

    let mut skip_to_newline: bool = false;
    for (_, chr) in program.as_bytes().iter().enumerate() {
        if skip_to_newline && (chr != &b'\n') {
            continue
        }
        skip_to_newline = false;
        let instruction = match chr {
            b' ' | b'\t' | b'\n' => continue,
            b'>' => Instructions::RightPointer,
            b'<' => Instructions::LeftPointer,
            b'+' => Instructions::Incremement,
            b'-' => Instructions::Decrement,
            b'.' => Instructions::Output,
            b',' => Instructions::Input,
            b'[' => Instructions::StartWhile,
            b']' => Instructions::EndWhile,
            _ => {
                skip_to_newline = false;
                continue;
            }
        };
        // dbg!(&instruction);
        output.push(instruction);
    }

    output
}
