use std::cell::RefCell;
use std::rc::Rc;

use crate::loader::ConstPool;
// struct Attribute {
//     name: String,
//     data: Vec<u8>,
// }

pub struct ExceptionTable {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_type: u16,
}

impl ExceptionTable {
    pub fn new(start_pc: u16, end_pc: u16, handler_pc: u16, catch_type: u16) -> Self {
        Self {
            start_pc,
            end_pc,
            handler_pc,
            catch_type,
        }
    }
}

pub struct LineNumberTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl LineNumberTableEntry {
    pub fn new(start_pc: u16, line_number: u16) -> Self {
        Self {
            start_pc,
            line_number,
        }
    }
}

pub struct LocalVariableTableEntry {
    start_pc: u16,
    line_number: u16,
}

impl LocalVariableTableEntry {
    pub fn new(start_pc: u16, line_number: u16) -> Self {
        Self {
            start_pc,
            line_number,
        }
    }
}

pub enum Attribute {
    ConstantValue(u16),
    Code {
        cp: Rc<RefCell<ConstPool>>,
        max_stack: u16,
        max_locals: u16,
        code: Vec<u8>,
        exception_table: Vec<ExceptionTable>,
        attributes: Vec<Attribute>,
    },
    Exceptions {
        exception_index_table: Vec<u16>,
    },
    SourceFile {
        cp: Rc<RefCell<ConstPool>>,
        index: u16,
    },
    LineNumberTable {
        line_number_table: Vec<LineNumberTableEntry>,
    },
    LocalVariableTable{
        local_variable_table: Vec<LocalVariableTableEntry>,
    },
    InnerClasses,
    Synthetic,
    Deprecated,
    EnclosingMethod,
    Signature,
    SourceDebugExtension,
    LocalVariableTypeTable,
    RuntimeVisibleAnnotations,
    RuntimeInvisibleAnnotations,
    RuntimeVisibleParameterAnnotations,
    RuntimeInvisibleParameterAnnotations,
    AnnotationDefault,
    StackMapTable,
    BootstrapMethods,
    RuntimeVisibleTypeAnnotations,
    RuntimeInvisibleTypeAnnotations,
    MethodParameters,
}
