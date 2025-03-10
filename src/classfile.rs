use std::{cell::RefCell, rc::Rc};

use crate::{attribute::Attribute, loader::Loader};

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

#[derive(Default)]
pub struct ConstPool(Vec<Const>);

impl ConstPool {
    pub fn resolve(&self, index: u16) -> String {
        let index = (index - 1) as usize;
        match &self.0[index] {
            Const::Utf8(s) => s.clone(),
            _ => String::from(""),
        }
    }

    pub fn push(&mut self, c: Const) {
        self.0.push(c);
    }
}

// Field type is used for both, fields and methods
pub struct Field {
    flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<Attribute>,
}

impl Field {
    pub fn new(flags: u16, name: String, descriptor: String, attributes: Vec<Attribute>) -> Self {
        Self {
            flags,
            name,
            descriptor,
            attributes,
        }
    }

    pub fn get_code(&self) -> Option<&Attribute> {
        self.attributes.iter().find(|a| match a {
            Attribute::Code { .. } => true,
            _ => false,
        })
    }
}

// Attributes contain addition information about fields and classes
// The most useful is "Code" attribute, which contains actual byte code

#[derive(Default)]
pub struct Class {
    major_version: u16,
    minor_version: u16,
    const_pool: Rc<RefCell<ConstPool>>,
    flags: u16,
    this_class: String,
    super_class: String,
    interfaces: Vec<String>,
    fields: Vec<Field>,
    methods: Vec<Field>,
    attributes: Vec<Attribute>,
}

impl Class {
    pub fn new(
        major_version: u16,
        minor_version: u16,
        const_pool: Rc<RefCell<ConstPool>>,
        flags: u16,
        this_class: String,
        super_class: String,
        interfaces: Vec<String>,
        fields: Vec<Field>,
        methods: Vec<Field>,
        attributes: Vec<Attribute>,
    ) -> Self {
        Self {
            major_version,
            minor_version,
            const_pool,
            flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }
}
