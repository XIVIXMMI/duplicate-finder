use std::path::PathBuf;

// a struct in rust equivalent to class in Java
pub struct ImageFile {
    pub path: PathBuf,
    #[allow(dead_code)]
    pub size: u64,
}

impl ImageFile {
    pub fn new(path: PathBuf, size: u64) -> Self {
        ImageFile { path, size }
    }

    #[allow(dead_code)]
    pub fn display(&self) {
        // &self = borrow, no ownership
        println!("File: {:?}, Size: {:?} byte", self.path, self.size)
    }
}
