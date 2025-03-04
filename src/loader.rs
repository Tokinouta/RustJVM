use std::{cell::RefCell, fs::File, io::Read, rc::Rc};

use crate::{
    attribute::{Attribute, ExceptionTable, LineNumberTableEntry, LocalVariableTableEntry},
    classfile::{Const, ConstPool},
};

pub struct Loader {
    file: File,
}

impl Loader {
    fn new(path: String) -> Self {
        Loader {
            file: File::open(path).unwrap(),
        }
    }

    fn bytes(&mut self, n: usize) -> Vec<u8> {
        let mut bytes = vec![0u8; n];
        match self.file.read_exact(&mut bytes) {
            Ok(_) => bytes,
            Err(_) => panic!("Failed to read bytes"),
        }
    }

    fn u1(&mut self) -> u8 {
        self.bytes(1)[0]
    }

    fn u2(&mut self) -> u16 {
        u16::from_be_bytes(self.bytes(2).try_into().unwrap())
    }

    fn u4(&mut self) -> u32 {
        u32::from_be_bytes(self.bytes(4).try_into().unwrap())
    }

    fn u8(&mut self) -> u64 {
        u64::from_be_bytes(self.bytes(8).try_into().unwrap())
    }

    fn cpinfo<'a>(&mut self, const_pool: Rc<RefCell<ConstPool>>) {
        let const_pool_count = self.u2();
        // Valid constant pool indices start from 1
        for _ in 1..const_pool_count {
            let tag = self.u1();
            let c = match tag {
                0x01 => {
                    // UTF8 string literal, 2 bytes length + data
                    let size = self.u2() as usize;
                    Const::Utf8(String::from_utf8(self.bytes(size)).unwrap())
                }
                0x03 => Const::Integer(self.u4() as i32),
                0x04 => Const::Float(f32::from_bits(self.u4())),
                0x05 => Const::Long(self.u8() as i64),
                0x06 => Const::Double(f64::from_bits(self.u8())),
                0x07 => {
                    Const::Class {
                        cp: const_pool.clone(),
                        name_index: self.u2(), // Class index
                    }
                }
                0x08 => {
                    Const::String {
                        cp: const_pool.clone(),
                        string_index: self.u2(), // String reference index
                    }
                }
                0x09 => Const::FieldRef {
                    cp: const_pool.clone(),
                    class_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x0a => Const::MethodRef {
                    cp: const_pool.clone(),
                    class_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x0b => Const::InterfaceMethodRef {
                    cp: const_pool.clone(),
                    class_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x0c => Const::NameAndType {
                    cp: const_pool.clone(),
                    name_index: self.u2(),
                    descriptor_index: self.u2(),
                },
                0x0f => Const::MethodHandle {
                    cp: const_pool.clone(),
                    reference_kind: self.u1(),
                    reference_index: self.u2(),
                },
                0x10 => Const::MethodType {
                    cp: const_pool.clone(),
                    descriptor_index: self.u2(),
                },
                0x11 => Const::Dynamic {
                    cp: const_pool.clone(),
                    bootstrap_method_attr_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x12 => Const::InvokeDynamic {
                    cp: const_pool.clone(),
                    bootstrap_method_attr_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x13 => Const::Module {
                    cp: const_pool.clone(),
                    name_index: self.u2(),
                },
                0x14 => Const::Package {
                    cp: const_pool.clone(),
                    name_index: self.u2(),
                },
                _ => {
                    println!("unsupported tag: {}", tag);
                    continue;
                }
            };
            const_pool.borrow_mut().push(c)
        }
    }

    fn interfaces(&mut self, const_pool: Rc<RefCell<ConstPool>>) -> Vec<String> {
        let mut interfaces = vec![];
        let interface_count = self.u2();
        for _ in 0..interface_count {
            let c = const_pool.borrow().resolve(self.u2());
            interfaces.push(c);
        }
        interfaces
    }

    fn fields(&mut self, const_pool: Rc<RefCell<ConstPool>>) -> Vec<Field> {
        let mut fields = vec![];
        let fields_count = self.u2();
        for _ in 0..fields_count {
            let name = const_pool.borrow().resolve(self.u2());
            let descriptor = const_pool.borrow().resolve(self.u2());
            fields.push(Field {
                flags: self.u2(),
                name,
                descriptor,
                attributes: self.attrs(const_pool.clone()),
            })
        }
        return fields;
    }

    fn attrs(&mut self, const_pool: Rc<RefCell<ConstPool>>) -> Vec<Attribute> {
        let mut attrs = vec![];
        let attributes_count = self.u2();
        for _ in 0..attributes_count {
            let name = const_pool.borrow().resolve(self.u2());
            let size = self.u4() as usize;
            // attrs.push(Attribute {
            //     name,
            //     data: self.bytes(size),
            // })
            let attr = match name.as_str() {
                "Code" => {
                    let max_stack = self.u2();
                    let max_locals = self.u2();
                    let code_length = self.u4() as usize;
                    let code = self.bytes(code_length);
                    let exception_table_length = self.u2();
                    let mut exception_table = Vec::new();
                    for _ in 0..exception_table_length {
                        let start_pc = self.u2();
                        let end_pc = self.u2();
                        let handler_pc = self.u2();
                        let catch_type = self.u2();
                        exception_table.push(ExceptionTable::new(
                            start_pc, end_pc, handler_pc, catch_type,
                        ));
                    }
                    let attributes = self.attrs(const_pool.clone());
                    Attribute::Code {
                        cp: const_pool.clone(),
                        max_stack,
                        max_locals,
                        code,
                        exception_table,
                        attributes,
                    }
                }
                "ConstantValue" => Attribute::ConstantValue(self.u2()),
                "Deprecated" => Attribute::Deprecated,
                "Exceptions" => {
                    let number_of_exceptions = self.u2();
                    let mut exception_index_table = vec![];
                    for _ in 0..number_of_exceptions {
                        exception_index_table.push(self.u2());
                    }
                    Attribute::Exceptions {
                        exception_index_table,
                    }
                }
                "LineNumberTable" => {
                    let line_number_table_length = self.u2();
                    let mut line_number_table = vec![];
                    for _ in 0..line_number_table_length {
                        let start_pc = self.u2();
                        let line_number = self.u2();
                        line_number_table.push(LineNumberTableEntry::new(start_pc, line_number));
                    }
                    Attribute::LineNumberTable { line_number_table }
                }
                "LocalVariableTable" => {
                    let local_variable_table_length = self.u2();
                    let mut local_variable_table = vec![];
                    for _ in 0..local_variable_table_length {
                        let start_pc = self.u2();
                        let line_number = self.u2();
                        local_variable_table
                            .push(LocalVariableTableEntry::new(start_pc, line_number));
                    }
                    Attribute::LocalVariableTable {
                        local_variable_table,
                    }
                }
                "SourceFile" => {
                    let index = self.u2();
                    Attribute::SourceFile {
                        cp: const_pool.clone(),
                        index,
                    }
                }
                "Synthetic" => Attribute::Synthetic,
                _ => continue,
            };
            attrs.push(attr);
        }
        return attrs;
    }
}

// Field type is used for both, fields and methods
struct Field {
    flags: u16,
    name: String,
    descriptor: String,
    attributes: Vec<Attribute>,
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
    pub fn load(path: String) -> Class {
        let mut loader = Loader::new(path);
        let mut c = Class::default();
        let magic = loader.u4();
        assert_eq!(magic, 0xcafebabe, "Error: Invalid magic number");
        c.major_version = loader.u2();
        c.minor_version = loader.u2();

        let cp = Rc::new(RefCell::new(ConstPool::default()));
        loader.cpinfo(cp.clone()); // const pool info
        c.flags = loader.u2(); // access flags
        c.this_class = cp.borrow_mut().resolve(loader.u2()); // this class
        c.super_class = cp.borrow_mut().resolve(loader.u2()); // super class
        c.interfaces = loader.interfaces(cp.clone());
        c.fields = loader.fields(cp.clone()); // fields
        c.methods = loader.fields(cp.clone()); // methods
        c.attributes = loader.attrs(cp.clone()); // methods
        c.const_pool = cp;
        return c;
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_loader_bytes() {
        let path = "test_file.bin"; // 确保这个文件存在于文件系统上并且至少有5个字节长
        let mut loader = Loader::new(path.to_string());
        let bytes = loader.bytes(5);
        assert_eq!(bytes.len(), 5);
    }

    #[test]
    fn test_loader_u1() {
        let path = "test_file.bin"; // 确保这个文件存在于文件系统上并且至少有一个字节长
        let mut loader = Loader::new(path.to_string());
        let byte = loader.u1();
        assert!(byte == 0x31);
    }

    #[test]
    fn test_loader_u2() {
        let path = "test_file.bin"; // 确保这个文件存在于文件系统上并且至少有两个字节长
        let mut loader = Loader::new(path.to_string());
        let word = loader.u2();
        assert!(word == 0x3132);
    }

    #[test]
    fn test_loader_u4() {
        let path = "test_file.bin"; // 确保这个文件存在于文件系统上并且至少有四个字节长
        let mut loader = Loader::new(path.to_string());
        let dword = loader.u4();
        assert!(dword == 0x31323334);
    }

    #[test]
    fn test_loader_u8() {
        let path = "test_file.bin"; // 确保这个文件存在于文件系统上并且至少有八个字节长
        let mut loader = Loader::new(path.to_string());
        let qword = loader.u8();
        assert!(qword == 0x3132333435363738);
    }

    #[test]
    fn test_loader_sequential_read() {
        let path = "test_file.bin"; // 确保这个文件存在于文件系统上并且至少有八个字节长
        let mut loader = Loader::new(path.to_string());
        let dword = loader.u4();
        // print dword as hexadecimal
        println!("dword: {:x}", dword);
        assert!(dword == 0x31323334);
        let dword = loader.u4();
        println!("dword: {:x}", dword);
        assert!(dword == 0x35363738);
        let dword = loader.u1();
        println!("dword: {:x}", dword);
        assert!(dword == 0x39);
    }
}
