use std::fs;
use std::io::Write;

pub struct MemoryManager {
    pub name: String,
    pub memory: Vec<u8>
}

impl MemoryManager {
    pub fn new() -> Self {
        Self { name: "memory".to_string(), memory: Vec::new() }
    }

    pub fn new_named(name: String) -> Self {
        Self { name, memory: Vec::new() }
    }

    pub fn get_postition(&self) -> usize { self.memory.len() }

    pub fn append_byte(&mut self, data: u8) -> usize
    {
        let position = self.get_postition();
        self.memory.push(data);
        position
    }

    pub fn append(&mut self, data: Vec<u8>) -> usize {
        let position = self.get_postition();
        self.memory.extend(data);
        position
    }

    pub fn reserve(&mut self, amount: usize) -> usize {
        let position = self.get_postition();
        self.memory.reserve(amount);
        for _ in 0..amount {
            self.memory.push(0);
        }
        position
    }

    pub fn dump_bytes(&self) {
        let name = self.name.clone() + " - dump.b";
        println!("Dumping memory to file '{}'", name.clone());

        let file = fs::OpenOptions::new().write(true)
            .create(true)
            .open(name);

        if file.is_err() {
            println!("Failed to open file - {}", file.unwrap_err().to_string());
            return;
        }

        let mut file = file.unwrap();
        let r = file.write_all(&self.memory);
        if r.is_err() { println!("Failed to write to file - {}", r.unwrap_err().to_string()) }
    }
}
