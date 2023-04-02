use crate::processing::processor::MemoryManagers;

pub struct StaticBitmask {
    address: usize,
    length: usize
}

impl StaticBitmask {
    pub fn create_bitmask(memory_managers: &mut MemoryManagers, mask: u8) -> Self {
        Self {
            address: memory_managers.variable_memory.append_byte(mask),
            length: 1
        }
    }

    pub fn create_long_mask(memory_managers: &mut MemoryManagers, mask: Vec<u8>) -> Self {
        let length = mask.len();
        Self {
            address: memory_managers.variable_memory.append(mask),
            length
        }
    }
}