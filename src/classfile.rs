use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{attribute::Attribute, loader::Loader, runtime_data_area::class::Constant};

pub struct Const {
    cp: Weak<RefCell<ConstPool>>,
    constant: Constant,
}

impl Const {
    pub fn new(cp: Weak<RefCell<ConstPool>>, constant: Constant) -> Self {
        Self { cp, constant }
    }

    pub fn get_constant(&self) -> Constant {
        self.constant.clone()
    }
}

pub struct ConstPool(Vec<Const>);

impl ConstPool {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn resolve(&self, index: u16) -> String {
        let index = (index - 1) as usize;
        match &self.0[index].constant {
            Constant::Utf8(s) => s.clone(),
            _ => String::from(""),
        }
    }

    pub fn push(&mut self, c: Const) {
        self.0.push(c);
    }
}

impl IntoIterator for ConstPool {
    type Item = Const;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ConstPool {
    type Item = &'a Const;
    type IntoIter = std::slice::Iter<'a, Const>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
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
