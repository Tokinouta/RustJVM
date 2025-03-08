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
    Iadd, Ladd, Fadd, Dadd, 
    Isub, Lsub, Fsub, Dsub, 
    Ineg, Lneg, Fneg, Dneg,
    Imul, Lmul, Fmul, Dmul, 
    Idiv, Ldiv, Fdiv, Ddiv, 
    Irem, Lrem, Frem, Drem,
    Ishl, Lshl, Ishr, Lshr, Iushr, Lushr,
    Iand, Land, Ior, Lor, IXor, Lxor,
    Iinc(u8, i8),
    I2l, I2f, I2d,
    L2i, L2f, L2d,
    F2i, F2l, F2d,
    D2i, D2l, D2f,
    // I2b, I2c, I2s,
    LCmp,
    FCmpL, FCmpG,
    DCmpL, DCmpG,
    IfEq(i16), IfNe(i16), IfLt(i16), IfLe(i16), IfGt(i16), IfGe(i16),
    IficmpEq(i16), IficmpNe(i16), IficmpLt(i16), IficmpLe(i16), IficmpGt(i16), IficmpGe(i16),
    IfacmpEq(i16), IfacmpNe(i16),
    Goto(i16),
    TableSwitch {
        default_offset: i32,
        low: i32,
        high: i32,
        offsets: Vec<i32>,
    },
    LookupSwitch{
        default_offset: i32,
        npairs: i32,
        match_offsets: Vec<i32>,
    },
    Return,
    GetStatic,
    PutStatic,
    GetField,
    PutField
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
            Self::Iadd => {}
            Self::Ladd => {}
            Self::Fadd => {}
            Self::Dadd => {}
            Self::Isub => {}
            Self::Lsub => {}
            Self::Fsub => {}
            Self::Dsub => {}
            Self::Ineg => {}
            Self::Lneg => {}
            Self::Fneg => {}
            Self::Dneg => {}
            Self::Imul => {}
            Self::Lmul => {}
            Self::Fmul => {}
            Self::Dmul => {}
            Self::Idiv => {}
            Self::Ldiv => {}
            Self::Fdiv => {}
            Self::Ddiv => {}
            Self::Irem => {}
            Self::Lrem => {}
            Self::Frem => {}
            Self::Drem => {}
            Self::Ishl => {}
            Self::Lshl => {}
            Self::Ishr => {}
            Self::Lshr => {}
            Self::Iushr => {}
            Self::Lushr => {}
            Self::Iand => {}
            Self::Land => {}
            Self::Ior => {}
            Self::Lor => {}
            Self::IXor => {}
            Self::Lxor => {}
            Self::Iinc(ref mut index, ref mut const_num) => {
                *index = reader.read_u8();
                *const_num = reader.read_i8();
            }
            Self::I2l => {}
            Self::I2f => {}
            Self::I2d => {}
            Self::L2i => {}
            Self::L2f => {}
            Self::L2d => {}
            Self::F2i => {}
            Self::F2l => {}
            Self::F2d => {}
            Self::D2i => {}
            Self::D2l => {}
            Self::D2f => {}
            Self::LCmp => {}
            Self::FCmpL => {}
            Self::FCmpG => {}
            Self::DCmpL => {}
            Self::DCmpG => {}
            Self::IfEq(ref mut offset) => *offset = reader.read_i16(),
            Self::IfNe(ref mut offset) => *offset = reader.read_i16(),
            Self::IfLt(ref mut offset) => *offset = reader.read_i16(),
            Self::IfLe(ref mut offset) => *offset = reader.read_i16(),
            Self::IfGt(ref mut offset) => *offset = reader.read_i16(),
            Self::IfGe(ref mut offset) => *offset = reader.read_i16(),
            Self::IficmpEq(ref mut offset) => *offset = reader.read_i16(),
            Self::IficmpNe(ref mut offset) => *offset = reader.read_i16(),
            Self::IficmpLt(ref mut offset) => *offset = reader.read_i16(),
            Self::IficmpGe(ref mut offset) => *offset = reader.read_i16(),
            Self::IficmpGt(ref mut offset) => *offset = reader.read_i16(),
            Self::IficmpLe(ref mut offset) => *offset = reader.read_i16(),
            Self::IfacmpEq(ref mut offset) => *offset = reader.read_i16(),
            Self::IfacmpNe(ref mut offset) => *offset = reader.read_i16(),
            Self::Goto(ref mut offset) => *offset = reader.read_i16(),
            Self::TableSwitch {
                default_offset,
                low,
                high,
                offsets,
            } => {
                while reader.pc() % 4 != 0 {
                    reader.read_u8();
                }
                *default_offset = reader.read_i32();
                *low = reader.read_i32();
                *high = reader.read_i32();
                for _ in *low..=*high {
                    let offset = reader.read_i32();
                    offsets.push(offset);
                }
            }
            Self::LookupSwitch {
                default_offset,
                npairs,
                match_offsets,
            } => {
                while reader.pc() % 4 != 0 {
                    reader.read_u8();
                }
                *default_offset = reader.read_i32();
                *npairs = reader.read_i32();
                for _ in 0..*npairs * 2 {
                    let offset = reader.read_i32();
                    match_offsets.push(offset);
                }
            }
            Self::Return => {}
            Self::GetStatic => {}
            Self::PutStatic => {}
            Self::GetField => {}
            Self::PutField => {}
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
            Self::Iadd => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 + val2);
            }
            Self::Ladd => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 + val2);
            }
            Self::Fadd => {
                let val2 = frame.operand_stack.pop_float();
                let val1 = frame.operand_stack.pop_float();
                frame.operand_stack.push_float(val1 + val2);
            }
            Self::Dadd => {
                let val2 = frame.operand_stack.pop_double();
                let val1 = frame.operand_stack.pop_double();
                frame.operand_stack.push_double(val1 + val2);
            }
            Self::Isub => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 - val2);
            }
            Self::Lsub => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 - val2);
            }
            Self::Fsub => {
                let val2 = frame.operand_stack.pop_float();
                let val1 = frame.operand_stack.pop_float();
                frame.operand_stack.push_float(val1 - val2);
            }
            Self::Dsub => {
                let val2 = frame.operand_stack.pop_double();
                let val1 = frame.operand_stack.pop_double();
                frame.operand_stack.push_double(val1 - val2);
            }
            Self::Imul => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 * val2);
            }
            Self::Lmul => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 * val2);
            }
            Self::Fmul => {
                let val2 = frame.operand_stack.pop_float();
                let val1 = frame.operand_stack.pop_float();
                frame.operand_stack.push_float(val1 * val2);
            }
            Self::Dmul => {
                let val2 = frame.operand_stack.pop_double();
                let val1 = frame.operand_stack.pop_double();
                frame.operand_stack.push_double(val1 * val2);
            }
            Self::Idiv => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val2 == 0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_int(val1 / val2);
            }
            Self::Ldiv => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                if val2 == 0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_long(val1 / val2);
            }
            Self::Fdiv => {
                let val2 = frame.operand_stack.pop_float();
                let val1 = frame.operand_stack.pop_float();
                if val2 == 0.0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_float(val1 / val2);
            }
            Self::Ddiv => {
                let val2 = frame.operand_stack.pop_double();
                let val1 = frame.operand_stack.pop_double();
                if val2 == 0.0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_double(val1 / val2);
            }
            Self::Ineg => {
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(-val1);
            }
            Self::Lneg => {
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(-val1);
            }
            Self::Fneg => {
                let val1 = frame.operand_stack.pop_float();
                frame.operand_stack.push_float(-val1);
            }
            Self::Dneg => {
                let val1 = frame.operand_stack.pop_double();
                frame.operand_stack.push_double(-val1);
            }
            Self::Irem => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val2 == 0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_int(val1 % val2);
            }
            Self::Lrem => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                if val2 == 0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_long(val1 % val2);
            }
            Self::Frem => {
                let val2 = frame.operand_stack.pop_float();
                let val1 = frame.operand_stack.pop_float();
                if val2 == 0.0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_float(val1 % val2);
            }
            Self::Drem => {
                let val2 = frame.operand_stack.pop_double();
                let val1 = frame.operand_stack.pop_double();
                if val2 == 0.0 {
                    panic!("java.lang.ArithmeticException: / by zero");
                }
                frame.operand_stack.push_double(val1 % val2);
            }
            Self::Ishl => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 << (val2 & 0x1f));
            }
            Self::Lshl => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 << (val2 & 0x3f));
            }
            Self::Ishr => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 >> (val2 & 0x1f));
            }
            Self::Lshr => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 >> (val2 & 0x3f));
            }
            Self::Iushr => {
                let val2 = frame.operand_stack.pop_int() as u32;
                let val1 = frame.operand_stack.pop_int() as u32;
                frame.operand_stack.push_int((val1 << (val2 & 0x1f)) as i32);
            }
            Self::Lushr => {
                let val2 = frame.operand_stack.pop_long() as u64;
                let val1 = frame.operand_stack.pop_long() as u64;
                frame
                    .operand_stack
                    .push_long((val1 >> (val2 & 0x3f)) as i64);
            }
            Self::Iand => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 & val2);
            }
            Self::Land => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 & val2);
            }
            Self::Ior => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 | val2);
            }
            Self::Lor => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 | val2);
            }
            Self::IXor => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                frame.operand_stack.push_int(val1 ^ val2);
            }
            Self::Lxor => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                frame.operand_stack.push_long(val1 ^ val2);
            }
            Self::Iinc(index, const_num) => {
                let val = frame.local_vars.get_int(*index as usize);
                frame
                    .local_vars
                    .set_int(*index as usize, val + (*const_num as i32));
            }
            Self::I2l => {
                let val = frame.operand_stack.pop_int();
                frame.operand_stack.push_long(val as i64);
            }
            Self::I2f => {
                let val = frame.operand_stack.pop_int();
                frame.operand_stack.push_float(val as f32);
            }
            Self::I2d => {
                let val = frame.operand_stack.pop_int();
                frame.operand_stack.push_double(val as f64);
            }
            Self::L2i => {
                let val = frame.operand_stack.pop_long();
                frame.operand_stack.push_int(val as i32);
            }
            Self::L2f => {
                let val = frame.operand_stack.pop_long();
                frame.operand_stack.push_float(val as f32);
            }
            Self::L2d => {
                let val = frame.operand_stack.pop_long();
                frame.operand_stack.push_double(val as f64);
            }
            Self::F2i => {
                let val = frame.operand_stack.pop_float();
                frame.operand_stack.push_int(val as i32);
            }
            Self::F2l => {
                let val = frame.operand_stack.pop_float();
                frame.operand_stack.push_long(val as i64);
            }
            Self::F2d => {
                let val = frame.operand_stack.pop_float();
                frame.operand_stack.push_double(val as f64);
            }
            Self::D2i => {
                let val = frame.operand_stack.pop_double();
                frame.operand_stack.push_int(val as i32);
            }
            Self::D2l => {
                let val = frame.operand_stack.pop_double();
                frame.operand_stack.push_long(val as i64);
            }
            Self::D2f => {
                let val = frame.operand_stack.pop_double();
                frame.operand_stack.push_float(val as f32);
            }
            Self::LCmp => {
                let val2 = frame.operand_stack.pop_long();
                let val1 = frame.operand_stack.pop_long();
                let val = if val1 > val2 {
                    1
                } else if val1 == val2 {
                    0
                } else {
                    -1
                };
                frame.operand_stack.push_int(val);
            }
            Self::FCmpG => Self::fcmp(frame, true),
            Self::FCmpL => Self::fcmp(frame, false),
            Self::DCmpG => Self::dcmp(frame, true),
            Self::DCmpL => Self::dcmp(frame, false),
            Self::IfEq(offset) => {
                let val = frame.operand_stack.pop_int();
                if val == 0 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IfNe(offset) => {
                let val = frame.operand_stack.pop_int();
                if val != 0 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IfLt(offset) => {
                let val = frame.operand_stack.pop_int();
                if val < 0 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IfLe(offset) => {
                let val = frame.operand_stack.pop_int();
                if val <= 0 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IfGt(offset) => {
                let val = frame.operand_stack.pop_int();
                if val > 0 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IfGe(offset) => {
                let val = frame.operand_stack.pop_int();
                if val >= 0 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IficmpEq(offset) => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val1 == val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IficmpNe(offset) => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val1 != val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IficmpLt(offset) => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val1 < val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IficmpGt(offset) => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val1 > val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IficmpLe(offset) => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val1 <= val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IficmpGe(offset) => {
                let val2 = frame.operand_stack.pop_int();
                let val1 = frame.operand_stack.pop_int();
                if val1 >= val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IfacmpEq(offset) => {
                let val2 = frame.operand_stack.pop_ref();
                let val1 = frame.operand_stack.pop_ref();
                if val1 == val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::IfacmpNe(offset) => {
                let val2 = frame.operand_stack.pop_ref();
                let val1 = frame.operand_stack.pop_ref();
                if val1 != val2 {
                    Self::branch(frame, *offset);
                }
            }
            Self::Goto(offset) => Self::branch(frame, *offset),
            Self::TableSwitch {
                default_offset,
                low,
                high,
                offsets,
            } => {
                let index = frame.operand_stack.pop_int();
                if index >= *low && index <= *high {
                    let offset = offsets[index as usize - *low as usize];
                    Self::branch(frame, offset as i16);
                } else {
                    Self::branch(frame, *default_offset as i16);
                }
            }
            Self::LookupSwitch {
                default_offset,
                npairs,
                match_offsets,
            } => {
                let index = frame.operand_stack.pop_int();
                for i in 0..*npairs {
                    if match_offsets[i as usize * 2] == index {
                        let offset = match_offsets[i as usize * 2 + 1];
                        Self::branch(frame, offset as i16);
                    }
                }
                Self::branch(frame, *default_offset as i16);
            }
            Self::Return => todo!(),
            Self::GetStatic => todo!(),
            Self::PutStatic => todo!(),
            Self::GetField => todo!(),
            Self::PutField => todo!(),
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

    fn fcmp(frame: &mut Frame, gflag: bool) {
        let val2 = frame.operand_stack.pop_float();
        let val1 = frame.operand_stack.pop_float();
        let result = if val1 > val2 {
            1
        } else if val1 == val2 {
            0
        } else if val1 < val2 {
            -1
        } else if gflag {
            1
        } else {
            -1
        };
        frame.operand_stack.push_int(result);
    }

    fn dcmp(frame: &mut Frame, gflag: bool) {
        let val2 = frame.operand_stack.pop_double();
        let val1 = frame.operand_stack.pop_double();
        let result = if val1 > val2 {
            1
        } else if val1 == val2 {
            0
        } else if val1 < val2 {
            -1
        } else if gflag {
            1
        } else {
            -1
        };
        frame.operand_stack.push_int(result);
    }

    fn branch(frame: &mut Frame, offset: i16) {
        let pc = frame.thread().pc();
        // if pc + offset < 0 || pc + offset > frame.method().code_len() as i32 {
        //     panic!("branch out of range")
        // }
        frame.thread().set_pc(pc + offset as usize);
        todo!()
    }
}
