use std::{cell::RefCell, rc::Rc};

use crate::{
    attribute::Attribute,
    classfile::Field,
    instructions::{bytecode_reader::BytecodeReader, Instruction},
    runtime_data_area::{Frame, Thread},
};

pub fn interpret(method_info: Field) {
    let code_attr = method_info.get_code().unwrap();
    if let Attribute::Code {
        cp,
        max_stack,
        max_locals,
        code,
        exception_table,
        attributes,
    } = code_attr
    {
        let mut thread = Rc::new(RefCell::new(Thread::new()));
        let mut frame = Frame::new(
            *max_locals as usize,
            *max_stack as usize,
            Rc::downgrade(&thread),
        );
        thread.borrow_mut().push_frame(frame);
    }
}

pub fn loop_interpret(thread: Rc<RefCell<Thread>>, bytecode: Vec<u8>) {
    let frame = thread.borrow_mut().pop_frame();
    let mut reader = BytecodeReader::new(bytecode);
    loop {
        let pc = frame.next_pc();
        thread.borrow_mut().set_pc(pc);
        reader.reset(bytecode, pc);
        let opcode = reader.read_u8();
        let instruction = match opcode {
            0x00 => Instruction::Nop,
            0x01 => panic!("aconst_null"),
            0x02 => panic!("iconst_m1"),
            0x03 => panic!("iconst_0"),
            0x04 => panic!("iconst_1"),
            0x05 => panic!("iconst_2"),
            0x06 => panic!("iconst_3"),
            0x07 => panic!("iconst_4"),
            0x08 => panic!("iconst_5"),
            0x09 => panic!("lconst_0"),
            0x0a => panic!("lconst_1"),
            0x0b => panic!("fconst_0"),
            0x0c => panic!("fconst_1"),
            0x0d => panic!("fconst_2"),
            0x0e => panic!("dconst_0"),
            0x0f => panic!("dconst_1"),
            0x10 => panic!("bipush"),
            0x11 => panic!("sipush"),
            0x12 => panic!("ldc"),
            0x13 => panic!("ldc_w"),
            0x14 => panic!("ldc2_w"),
            0x15 => panic!("iload"),
            0x16 => panic!("lload"),
            0x17 => panic!("fload"),
            0x18 => panic!("dload"),
            0x19 => panic!("aload"),
            0x1a => panic!("iload_0"),
            0x1b => panic!("iload_1"),
            0x1c => panic!("iload_2"),
            0x1d => panic!("iload_3"),
            0x1e => panic!("lload_0"),
            0x1f => panic!("lload_1"),
            0x20 => panic!("lload_2"),
            0x21 => panic!("lload_3"),
            0x22 => panic!("fload_0"),
            0x23 => panic!("fload_1"),
            0x24 => panic!("fload_2"),
            0x25 => panic!("fload_3"),
            0x26 => panic!("dload_0"),
            _ => panic!("Unsupported opcode: {}", opcode),
        };
        println!("opcode: {}", opcode);
    }
}
