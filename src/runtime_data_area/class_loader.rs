use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use crate::{classfile_loader::ClassFileLoader, classpath::ClassPath};

use super::class::Class;

pub struct ClassLoader {
    class_path: Rc<RefCell<ClassPath>>,
    class_map: HashMap<String, Rc<RefCell<Class>>>,
    self_ref: Weak<RefCell<ClassLoader>>,
}

impl ClassLoader {
    pub fn new(class_path: Rc<RefCell<ClassPath>>) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|weak| {
            RefCell::new(Self {
                class_path: class_path.clone(),
                class_map: HashMap::new(),
                self_ref: weak.clone(),
            })
        })
    }

    pub fn load_class(&mut self, name: String) -> Rc<RefCell<Class>> {
        if let Some(class) = self.class_map.get(&name) {
            return class.clone();
        }
        let data = self.read_class(&name);
        let class = self.define_class(data);
        self.link_class(class.clone());
        class
    }

    fn read_class(&self, name: &str) -> Vec<u8> {
        match self.class_path.borrow().read_class(name) {
            Ok(data) => data,
            Err(_) => panic!("java.lang.ClassNotFoundException: {}", name),
        }
    }

    fn define_class(&mut self, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class = self.parse_class(data);
        class.borrow_mut().loader = Some(self.self_ref.clone());
        self.resolve_super_class(&class);
        self.resolve_interfaces(&class);
        self.class_map
            .insert(class.borrow().name.clone(), class.clone());
        class
    }

    fn parse_class(&self, data: Vec<u8>) -> Rc<RefCell<Class>> {
        let class_file = ClassFileLoader::load(data);
        let class = Class::new(class_file);
        class
    }

    fn resolve_super_class(&mut self, class: &Rc<RefCell<Class>>) {
        if class.borrow().name != "java/lang/Object" {
            class.borrow_mut().super_class =
                Some(self.load_class(class.borrow().super_class_name.clone()));
        }
    }

    fn resolve_interfaces(&mut self, class: &Rc<RefCell<Class>>) {
        let interface_count = class.borrow().interface_names.len();
        if interface_count > 0 {
            class.borrow_mut().interfaces = Vec::with_capacity(interface_count);
            for interface_name in class.borrow().interface_names.iter() {
                class
                    .borrow_mut()
                    .interfaces
                    .push(self.load_class(interface_name.clone()));
            }
        }
    }

    fn link_class(&self, class: Rc<RefCell<Class>>) {
        self.verify(class.clone());
        self.prepare(class.clone());
    }

    fn verify(&self, class: Rc<RefCell<Class>>) {
        todo!("Implement class verification logic")
    }

    fn prepare(&self, class: Rc<RefCell<Class>>) {
        todo!("Implement class preparation logic")
    }
}
