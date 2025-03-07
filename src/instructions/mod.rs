use std::{cell::RefCell, rc::Rc};

use crate::runtime_data_area::Frame;

mod bytecode_reader;
enum InstructionType {
    NoOperand,
    Branch { offset: i16 },
    Index8 { index: u8 },
}

trait InstructionOp {
    fn fetch_operands(&mut self, reader: &mut bytecode_reader::BytecodeReader);
    fn execute(Frame: &mut Frame);
}

impl InstructionOp for InstructionType {
    fn fetch_operands(&mut self, reader: &mut bytecode_reader::BytecodeReader) {
        match self {
            Self::NoOperand => {}
            Self::Branch { ref mut offset } => {
                *offset = reader.read_i16();
            }
            Self::Index8 { ref mut index } => {
                *index = reader.read_u8();
            }
        }
    }

    fn execute(Frame: &mut Frame) {
        todo!()
    }
}

#[rustfmt::skip]
enum Instruction {
    Nop,
    AconstNull,
    Iconst0, Iconst1, Iconst2, Iconst3, Iconst4, Iconst5, IconstM1,
    Lconst0, Lconst1, Fconst0, Fconst1, Fconst2, Dconst0, Dconst1, 
    Bipush(i8),
    Sipush(i16),
    Iload(u8), Iload0, Iload1, Iload2, Iload3,
    Lload(u8), Lload0, Lload1, Lload2, Lload3,
    Fload(u8), Fload0, Fload1, Fload2, Fload3,
    Dload(u8), Dload0, Dload1, Dload2, Dload3,
    Aload(u8), Aload0, Aload1, Aload2, Aload3,
    Iaload, Laload, Faload, Daload, Aaload,
    Pop, Pop2, Dup, DupX1, DupX2, Dup2, Dup2X1, Dup2X2,
    Swap,
}

impl Instruction {
    fn fetch_operands(&mut self, reader: &mut bytecode_reader::BytecodeReader) {
        match self {
            Self::Nop => {}
            Self::AconstNull => {}
            Self::IconstM1 => {}
            Self::Iconst0 => {}
            Self::Iconst1 => {}
            Self::Iconst2 => {}
            Self::Iconst3 => {}
            Self::Iconst4 => {}
            Self::Iconst5 => {}
            Self::Lconst0 => {}
            Self::Lconst1 => {}
            Self::Fconst0 => {}
            Self::Fconst1 => {}
            Self::Fconst2 => {}
            Self::Dconst0 => {}
            Self::Dconst1 => {}
            Self::Bipush(ref mut val) => *val = reader.read_i8(),
            Self::Sipush(ref mut val) => *val = reader.read_i16(),
            Self::Iload(ref mut index) => *index = reader.read_u8(),
            Self::Iload0 => {}
            Self::Iload1 => {}
            Self::Iload2 => {}
            Self::Iload3 => {}
            Self::Lload(ref mut index) => *index = reader.read_u8(),
            Self::Lload0 => {}
            Self::Lload1 => {}
            Self::Lload2 => {}
            Self::Lload3 => {}
            Self::Fload(ref mut index) => *index = reader.read_u8(),
            Self::Fload0 => {}
            Self::Fload1 => {}
            Self::Fload2 => {}
            Self::Fload3 => {}
            Self::Dload(ref mut index) => *index = reader.read_u8(),
            Self::Dload0 => {}
            Self::Dload1 => {}
            Self::Dload2 => {}
            Self::Dload3 => {}
            Self::Aload(ref mut index) => *index = reader.read_u8(),
            Self::Aload0 => {}
            Self::Aload1 => {}
            Self::Aload2 => {}
            Self::Aload3 => {}
            Self::Iaload => todo!(),
            Self::Laload => todo!(),
            Self::Faload => todo!(),
            Self::Daload => todo!(),
            Self::Aaload => todo!(),
            Self::Pop => {}
            Self::Pop2 => {}
            Self::Dup => {}
            Self::DupX1 => {}
            Self::DupX2 => {}
            Self::Dup2 => {}
            Self::Dup2X1 => {}
            Self::Dup2X2 => {}
            Self::Swap => {}
        }
    }

    fn execute(&mut self, frame: &mut Frame) {
        match self {
            Self::Nop => {}
            Self::AconstNull => frame.operand_stack.push_ref(None),
            Self::IconstM1 => frame.operand_stack.push_int(-1),
            Self::Iconst0 => frame.operand_stack.push_int(0),
            Self::Iconst1 => frame.operand_stack.push_int(1),
            Self::Iconst2 => frame.operand_stack.push_int(2),
            Self::Iconst3 => frame.operand_stack.push_int(3),
            Self::Iconst4 => frame.operand_stack.push_int(4),
            Self::Iconst5 => frame.operand_stack.push_int(5),
            Self::Lconst0 => frame.operand_stack.push_long(0),
            Self::Lconst1 => frame.operand_stack.push_long(1),
            Self::Fconst0 => frame.operand_stack.push_float(0.0),
            Self::Fconst1 => frame.operand_stack.push_float(1.0),
            Self::Fconst2 => frame.operand_stack.push_float(2.0),
            Self::Dconst0 => frame.operand_stack.push_double(0.0),
            Self::Dconst1 => frame.operand_stack.push_double(1.0),
            Self::Bipush(val) => frame.operand_stack.push_int(*val as i32),
            Self::Sipush(val) => frame.operand_stack.push_int(*val as i32),
            Self::Iload(index) => Self::iload(frame, *index),
            Self::Iload0 => Self::iload(frame, 0),
            Self::Iload1 => Self::iload(frame, 1),
            Self::Iload2 => Self::iload(frame, 2),
            Self::Iload3 => Self::iload(frame, 3),
            Self::Lload(index) => Self::lload(frame, *index),
            Self::Lload0 => Self::lload(frame, 0),
            Self::Lload1 => Self::lload(frame, 1),
            Self::Lload2 => Self::lload(frame, 2),
            Self::Lload3 => Self::lload(frame, 3),
            Self::Fload(index) => Self::fload(frame, *index),
            Self::Fload0 => Self::fload(frame, 0),
            Self::Fload1 => Self::fload(frame, 1),
            Self::Fload2 => Self::fload(frame, 2),
            Self::Fload3 => Self::fload(frame, 3),
            Self::Dload(index) => Self::dload(frame, *index),
            Self::Dload0 => Self::dload(frame, 0),
            Self::Dload1 => Self::dload(frame, 1),
            Self::Dload2 => Self::dload(frame, 2),
            Self::Dload3 => Self::dload(frame, 3),
            Self::Aload(index) => Self::aload(frame, *index),
            Self::Aload0 => Self::aload(frame, 0),
            Self::Aload1 => Self::aload(frame, 1),
            Self::Aload2 => Self::aload(frame, 2),
            Self::Aload3 => Self::aload(frame, 3),
            Self::Iaload => todo!(),
            Self::Laload => todo!(),
            Self::Faload => todo!(),
            Self::Daload => todo!(),
            Self::Aaload => todo!(),
            Self::Pop => {
                frame.operand_stack.pop_slot();
            }
            Self::Pop2 => {
                frame.operand_stack.pop_slot();
                frame.operand_stack.pop_slot();
            }
            Self::Dup => {
                let slot = frame.operand_stack.pop_slot();
                frame.operand_stack.push_slot(slot.clone());
                frame.operand_stack.push_slot(slot);
            }
            Self::DupX1 => {
                let slot1 = frame.operand_stack.pop_slot();
                let slot2 = frame.operand_stack.pop_slot();
                frame.operand_stack.push_slot(slot1.clone());
                frame.operand_stack.push_slot(slot2);
                frame.operand_stack.push_slot(slot1);
            }
            Self::DupX2 => {
                let slot1 = frame.operand_stack.pop_slot();
                let slot2 = frame.operand_stack.pop_slot();
                let slot3 = frame.operand_stack.pop_slot();
                frame.operand_stack.push_slot(slot1.clone());
                frame.operand_stack.push_slot(slot3);
                frame.operand_stack.push_slot(slot2);
                frame.operand_stack.push_slot(slot1);
            }
            Self::Dup2 => {
                let slot1 = frame.operand_stack.pop_slot();
                let slot2 = frame.operand_stack.pop_slot();
                frame.operand_stack.push_slot(slot2.clone());
                frame.operand_stack.push_slot(slot1.clone());
                frame.operand_stack.push_slot(slot2);
                frame.operand_stack.push_slot(slot1);
            }
            Self::Dup2X1 => {
                let slot1 = frame.operand_stack.pop_slot();
                let slot2 = frame.operand_stack.pop_slot();
                let slot3 = frame.operand_stack.pop_slot();
                frame.operand_stack.push_slot(slot2.clone());
                frame.operand_stack.push_slot(slot1.clone());
                frame.operand_stack.push_slot(slot3);
                frame.operand_stack.push_slot(slot2);
                frame.operand_stack.push_slot(slot1);
            }
            Self::Dup2X2 => {
                let slot1 = frame.operand_stack.pop_slot();
                let slot2 = frame.operand_stack.pop_slot();
                let slot3 = frame.operand_stack.pop_slot();
                let slot4 = frame.operand_stack.pop_slot();
                frame.operand_stack.push_slot(slot2.clone());
                frame.operand_stack.push_slot(slot1.clone());
                frame.operand_stack.push_slot(slot4);
                frame.operand_stack.push_slot(slot3);
                frame.operand_stack.push_slot(slot2);
                frame.operand_stack.push_slot(slot1);
            }
            Self::Swap => {
                let slot1 = frame.operand_stack.pop_slot();
                let slot2 = frame.operand_stack.pop_slot();
                frame.operand_stack.push_slot(slot1);
                frame.operand_stack.push_slot(slot2);
            }
        }
    }

    fn iload(frame: &mut Frame, index: u8) {
        let val = frame.local_vars.get_int(index as usize);
        frame.operand_stack.push_int(val);
    }

    fn lload(frame: &mut Frame, index: u8) {
        let val = frame.local_vars.get_long(index as usize);
        frame.operand_stack.push_long(val);
    }

    fn fload(frame: &mut Frame, index: u8) {
        let val = frame.local_vars.get_float(index as usize);
        frame.operand_stack.push_float(val);
    }

    fn dload(frame: &mut Frame, index: u8) {
        let val = frame.local_vars.get_double(index as usize);
        frame.operand_stack.push_double(val);
    }

    fn aload(frame: &mut Frame, index: u8) {
        let val = frame.local_vars.get_ref(index as usize);
        frame.operand_stack.push_ref(val);
    }
}
