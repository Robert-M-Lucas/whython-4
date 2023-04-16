use std::fs;
use std::io::Write;
use num_format::{Locale, ToFormattedString};

pub struct MemoryManager {
    pub memory: Vec<u8>
}

impl MemoryManager {
    pub fn new() -> Self {
        Self { memory: Vec::new() }
    }

    pub fn get_position(&self) -> usize { self.memory.len() }

    pub fn append_byte(&mut self, data: u8) -> usize {
        let position = self.get_position();
        self.memory.push(data);
        position
    }

    pub fn append(&mut self, data: &[u8]) -> usize {
        let position = self.get_position();
        self.memory.extend(data);
        position
    }

    pub fn overwrite(&mut self, position: usize, data: &[u8]) {
        for (i, b) in data.into_iter().enumerate() {
            self.memory[position + i] = *b;
        }
    }

    pub fn reserve(&mut self, amount: usize) -> usize {
        let position = self.get_position();
        self.memory.reserve(amount);
        for _ in 0..amount {
            self.memory.push(0);
        }
        position
    }

    pub fn dump_bytes(&self, name: String) {
        let name = name + " - dump.b";
        println!("Dumping memory to file '{}' [{} bytes]",
                 name.clone(), self.memory.len().to_formatted_string(&Locale::en));

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
