use std::{fs::File, io::Read};

struct Loader {
    // path: String,
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
