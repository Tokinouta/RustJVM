use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

use zip::ZipArchive;

pub trait Entry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String>;
}

pub fn new_entry(path: String) -> Box<dyn Entry> {
    if path.contains(';') {
        Box::new(CompositeEntry::new(path))
    } else if path.ends_with("*") {
        Box::new(CompositeEntry::new_wildcard(path))
    } else if path.ends_with(".jar")
        || path.ends_with(".JAR")
        || path.ends_with(".zip")
        || path.ends_with(".ZIP")
    {
        Box::new(ZipEntry::new(path))
    } else {
        Box::new(DirEntry::new(path))
    }
}

pub struct DirEntry {
    abs_dir: PathBuf,
}

impl DirEntry {
    pub fn new(path: String) -> DirEntry {
        let path = fs::canonicalize(path).unwrap();
        DirEntry { abs_dir: path }
    }
}

impl Entry for DirEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        let file_path = self.abs_dir.join(class_name);
        match fs::read(file_path) {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Error reading class file: {}", e)),
        }
    }
}

pub struct ZipEntry {
    abs_path: PathBuf,
}

impl ZipEntry {
    pub fn new(path: String) -> ZipEntry {
        let path = fs::canonicalize(path).unwrap();
        ZipEntry { abs_path: path }
    }
}

impl Entry for ZipEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        let file =
            File::open(self.abs_path.clone()).map_err(|e| format!("Error opening file: {}", e))?;
        let mut archive =
            ZipArchive::new(file).map_err(|e| format!("Error reading zip file: {}", e))?;
        let mut class = match archive.by_name(class_name) {
            Ok(f) => f,
            Err(_) => return Err(format!("Class not found: {}", class_name)),
        };
        let mut data = Vec::new();
        class
            .read_to_end(&mut data)
            .map_err(|e| format!("Error reading class file: {}", e))?;
        Ok(data)
    }
}

pub struct CompositeEntry {
    entries: Vec<Box<dyn Entry>>,
}

impl CompositeEntry {
    pub fn new(path: String) -> CompositeEntry {
        let mut entries = Vec::new();
        for entry in path.split(';') {
            entries.push(new_entry(entry.to_string()));
        }
        CompositeEntry { entries }
    }

    pub fn new_wildcard(path: String) -> Self {
        let path = fs::canonicalize(path.trim_end_matches("*")).unwrap();
        let files = fs::read_dir(path).unwrap();
        let mut ret = Self { entries: vec![] };
        for file in files {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();
            if file_name.ends_with(".jar") || file_name.ends_with(".JAR") {
                ret.entries.push(Box::new(ZipEntry::new(file_name)));
            }
        }
        ret
    }
}

impl Entry for CompositeEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, String> {
        for entry in &self.entries {
            match entry.read_class(class_name) {
                Ok(data) => return Ok(data),
                Err(_) => continue,
            }
        }
        Err(format!("Class not found: {}", class_name))
    }
}
