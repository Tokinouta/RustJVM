use std::{cell::RefCell, rc::Rc};

use crate::{
    attribute::{Attribute, ExceptionTable, LineNumberTableEntry, LocalVariableTableEntry},
    classfile::{ClassFile, Const, ConstPool, Field},
};

pub struct ClassFileLoader {
    data: Vec<u8>,
    pos: usize, // Add a position field to track the current reading position
}

impl ClassFileLoader {
    fn new(data: Vec<u8>) -> Self {
        ClassFileLoader { data, pos: 0 }
    }

    fn bytes(&mut self, n: usize) -> &[u8] {
        if self.pos + n > self.data.len() {
            panic!("Failed to read bytes: not enough data");
        }
        let bytes = &self.data[self.pos..self.pos + n];
        self.pos += n;
        bytes
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

    fn cpinfo(&mut self, const_pool: Rc<RefCell<ConstPool>>) {
        let const_pool_count = self.u2();
        // Valid constant pool indices start from 1
        for _ in 1..const_pool_count {
            let tag = self.u1();
            let c = match tag {
                0x01 => {
                    // UTF8 string literal, 2 bytes length + data
                    let size = self.u2() as usize;
                    Const::Utf8(String::from_utf8(self.bytes(size).to_vec()).unwrap())
                }
                0x03 => Const::Integer(self.u4() as i32),
                0x04 => Const::Float(f32::from_bits(self.u4())),
                0x05 => Const::Long(self.u8() as i64),
                0x06 => Const::Double(f64::from_bits(self.u8())),
                0x07 => {
                    Const::Class {
                        cp: Rc::downgrade(&const_pool),
                        name_index: self.u2(), // Class index
                    }
                }
                0x08 => {
                    Const::String {
                        cp: Rc::downgrade(&const_pool),
                        string_index: self.u2(), // String reference index
                    }
                }
                0x09 => Const::FieldRef {
                    cp: Rc::downgrade(&const_pool),
                    class_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x0a => Const::MethodRef {
                    cp: Rc::downgrade(&const_pool),
                    class_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x0b => Const::InterfaceMethodRef {
                    cp: Rc::downgrade(&const_pool),
                    class_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x0c => Const::NameAndType {
                    cp: Rc::downgrade(&const_pool),
                    name_index: self.u2(),
                    descriptor_index: self.u2(),
                },
                0x0f => Const::MethodHandle {
                    cp: Rc::downgrade(&const_pool),
                    reference_kind: self.u1(),
                    reference_index: self.u2(),
                },
                0x10 => Const::MethodType {
                    cp: Rc::downgrade(&const_pool),
                    descriptor_index: self.u2(),
                },
                0x11 => Const::Dynamic {
                    cp: Rc::downgrade(&const_pool),
                    bootstrap_method_attr_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x12 => Const::InvokeDynamic {
                    cp: Rc::downgrade(&const_pool),
                    bootstrap_method_attr_index: self.u2(),
                    name_and_type_index: self.u2(),
                },
                0x13 => Const::Module {
                    cp: Rc::downgrade(&const_pool),
                    name_index: self.u2(),
                },
                0x14 => Const::Package {
                    cp: Rc::downgrade(&const_pool),
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
            let c = const_pool.borrow().get_utf8(self.u2());
            interfaces.push(c);
        }
        interfaces
    }

    fn fields(&mut self, const_pool: Rc<RefCell<ConstPool>>) -> Vec<Field> {
        let mut fields = vec![];
        let fields_count = self.u2();
        for _ in 0..fields_count {
            let name = const_pool.borrow().get_utf8(self.u2());
            let descriptor = const_pool.borrow().get_utf8(self.u2());
            fields.push(Field::new(
                self.u2(),
                name,
                descriptor,
                self.attrs(const_pool.clone()),
            ))
        }
        return fields;
    }

    fn attrs(&mut self, const_pool: Rc<RefCell<ConstPool>>) -> Vec<Attribute> {
        let mut attrs = vec![];
        let attributes_count = self.u2();
        for _ in 0..attributes_count {
            let name = const_pool.borrow().get_utf8(self.u2());
            let _size = self.u4() as usize; // read the size of the attribute, though not used in the construction
            let attr = match name.as_str() {
                "Code" => {
                    let max_stack = self.u2();
                    let max_locals = self.u2();
                    let code_length = self.u4() as usize;
                    let code = self.bytes(code_length).to_vec();
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

    pub fn load(data: Vec<u8>) -> ClassFile {
        let mut loader = Self::new(data);
        let magic = loader.u4();
        assert_eq!(magic, 0xcafebabe, "Error: Invalid magic number");
        let major_version = loader.u2();
        let minor_version = loader.u2();

        let cp = Rc::new(RefCell::new(ConstPool::new()));
        loader.cpinfo(cp.clone()); // const pool info
        let access_flags = loader.u2(); // access flags
        let this_class = cp.borrow_mut().get_utf8(loader.u2()); // this class
        let super_class = cp.borrow_mut().get_utf8(loader.u2()); // super class
        let interfaces = loader.interfaces(cp.clone());
        let fields = loader.fields(cp.clone()); // fields
        let methods = loader.fields(cp.clone()); // methods
        let attributes = loader.attrs(cp.clone()); // methods
        let const_pool = cp;
        ClassFile::new(
            major_version,
            minor_version,
            const_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        )
    }
}

mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    fn read_data(path: &str) -> Vec<u8> {
        let mut file = File::open(path).expect("Failed to open file");
        let mut data = Vec::new();
        file.read_to_end(&mut data).expect("Failed to read file");
        data
    }

    #[test]
    fn test_loader_bytes() {
        let data = read_data("test_file.bin");
        let mut loader = ClassFileLoader::new(data);
        let bytes = loader.bytes(5);
        assert_eq!(bytes.len(), 5);
    }

    #[test]
    fn test_loader_u1() {
        let data = read_data("test_file.bin");
        let mut loader = ClassFileLoader::new(data);
        let byte = loader.u1();
        assert!(byte == 0x31);
    }

    #[test]
    fn test_loader_u2() {
        let data = read_data("test_file.bin");
        let mut loader = ClassFileLoader::new(data);
        let word = loader.u2();
        assert!(word == 0x3132);
    }

    #[test]
    fn test_loader_u4() {
        let data = read_data("test_file.bin");
        let mut loader = ClassFileLoader::new(data);
        let dword = loader.u4();
        assert!(dword == 0x31323334);
    }

    #[test]
    fn test_loader_u8() {
        let data = read_data("test_file.bin");
        let mut loader = ClassFileLoader::new(data);
        let qword = loader.u8();
        assert!(qword == 0x3132333435363738);
    }

    #[test]
    fn test_loader_sequential_read() {
        let data = read_data("test_file.bin");
        let mut loader = ClassFileLoader::new(data);
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
