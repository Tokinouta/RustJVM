use std::{cell::RefCell, rc::Rc};

struct Object {
    // class: Class,
    // fields: Vec<Value>,
}

#[derive(Clone)]
struct Slot{
    num: i32,
    objref: Option<Rc<RefCell<Object>>>,
}

struct LocalVars {
    slots: Vec<Slot>,
}

impl LocalVars {
    fn new(max_locals: usize) -> Self {
        Self {
            slots: vec![Slot { num: 0, objref: None }; max_locals],
        }
    }

    fn set_int(&mut self, index: usize, val: i32) {
        self.slots[index].num = val;
    }

    fn get_int(&self, index: usize) -> i32 {
        self.slots[index].num
    }

    fn set_ref(&mut self, index: usize, obj: Rc<RefCell<Object>>) {
        self.slots[index].objref = Some(obj);
    }

    fn get_ref(&self, index: usize) -> Option<Rc<RefCell<Object>>> {
        match &self.slots[index].objref {
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }

    fn get_float(&self, index: usize) -> f32 {
        f32::from_bits(self.slots[index].num as u32)
    }

    fn set_float(&mut self, index: usize, val: f32) {
        self.slots[index].num = val.to_bits() as i32;
    }

    fn get_long(&self, index: usize) -> i64 {
        let low = self.slots[index].num as u32;
        let high = self.slots[index + 1].num as u32;
        ((high as i64) << 32) | (low as i64)
    }

    fn set_long(&mut self, index: usize, val: i64) {
        let low = val as u32;
        let high = (val >> 32) as u32;
        self.slots[index].num = low as i32;
        self.slots[index + 1].num = high as i32;
    }

    fn get_double(&self, index: usize) -> f64 {
        f64::from_bits(self.get_long(index) as u64)
    }

    fn set_double(&mut self, index: usize, val: f64) {
        self.set_long(index, val.to_bits() as i64);
    }
}

struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    fn new(max_stack: usize) -> Self {
        Self {
            size: 0,
            slots: vec![Slot { num: 0, objref: None }; max_stack],
        }
    }

    fn push_int(&mut self, val: i32) {
        self.slots[self.size].num = val;
        self.size += 1;
    }

    fn pop_int(&mut self) -> i32 {
        self.size -= 1;
        self.slots[self.size].num
    }

    fn push_ref(&mut self, obj: Rc<RefCell<Object>>) {
        self.slots[self.size].objref = Some(obj);
        self.size += 1;
    }

    fn pop_ref(&mut self) -> Option<Rc<RefCell<Object>>> {
        self.size -= 1;
        match &self.slots[self.size].objref {
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }

    fn push_float(&mut self, val: f32) {
        self.slots[self.size].num = val.to_bits() as i32;
        self.size += 1;
    }

    fn pop_float(&mut self) -> f32 {
        self.size -= 1;
        f32::from_bits(self.slots[self.size].num as u32)
    }

    fn push_long(&mut self, val: i64) {
        let low = val as u32;
        let high = (val >> 32) as u32;
        self.slots[self.size].num = low as i32;
        self.slots[self.size + 1].num = high as i32;
        self.size += 2;
    }

    fn pop_long(&mut self) -> i64 {
        self.size -= 2;
        let low = self.slots[self.size].num as u32;
        let high = self.slots[self.size + 1].num as u32;
        ((high as i64) << 32) | (low as i64)
    }

    fn push_double(&mut self, val: f64) {
        self.push_long(val.to_bits() as i64);
    }

    fn pop_double(&mut self) -> f64 {
        f64::from_bits(self.pop_long() as u64)
    }
}

struct Frame {
    lower: Option<Box<Frame>>,
    local_vars: Vec<i32>,
    operand_stack: Stack,
}

impl Frame {
    fn new(max_locals: usize, max_stack: usize) -> Self {
        Self {
            lower: None,
            local_vars: vec![0; max_locals],
            operand_stack: Stack::new(max_stack),
        }
    }
}

struct Stack {
    max_size: usize,
    size: usize,
    top: Option<Box<Frame>>,
}

impl Stack {
    fn new(max_size: usize) -> Self {
        Self {
            max_size,
            size: 0,
            top: None,
        }
    }

    fn push(&mut self, mut frame: Frame) {
        if self.size >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }

        frame.lower = self.top.take();
        self.top = Some(Box::new(frame));
        self.size += 1;
    }

    fn pop(&mut self) -> Box<Frame> {
        let frame = match &self.top {
            Some(_) => self.top.take(),
            None => panic!("jvm stack is empty"),
        };
        self.size -= 1;
        match frame {
            Some(mut f) => {
                self.top = f.lower;
                f.lower = None;
                f
            },
            None => panic!("jvm stack is empty"),
        }
    }

    fn top(&self) -> &Frame {
        match &self.top {
            Some(f) => f,
            None => panic!("jvm stack is empty"),
        }
    }
}

struct Thread {
    pc: usize,
    stack: Box<Stack>,
}

impl Thread {
    fn new() -> Self {
        Self {
            pc: 0,
            stack: Box::new(Stack::new(1024)),
        }
    }

    fn pc(&self) -> usize {
        self.pc
    }

    fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    fn push_frame(&mut self, frame: Frame) {
        self.stack.push(frame);
    }

    fn pop_frame(&mut self) -> Box<Frame> {
        self.stack.pop()
    }

    fn current_frame(&self) -> &Frame {
        self.stack.top()
    }
}
