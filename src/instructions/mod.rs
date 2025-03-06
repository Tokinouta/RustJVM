use crate::runtime_data_area::Frame;

mod bytecode_reader;
enum Instruction {
    NoOperand,
    Branch { offset: i16 },
    Index8 { index: u8 },
    Add,
    Sub,
    Mul,
    Div,
    Hlt,
}

impl Instruction {
    fn fetch_operands(&mut self, reader: &mut bytecode_reader::BytecodeReader) {
        match self {
            Self::NoOperand => {}
            Self::Branch { ref mut offset } => {
                *offset = reader.read_i16();
            }
            Self::Index8 { ref mut index } => {
                *index = reader.read_u8();
            }
            Self::Add => {}
            Self::Sub => {}
            Self::Mul => {}
            Self::Div => {}
            Self::Hlt => {}
        }
    }

    fn execute(Frame: &mut Frame) {
        todo!()
    }

    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Self::Add),
            1 => Some(Self::Sub),
            2 => Some(Self::Mul),
            3 => Some(Self::Div),
            4 => Some(Self::Hlt),
            _ => None,
        }
    }
}
