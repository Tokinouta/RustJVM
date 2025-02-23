use std::{fs::File, io::Read};

struct Loader {
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

    fn cpinfo<'a>(&mut self, const_pool: &'a mut ConstPool) {
        let const_pool_count = self.u2();
        // Valid constant pool indices start from 1
        for _ in 1..const_pool_count {
            let mut c = Const {
                tag: self.u1(),
                ..Default::default()
            };
            match c.tag {
                0x01 => {
                    // UTF8 string literal, 2 bytes length + data
                    let size = self.u2() as usize;
                    c.string = String::from_utf8(self.bytes(size)).unwrap();
                }
                0x07 => {
                    // Class index
                    c.name_index = self.u2();
                }
                0x08 => {
                    // String reference index
                    c.string_index = self.u2();
                }
                0x09 | 0x0a => {
                    // Field and method: class index + NaT index
                    c.class_index = self.u2();
                    c.name_and_type_index = self.u2();
                }
                0x0c => {
                    // Name-and-type
                    c.name_index = self.u2();
                    c.desc_index = self.u2();
                }
                _ => {
                    println!("unsupported tag: {}", c.tag);
                }
            }
            const_pool.0.push(c)
        }
    }

    fn interfaces(&mut self, const_pool: &ConstPool) -> Vec<String> {
        let mut interfaces = vec![];
        let interface_count = self.u2();
        for _ in 0..interface_count {
            let c = const_pool.resolve(self.u2());
            interfaces.push(c);
        }
        interfaces
    }

    fn fields(&mut self, const_pool: &ConstPool) -> Vec<Field> {
        let mut fields = vec![];
        let fields_count = self.u2();
        for _ in 0..fields_count {
            let name = const_pool.resolve(self.u2());
            let descriptor = const_pool.resolve(self.u2());
            fields.push(Field {
                flags: self.u2(),
                name,
                descriptor,
                attributes: self.attrs(const_pool),
            })
        }
        return fields;
    }

    fn attrs(&mut self, const_pool: &ConstPool) -> Vec<Attribute> {
        let mut attrs = vec![];
        let attributes_count = self.u2();
        for _ in 0..attributes_count {
            let name = const_pool.resolve(self.u2());
            let size = self.u4() as usize;
            attrs.push(Attribute {
                name,
                data: self.bytes(size),
            })
        }
        return attrs;
    }
}

#[derive(Default)]
struct Const {
    tag: u8,
    name_index: u16,
    class_index: u16,
    name_and_type_index: u16,
    string_index: u16,
    desc_index: u16,
    string: String,
}

#[derive(Default)]
struct ConstPool(Vec<Const>);

impl ConstPool {
    fn resolve(&self, index: u16) -> String {
        let index = (index - 1) as usize;
        if self.0[index].tag == 0x01 {
            self.0[index].string.clone()
        } else {
            String::from("")
        }
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
struct Attribute {
    name: String,
    data: Vec<u8>,
}

#[derive(Default)]
pub struct Class {
    const_pool: ConstPool,
    name: String,
    Super: String,
    flags: u16,
    interfaces: Vec<String>,
    fields: Vec<Field>,
    methods: Vec<Field>,
    attributes: Vec<Attribute>,
}

pub fn load(path: String) -> Class {
    let mut loader = Loader::new(path);
    let mut c = Class::default();
    loader.u8(); // magic (u32), minor (u16), major (u16)
    let mut cp = ConstPool::default();
    loader.cpinfo(&mut cp); // const pool info
    c.flags = loader.u2(); // access flags
    c.name = cp.resolve(loader.u2()); // this class
    c.Super = cp.resolve(loader.u2()); // super class
    c.interfaces = loader.interfaces(&cp);
    c.fields = loader.fields(&cp); // fields
    c.methods = loader.fields(&cp); // methods
    c.attributes = loader.attrs(&cp); // methods
    c.const_pool = cp;
    return c;
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
