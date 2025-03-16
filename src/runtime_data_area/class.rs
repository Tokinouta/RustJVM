use core::panic;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

// use crate::classfile::ConstPool;

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
pub enum Constant {
    Utf8(String), // 标签值 1
    Integer(i32), // 标签值 3
    Float(f32),   // 标签值 4
    Long(i64),    // 标签值 5
    Double(f64),  // 标签值 6
    Class {
        name_index: u16,
    }, // 标签值 7
    String {
        string_index: u16,
    }, // 标签值 8
    FieldRef {
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 9
    MethodRef {
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 10
    InterfaceMethodRef {
        class_index: u16,
        name_and_type_index: u16,
    }, // 标签值 11
    NameAndType {
        name_index: u16,
        descriptor_index: u16,
    }, // 标签值 12
    MethodHandle {
        reference_kind: u8,
        reference_index: u16,
    }, // 标签值 15
    MethodType {
        descriptor_index: u16,
    }, // 标签值 16
    Dynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 标签值 17
    InvokeDynamic {
        bootstrap_method_attr_index: u16,
        name_and_type_index: u16,
    }, // 标签值 18
    Module {
        name_index: u16,
    }, // 标签值 19
    Package {
        name_index: u16,
    }, // 标签值 20
}

struct ConstantPool {
    class: Weak<RefCell<Class>>,
    consts: Vec<Option<Constant>>,
}

impl ConstantPool {
    pub fn new(
        class: Rc<RefCell<Class>>,
        classfile_constants: &crate::classfile::ConstPool,
    ) -> Self {
        let mut constant_pool = Self {
            class: Rc::downgrade(&class),
            consts: vec![],
        };
        for constant in classfile_constants {
            constant_pool.consts.push(Some(constant.get_constant()))
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
