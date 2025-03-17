use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{attribute::Attribute, loader::Loader};

// pub struct Const {
//     cp: Weak<RefCell<ConstPool>>,
//     constant: Constant,
// }

#[derive(Clone)]
pub enum Const {
    Utf8(String), // 标签值 1
    Integer(i32), // 标签值 3
    Float(f32),   // 标签值 4
    Long(i64),    // 标签值 5
    Double(f64),  // 标签值 6
    Class {
        cp: Weak<RefCell<ConstPool>>,
        name_index: u16,
    }, // 标签值 7
    String {
        cp: Weak<RefCell<ConstPool>>,
        string_index: u16,
    }, // 标签值 8
    FieldRef {
        cp: Weak<RefCell<ConstPool>>,
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 9
    MethodRef {
        cp: Weak<RefCell<ConstPool>>,
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 10
    InterfaceMethodRef {
        cp: Weak<RefCell<ConstPool>>,
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 11
    NameAndType {
        cp: Weak<RefCell<ConstPool>>,
        name_index: u16,
        descriptor_index: u16,
    }, // 标签值 12
    MethodHandle {
        cp: Weak<RefCell<ConstPool>>,
        reference_kind: u8,
        reference_index: u16,
    }, // 标签值 15
    MethodType {
        cp: Weak<RefCell<ConstPool>>,
        descriptor_index: u16,
    }, // 标签值 16
    Dynamic {
        cp: Weak<RefCell<ConstPool>>,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 标签值 17
    InvokeDynamic {
        cp: Weak<RefCell<ConstPool>>,
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 标签值 18
    Module {
        cp: Weak<RefCell<ConstPool>>,
        name_index: u16,
    }, // 标签值 19
    Package {
        cp: Weak<RefCell<ConstPool>>,
        name_index: u16,
    }, // 标签值 20
}

// impl Const {
//     pub fn new(cp: Weak<RefCell<ConstPool>>, constant: Constant) -> Self {
//         Self { cp, constant }
//     }

//     pub fn get_constant(&self) -> Constant {
//         self.constant.clone()
//     }
// }

pub struct ConstPool(Vec<Const>);

impl ConstPool {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }

    pub fn push(&mut self, c: Const) {
        self.0.push(c);
    }

    pub fn get_utf8(&self, index: u16) -> String {
        let index = (index - 1) as usize;
        match &self.0[index] {
            Const::Utf8(s) => s.clone(),
            _ => String::from(""),
        }
    }

    pub fn get_class(&self, index: u16) -> String {
        let index = (index - 1) as usize;
        match &self.0[index] {
            Const::Class { cp, name_index } => {
                let cp = cp.upgrade().unwrap();
                let cp = cp.borrow();
                cp.get_utf8(*name_index)
            }
            _ => String::from(""),
        }
    }

    pub fn get_name_and_type(&self, index: u16) -> (String, String) {
        let index = (index - 1) as usize;
        match &self.0[index] {
            Const::NameAndType {
                cp,
                name_index,
                descriptor_index,
            } => {
                let cp = cp.upgrade().unwrap();
                let cp = cp.borrow();
                (
                    cp.get_utf8(*name_index),
                    cp.get_utf8(*descriptor_index),
                )
            }
            _ => (String::from(""), String::from("")),
        }
    }
}

impl Iterator for ConstPool {
    type Item = Const;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(self.0.remove(0))
        }
    }
}

impl<'a> Iterator for &'a ConstPool {
    type Item = &'a Const;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0.is_empty() {
            None
        } else {
            Some(&self.0[0])
        }
    }
}

// Field type is used for both, fields and methods
pub struct Field {
    pub flags: u16,
    pub name: String,
    pub descriptor: String,
    pub attributes: Vec<Attribute>,
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

pub struct ClassFile {
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

impl ClassFile {
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
