struct Chip8 {
    registers: [u8; 16],
    i: u16,
    pc: u16,
    sp: u8,
    stack: [u8; 64],
    memory: [u8; 4096],
}

const ROM_START: u16 = 0x200;

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0x00; 16],
            i: 0x000,
            pc: ROM_START,
            sp: 0x00,
            stack: [0x00; 64],
            memory: [0x00; 4096],
        }
    }

    pub fn ROM_loader(&mut self, rom: &[u8]) -> Result<(), &'static str> {
        // takes all the instructions and puts them in memory starts at 0x200 (for some fun reason). all instructionsgit .
        let end = ROM_START as usize + rom.len();
        if end > self.memory.len() {
            return Err("Your program is too damn long");
        }
        self.memory[ROM_START as usize..end].copy_from_slice(rom);
        self.pc = ROM_START;
        Ok(())
    }

    pub fn fetch(&mut self) -> u16 {
        let byte_1 = self.memory[self.pc as usize];
        let byte_2 = self.memory[self.pc as usize + 1];
        self.pc += 2;

        ((byte_1 as u16) << 8) | byte_2 as u16
    }
}
