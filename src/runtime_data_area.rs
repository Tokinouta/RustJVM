use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

#[derive(PartialEq)]
pub struct Object {
    // class: Class,
    // fields: Vec<Value>,
}

#[derive(Clone)]
pub struct Slot {
    num: i32,
    objref: Option<Rc<RefCell<Object>>>,
}

pub struct LocalVars {
    max_locals: usize,
    slots: Vec<Slot>,
}

impl LocalVars {
    pub fn new(max_locals: usize) -> Self {
        Self {
            max_locals,
            slots: vec![
                Slot {
                    num: 0,
                    objref: None
                };
                max_locals
            ],
        }
    }

    pub fn set_int(&mut self, index: usize, val: i32) {
        self.slots[index].num = val;
    }

    pub fn get_int(&self, index: usize) -> i32 {
        self.slots[index].num
    }

    pub fn set_ref(&mut self, index: usize, obj: Option<Rc<RefCell<Object>>>) {
        self.slots[index].objref = obj;
    }

    pub fn get_ref(&self, index: usize) -> Option<Rc<RefCell<Object>>> {
        match &self.slots[index].objref {
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }

    pub fn get_float(&self, index: usize) -> f32 {
        f32::from_bits(self.slots[index].num as u32)
    }

    pub fn set_float(&mut self, index: usize, val: f32) {
        self.slots[index].num = val.to_bits() as i32;
    }

    pub fn get_long(&self, index: usize) -> i64 {
        let low = self.slots[index].num as u32;
        let high = self.slots[index + 1].num as u32;
        ((high as i64) << 32) | (low as i64)
    }

    pub fn set_long(&mut self, index: usize, val: i64) {
        let low = val as u32;
        let high = (val >> 32) as u32;
        self.slots[index].num = low as i32;
        self.slots[index + 1].num = high as i32;
    }

    pub fn get_double(&self, index: usize) -> f64 {
        f64::from_bits(self.get_long(index) as u64)
    }

    pub fn set_double(&mut self, index: usize, val: f64) {
        self.set_long(index, val.to_bits() as i64);
    }
}

pub struct OperandStack {
    size: usize,
    slots: Vec<Slot>,
}

impl OperandStack {
    pub fn new(max_stack: usize) -> Self {
        Self {
            size: 0,
            slots: vec![
                Slot {
                    num: 0,
                    objref: None
                };
                max_stack
            ],
        }
    }

    pub fn push_int(&mut self, val: i32) {
        self.slots[self.size].num = val;
        self.size += 1;
    }

    pub fn pop_int(&mut self) -> i32 {
        self.size -= 1;
        self.slots[self.size].num
    }

    pub fn push_ref(&mut self, obj: Option<Rc<RefCell<Object>>>) {
        self.slots[self.size].objref = obj;
        self.size += 1;
    }

    pub fn pop_ref(&mut self) -> Option<Rc<RefCell<Object>>> {
        self.size -= 1;
        match &self.slots[self.size].objref {
            Some(obj) => Some(obj.clone()),
            None => None,
        }
    }

    pub fn push_float(&mut self, val: f32) {
        self.slots[self.size].num = val.to_bits() as i32;
        self.size += 1;
    }

    pub fn pop_float(&mut self) -> f32 {
        self.size -= 1;
        f32::from_bits(self.slots[self.size].num as u32)
    }

    pub fn push_long(&mut self, val: i64) {
        let low = val as u32;
        let high = (val >> 32) as u32;
        self.slots[self.size].num = low as i32;
        self.slots[self.size + 1].num = high as i32;
        self.size += 2;
    }

    pub fn pop_long(&mut self) -> i64 {
        self.size -= 2;
        let low = self.slots[self.size].num as u32;
        let high = self.slots[self.size + 1].num as u32;
        ((high as i64) << 32) | (low as i64)
    }

    pub fn push_double(&mut self, val: f64) {
        self.push_long(val.to_bits() as i64);
    }

    pub fn pop_double(&mut self) -> f64 {
        f64::from_bits(self.pop_long() as u64)
    }

    pub fn push_slot(&mut self, slot: Slot) {
        self.slots[self.size] = slot;
        self.size += 1;
    }

    pub fn pop_slot(&mut self) -> Slot {
        self.size -= 1;
        self.slots[self.size].clone()
    }
}

pub struct Frame {
    pub lower: Option<Box<Frame>>,
    pub local_vars: LocalVars,
    pub operand_stack: OperandStack,
    pub thread: Weak<RefCell<Thread>>,
    next_pc: usize,
}

impl Frame {
    pub fn new(max_locals: usize, max_stack: usize, thread: Weak<RefCell<Thread>>) -> Self {
        Self {
            lower: None,
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack),
            thread,
            next_pc: 0,
        }
    }

    pub fn thread(&mut self) -> Option<Rc<RefCell<Thread>>> {
        self.thread.upgrade()
    }

    pub fn next_pc(&self) -> usize {
        self.next_pc
    }
}

pub struct Stack {
    max_size: usize,
    size: usize,
    top: Option<Box<Frame>>,
}

impl Stack {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            size: 0,
            top: None,
        }
    }

    pub fn push(&mut self, mut frame: Frame) {
        if self.size >= self.max_size {
            panic!("java.lang.StackOverflowError");
        }

        frame.lower = self.top.take();
        self.top = Some(Box::new(frame));
        self.size += 1;
    }

    pub fn pop(&mut self) -> Box<Frame> {
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
            }
            None => panic!("jvm stack is empty"),
        }
    }

    pub fn top(&self) -> &Frame {
        match &self.top {
            Some(f) => f,
            None => panic!("jvm stack is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct Thread {
    pc: usize,
    stack: Rc<RefCell<Stack>>,
}

impl Thread {
    pub fn new() -> Self {
        Self {
            pc: 0,
            stack: Rc::new(RefCell::new(Stack::new(1024))),
        }
    }

    pub fn pc(&self) -> usize {
        self.pc
    }

    pub fn set_pc(&mut self, pc: usize) {
        self.pc = pc;
    }

    pub fn push_frame(&mut self, frame: Frame) {
        self.stack.borrow_mut().push(frame);
    }

    pub fn pop_frame(&mut self) -> Box<Frame> {
        self.stack.borrow_mut().pop()
    }

    pub fn current_frame(&self) -> Ref<Frame> {
        Ref::map(self.stack.borrow(), |stack| stack.top())
    }
}

mod test {
    use std::os::unix::thread;

    use super::*;

    #[test]
    fn test_local_vars() {
        let mut local_vars = LocalVars::new(10);
        local_vars.set_int(0, 100);
        assert_eq!(local_vars.get_int(0), 100);

        local_vars.set_float(1, 3.14);
        assert_eq!(local_vars.get_float(1), 3.14);

        local_vars.set_long(2, 2997924580);
        assert_eq!(local_vars.get_long(2), 2997924580);

        local_vars.set_double(4, 2.71828182845);
        assert_eq!(local_vars.get_double(4), 2.71828182845);
    }

    #[test]
    fn test_operand_stack() {
        let mut operand_stack = OperandStack::new(10);
        operand_stack.push_int(100);
        assert_eq!(operand_stack.pop_int(), 100);

        operand_stack.push_float(3.14);
        assert_eq!(operand_stack.pop_float(), 3.14);

        operand_stack.push_long(2997924580);
        assert_eq!(operand_stack.pop_long(), 2997924580);

        operand_stack.push_double(2.71828182845);
        assert_eq!(operand_stack.pop_double(), 2.71828182845);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new(10);
        let thread = Rc::new(RefCell::new(Thread::new()));
        let frame = Frame::new(10, 10, Rc::downgrade(&thread));
        stack.push(frame);
        assert_eq!(stack.size, 1);
        let frame = stack.pop();
        assert_eq!(stack.size, 0);
    }

    #[test]
    fn test_thread() {
        let thread = Rc::new(RefCell::new(Thread::new()));
        let frame = Frame::new(10, 10, Rc::downgrade(&thread));
        thread.borrow_mut().push_frame(frame);
        assert_eq!(thread.borrow().stack.borrow().size(), 1);
        let frame = thread.borrow_mut().pop_frame();
        assert_eq!(thread.borrow().stack.borrow().size(), 0);
    }
}
