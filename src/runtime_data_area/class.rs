use core::panic;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// use crate::classfile::ConstPool;

use crate::classfile::Const;

use super::Slot;

// 定义类、字段和方法的访问标志常量
const ACC_PUBLIC: u16 = 0x0001; // class field method
const ACC_PRIVATE: u16 = 0x0002; //       field method
const ACC_PROTECTED: u16 = 0x0004; //       field method
const ACC_STATIC: u16 = 0x0008; //       field method
const ACC_FINAL: u16 = 0x0010; // class field method
const ACC_SUPER: u16 = 0x0020; // class
const ACC_SYNCHRONIZED: u16 = 0x0020; //             method
const ACC_VOLATILE: u16 = 0x0040; //       field
const ACC_BRIDGE: u16 = 0x0040; //             method
const ACC_TRANSIENT: u16 = 0x0080; //       field
const ACC_VARARGS: u16 = 0x0080; //             method
const ACC_NATIVE: u16 = 0x0100; //             method
const ACC_INTERFACE: u16 = 0x0200; // class
const ACC_ABSTRACT: u16 = 0x0400; // class       method
const ACC_STRICT: u16 = 0x0800; //             method
const ACC_SYNTHETIC: u16 = 0x1000; // class field method
const ACC_ANNOTATION: u16 = 0x2000; // class
const ACC_ENUM: u16 = 0x4000; // class field

struct Class {
    access_flags: u16,
    name: String,
    super_class: Rc<RefCell<Class>>,
    super_class_name: String,
    constant_pool: Box<ConstantPool>,
    fields: Vec<Field>,
    methods: Vec<Method>,
    interfaces: Vec<Class>,
    interface_names: Vec<String>,
    loader: Rc<RefCell<ClassLoader>>,
    instance_slot_count: usize,
    static_slot_count: usize,
    static_vars: Vec<Slot>,
    // init_started: bool,
    // init_thread: Rc<RefCell<Thread>>,
    // clinit_method: Option<Method>,
    // init_method: Option<Method>,
    // instance_init_method: Option<Method>,
}

impl Class {
    pub fn new() -> Self {
        Self {
            access_flags: 0,
            name: String::new(),
            super_class: Rc::new(RefCell::new(Class::new())),
            super_class_name: String::new(),
            constant_pool: Box::new(ConstPool::new()),
            fields: vec![],
            methods: vec![],
            interfaces: vec![],
            interface_names: vec![],
            loader: Rc::new(RefCell::new(ClassLoader::new())),
            instance_slot_count: 0,
            static_slot_count: 0,
            static_vars: vec![],
        }
    }

    pub fn is_public(&self) -> bool {
        self.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.access_flags & ACC_FINAL != 0
    }

    pub fn is_super(&self) -> bool {
        self.access_flags & ACC_SUPER != 0
    }

    pub fn is_interface(&self) -> bool {
        self.access_flags & ACC_INTERFACE != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.access_flags & ACC_ABSTRACT != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_annotation(&self) -> bool {
        self.access_flags & ACC_ANNOTATION != 0
    }

    pub fn is_enum(&self) -> bool {
        self.access_flags & ACC_ENUM != 0
    }
}

#[derive(Clone)]
struct ClassMember {
    access_flags: u16,
    name: String,
    descriptor: String,
    class: Weak<RefCell<Class>>,
}

impl ClassMember {
    pub fn new() -> Self {
        Self {
            access_flags: 0,
            name: String::new(),
            descriptor: String::new(),
            class: Weak::new(),
        }
    }

    pub fn copy_member_info(&mut self, member_info: &crate::classfile::Field) {
        self.access_flags = member_info.flags;
        self.name = member_info.name.clone();
        self.descriptor = member_info.descriptor.clone();
    }
}

#[derive(Clone)]
struct Field {
    info: ClassMember,
}

impl Field {
    pub fn new() -> Self {
        Self {
            info: ClassMember::new(),
        }
    }

    pub fn new_fields(
        class: Rc<RefCell<Class>>,
        class_fields: &[crate::classfile::Field],
    ) -> Vec<Field> {
        let mut fields = vec![];
        for field in class_fields {
            let mut f = Field::new();
            f.info.copy_member_info(&field);
            f.info.class = Rc::downgrade(&class);
            fields.push(f);
        }
        fields
    }

    pub fn is_public(&self) -> bool {
        self.info.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_protected(&self) -> bool {
        self.info.access_flags & ACC_PROTECTED != 0
    }

    pub fn is_private(&self) -> bool {
        self.info.access_flags & ACC_PRIVATE != 0
    }

    pub fn is_static(&self) -> bool {
        self.info.access_flags & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.info.access_flags & ACC_FINAL != 0
    }

    pub fn is_volatile(&self) -> bool {
        self.info.access_flags & ACC_VOLATILE != 0
    }

    pub fn is_transient(&self) -> bool {
        self.info.access_flags & ACC_TRANSIENT != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.info.access_flags & ACC_SYNTHETIC != 0
    }

    pub fn is_enum(&self) -> bool {
        self.info.access_flags & ACC_ENUM != 0
    }
}

#[derive(Clone)]
struct Method {
    info: ClassMember,
    max_stack: u16,
    max_locals: u16,
    code: Vec<u8>,
}

impl Method {
    pub fn new() -> Self {
        Self {
            info: ClassMember::new(),
            max_stack: 0,
            max_locals: 0,
            code: vec![],
        }
    }

    pub fn new_methods(
        class: Rc<RefCell<Class>>,
        class_methods: &[crate::classfile::Field],
    ) -> Vec<Method> {
        let mut methods = vec![];
        for method in class_methods {
            let mut m = Method::new();
            m.info.copy_member_info(&method);
            m.info.class = Rc::downgrade(&class);
            method.attributes.iter().for_each(|a| {
                if let crate::attribute::Attribute::Code {
                    max_stack,
                    max_locals,
                    code,
                    ..
                } = a
                {
                    m.max_stack = *max_stack;
                    m.max_locals = *max_locals;
                    m.code = code.clone();
                }
            });
            methods.push(m);
        }
        methods
    }

    pub fn is_public(&self) -> bool {
        self.info.access_flags & ACC_PUBLIC != 0
    }

    pub fn is_private(&self) -> bool {
        self.info.access_flags & ACC_PRIVATE != 0
    }

    pub fn is_protected(&self) -> bool {
        self.info.access_flags & ACC_PROTECTED != 0
    }

    pub fn is_static(&self) -> bool {
        self.info.access_flags & ACC_STATIC != 0
    }

    pub fn is_final(&self) -> bool {
        self.info.access_flags & ACC_FINAL != 0
    }

    pub fn is_synchronized(&self) -> bool {
        self.info.access_flags & ACC_SYNCHRONIZED != 0
    }

    pub fn is_bridge(&self) -> bool {
        self.info.access_flags & ACC_BRIDGE != 0
    }

    pub fn is_varargs(&self) -> bool {
        self.info.access_flags & ACC_VARARGS != 0
    }

    pub fn is_native(&self) -> bool {
        self.info.access_flags & ACC_NATIVE != 0
    }

    pub fn is_abstract(&self) -> bool {
        self.info.access_flags & ACC_ABSTRACT != 0
    }

    pub fn is_strict(&self) -> bool {
        self.info.access_flags & ACC_STRICT != 0
    }

    pub fn is_synthetic(&self) -> bool {
        self.info.access_flags & ACC_SYNTHETIC != 0
    }
}

#[derive(Clone)]
struct SymRef {
    cp: Weak<RefCell<ConstantPool>>,
    class_name: String,
    class: Weak<RefCell<Class>>,
}

#[derive(Clone)]
pub enum Constant {
    Utf8(String),
    Integer(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Class(SymRef),
    String(String),
    FieldRef {
        sym_ref: SymRef,
        name: String,
        descriptor: String,
        field: Box<Field>,
    },
    MethodRef {
        sym_ref: SymRef,
        name: String,
        descriptor: String,
        method: Box<Method>,
    },
    InterfaceMethodRef {
        sym_ref: SymRef,
        name: String,
        descriptor: String,
        method: Box<Method>,
    },
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    },
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    },
    MethodType {
        descriptor_index: u16,
    },
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    },
    Module {
        name_index: u16,
    },
    Package {
        name_index: u16,
    },
}

#[derive(Clone)]
struct ConstantPool {
    class: Weak<RefCell<Class>>,
    consts: Vec<Option<Constant>>,
}

impl ConstantPool {
    // TODO: It needs an implementation that preallocates all the constant pool slots.
    pub fn new(
        class: Rc<RefCell<Class>>,
        classfile_constants: &crate::classfile::ConstPool,
    ) -> Rc<RefCell<Self>> {
        let mut constant_pool = Rc::new(RefCell::new(Self {
            class: Rc::downgrade(&class),
            consts: vec![],
        }));
        for cf_const in classfile_constants {
            match cf_const {
                Const::Utf8(s) => {
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::Utf8(s.clone())));
                }
                Const::Integer(i) => {
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::Integer(*i)));
                }
                Const::Float(f) => {
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::Float(*f)));
                }
                Const::Long(l) => {
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::Long(*l)));
                    constant_pool.borrow_mut().consts.push(None);
                }
                Const::Double(d) => {
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::Double(*d)));
                    constant_pool.borrow_mut().consts.push(None);
                }
                Const::Class { cp, name_index } => {
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::Class(SymRef {
                            cp: Rc::downgrade(&constant_pool),
                            class_name: cp.upgrade().unwrap().borrow().get_utf8(*name_index),
                            class: Rc::downgrade(&class),
                        })));
                }
                Const::String { cp, string_index } => {
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::String(
                            cp.upgrade().unwrap().borrow().get_utf8(*string_index),
                        )));
                }
                Const::FieldRef {
                    cp,
                    class_index,
                    name_and_type_index,
                } => {
                    let class_name = cp.upgrade().unwrap().borrow().get_utf8(*class_index);
                    let (name, descriptor) = cp
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .get_name_and_type(*name_and_type_index);
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::FieldRef {
                            sym_ref: SymRef {
                                cp: Rc::downgrade(&constant_pool),
                                class_name,
                                class: Rc::downgrade(&class),
                            },
                            name,
                            descriptor,
                            field: Box::new(Field::new()),
                        }));
                }
                Const::MethodRef {
                    cp,
                    class_index,
                    name_and_type_index,
                } => {
                    let class_name = cp.upgrade().unwrap().borrow().get_utf8(*class_index);
                    let (name, descriptor) = cp
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .get_name_and_type(*name_and_type_index);
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::MethodRef {
                            sym_ref: SymRef {
                                cp: Rc::downgrade(&constant_pool),
                                class_name,
                                class: Rc::downgrade(&class),
                            },
                            name,
                            descriptor,
                            method: Box::new(Method::new()),
                        }));
                },
                Const::InterfaceMethodRef {
                    cp,
                    class_index,
                    name_and_type_index,
                } => {
                    let class_name = cp.upgrade().unwrap().borrow().get_utf8(*class_index);
                    let (name, descriptor) = cp
                        .upgrade()
                        .unwrap()
                        .borrow()
                        .get_name_and_type(*name_and_type_index);
                    constant_pool
                        .borrow_mut()
                        .consts
                        .push(Some(Constant::InterfaceMethodRef {
                            sym_ref: SymRef {
                                cp: Rc::downgrade(&constant_pool),
                                class_name,
                                class: Rc::downgrade(&class),
                            },
                            name,
                            descriptor,
                            method: Box::new(Method::new()),
                        }));
                },
                Const::NameAndType {
                    cp,
                    name_index,
                    descriptor_index,
                } => todo!(),
                Const::MethodHandle {
                    cp,
                    reference_kind,
                    reference_index,
                } => todo!(),
                Const::MethodType {
                    cp,
                    descriptor_index,
                } => todo!(),
                Const::Dynamic {
                    cp,
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => todo!(),
                Const::InvokeDynamic {
                    cp,
                    bootstrap_method_attr_index,
                    name_and_type_index,
                } => todo!(),
                Const::Module { cp, name_index } => todo!(),
                Const::Package { cp, name_index } => todo!(),
            }
        }
        constant_pool
    }

    pub fn get(&self, index: usize) -> &Constant {
        if let Some(constant) = self.consts[index].as_ref() {
            constant
        } else {
            panic!("invalid constant pool index")
        }
    }
}
