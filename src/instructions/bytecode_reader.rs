pub struct BytecodeReader {
    pc: usize,
    code: Vec<u8>,
}

impl BytecodeReader {
    pub fn new(code: Vec<u8>) -> Self {
        Self { pc: 0, code }
    }

    pub fn reset(&mut self, code: Vec<u8>, pc: usize) {
        self.code = code;
        self.pc = pc;
    }

    pub fn read_u8(&mut self) -> u8 {
        let byte = self.code[self.pc];
        self.pc += 1;
        byte
    }

    pub fn read_i8(&mut self) -> i8 {
        let byte = self.code[self.pc] as i8;
        self.pc += 1;
        byte
    }

    pub fn read_u16(&mut self) -> u16 {
        let byte1 = self.code[self.pc] as u16;
        let byte2 = self.code[self.pc + 1] as u16;
        self.pc += 2;
        (byte1 << 8) | byte2
    }

    pub fn read_i16(&mut self) -> i16 {
        let byte1 = self.code[self.pc] as i16;
        let byte2 = self.code[self.pc + 1] as i16;
        self.pc += 2;
        (byte1 << 8) | byte2
    }

    pub fn read_u32(&mut self) -> u32 {
        let byte1 = self.code[self.pc] as u32;
        let byte2 = self.code[self.pc + 1] as u32;
        let byte3 = self.code[self.pc + 2] as u32;
        let byte4 = self.code[self.pc + 3] as u32;
        self.pc += 4;
        (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4
    }

    pub fn read_i32(&mut self) -> i32 {
        let byte1 = self.code[self.pc] as i32;
        let byte2 = self.code[self.pc + 1] as i32;
        let byte3 = self.code[self.pc + 2] as i32;
        let byte4 = self.code[self.pc + 3] as i32;
        self.pc += 4;
        (byte1 << 24) | (byte2 << 16) | (byte3 << 8) | byte4
    }

    pub fn pc(&self) -> usize {
        self.pc
    }
}
