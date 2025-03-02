use std::{cell::RefCell, rc::Rc};

use crate::loader::{ConstPool, Loader};

#[repr(u8)]
pub enum Const {
    Utf8(String), // 标签值 1
    Integer(i32), // 标签值 3
    Float(f32),   // 标签值 4
    Long(i64),    // 标签值 5
    Double(f64),  // 标签值 6
    Class {
        cp: Rc<RefCell<ConstPool>>,
        name_index: u16,
    }, // 标签值 7
    String {
        cp: Rc<RefCell<ConstPool>>,
        string_index: u16,
    }, // 标签值 8
    FieldRef {
        cp: Rc<RefCell<ConstPool>>,
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 9
    MethodRef {
        cp: Rc<RefCell<ConstPool>>,
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 10
    InterfaceMethodRef {
        cp: Rc<RefCell<ConstPool>>,
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 11
    NameAndType {
        cp: Rc<RefCell<ConstPool>>,
        name_index: u16,
        descriptor_index: u16,
    }, // 标签值 12
    MethodHandle {
        cp: Rc<RefCell<ConstPool>>,
        reference_kind: u8,
        reference_index: u16,
    }, // 标签值 15
    MethodType {
        cp: Rc<RefCell<ConstPool>>,
        descriptor_index: u16,
    }, // 标签值 16
    Dynamic {
        cp: Rc<RefCell<ConstPool>>,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 标签值 17
    InvokeDynamic {
        cp: Rc<RefCell<ConstPool>>,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 标签值 18
    Module {
        cp: Rc<RefCell<ConstPool>>,
        name_index: u16,
    }, // 标签值 19
    Package {
        cp: Rc<RefCell<ConstPool>>,
        name_index: u16,
    }, // 标签值 20
}

trait ConstantInfo {
    fn read_info(&mut self, reader: &mut Loader);
}
