use crate::isa::OpCode;

const ROM_START: u16 = 0x200;
const W: usize = 64;
const H: usize = 32;

pub struct Chip8 {
    pub registers: [u8; 16],
    pub i: u16,
    pub pc: u16,
    pub sp: u8,
    dt: u8,
    st: u8,
    pub fb: [[bool; W]; H],
    stack: [u16; 16],
    memory: [u8; 4096],
    pub keypad: [bool; 16]
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            registers: [0x00; 16],
            i: 0x000,
            pc: ROM_START,
            sp: 0x00,
            dt: 0x00,
            st: 0x00,
            fb: [[false; W]; H],
            stack: [0x00; 16],
            memory: [0x00; 4096],
            keypad: [false; 16]
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
        self.pc = self.pc.wrapping_add(2);

        ((byte_1 as u16) << 8) | byte_2 as u16
    }

    fn skip_next(&mut self) {
        self.pc = self.pc.wrapping_add(2);
    }

    pub fn execute(&mut self, op: OpCode) -> Result<(), &'static str> {
        match op {
            OpCode::Cls => {
                self.fb = [[false; W]; H];
            }
            OpCode::Ret => {
                self.sp = self.sp.wrapping_sub(1);
                self.pc = self.stack[self.sp as usize];
            }
            OpCode::Jp { addr } => {
                self.pc = addr;
            }
            OpCode::Call { addr } => {
                self.stack[self.sp as usize] = self.pc;
                self.sp = self.sp.wrapping_add(1);
                self.pc = addr;
            }
            OpCode::SE_vx_byte { x, kk } => {
                if self.registers[x as usize] == kk {
                    self.skip_next();
                }
            }
            OpCode::SNE_vx_byte { x, kk } => {
                if self.registers[x as usize] != kk {
                    self.skip_next();
                }
            }
            OpCode::SE_vx_vy { x, y } => {
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.skip_next();
                }
            }
            OpCode::SNE_vx_vy { x, y } => {
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.skip_next();
                }
            }
            OpCode::LD_vx_byte { x, kk } => {
                self.registers[x as usize] = kk;
            }
            OpCode::ADD_vx_byte { x, kk } => {
                let vx = self.registers[x as usize];
                self.registers[x as usize] = vx.wrapping_add(kk);
            }
            OpCode::LD_vx_vy { x, y } => {
                self.registers[x as usize] = self.registers[y as usize];
            }
            OpCode::OR_vx_vy { x, y } => {
                self.registers[x as usize] |= self.registers[y as usize];
                self.registers[0xF] = 0;
            }
            OpCode::AND_vx_vy { x, y } => {
                self.registers[x as usize] &= self.registers[y as usize];
                self.registers[0xF] = 0;
            }
            OpCode::XOR_vx_vy { x, y } => {
                self.registers[x as usize] ^= self.registers[y as usize];
                self.registers[0xF] = 0;
            }
            OpCode::ADD_vx_vy { x, y } => {
                let a = self.registers[x as usize] as u16;
                let b = self.registers[y as usize] as u16;
                let s = a + b;
                self.registers[x as usize] = s as u8;
                self.registers[0xF] = if s > 0xFF { 1 } else { 0 };
            }
            OpCode::SUB_vx_vy { x, y } => {
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                self.registers[0xF] = if vx > vy { 1 } else { 0 };
                self.registers[x as usize] = vx.wrapping_sub(vy);
            }
            OpCode::SHR_vx_vy { x, .. } => {
                let vx = self.registers[x as usize];
                self.registers[0xF] = vx & 0x01;
                self.registers[x as usize] = vx >> 1;
            }
            OpCode::SUBN_vx_vy { x, y } => {
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                self.registers[0xF] = if vy > vx { 1 } else { 0 };
                self.registers[x as usize] = vy.wrapping_sub(vx);
            }
            OpCode::SHL_vx_vy { x, .. } => {
                let vx = self.registers[x as usize];
                self.registers[0xF] = (vx >> 7) & 0x01;
                self.registers[x as usize] = vx << 1;
            }
            OpCode::LD_I_addr { addr } => {
                self.i = addr;
            }
            OpCode::JP_v0_addr { addr } => {
                self.pc = addr.wrapping_add(self.registers[0] as u16);
            }
            OpCode::RND_vx_byte { x, kk } => {
                let r: u8 = rand::random();
                self.registers[x as usize] = r & kk;
            }
            OpCode::DRW_x_y_nibble { x, y, n } => {
                let vx = self.registers[x as usize];
                let vy = self.registers[y as usize];
                self.draw_sprite(vx, vy, n);
            }
            OpCode::SKP_vx { x } => {
                let key = self.registers[x as usize] as usize;
                if key < 16 && self.keypad[key] { self.skip_next(); }
            }
            OpCode::SKNP_vx { x } => {
                let key = self.registers[x as usize] as usize;
                if key < 16 && !self.keypad[key] { self.skip_next(); }
            }
            OpCode::LD_vx_k { x } => {
                if let Some(k) = self.keypad.iter().position(|&pressed| pressed) {
                    self.registers[x as usize] = k as u8;
                } else {
                    self.pc = self.pc.wrapping_sub(2);
                }
            }
            OpCode::LD_vx_dt { x } => {
                self.registers[x as usize] = self.dt;
            }
            OpCode::LD_dt_vx { x } => {
                self.dt = self.registers[x as usize];
            }
            OpCode::LD_st_vx { x } => {
                self.st = self.registers[x as usize];
            }
            OpCode::ADD_I_vx { x } => {
                self.i = self.i.wrapping_add(self.registers[x as usize] as u16);
            }
            OpCode::LD_F_vx { x } => {
                self.i = (self.registers[x as usize] as u16) * 5;
            }
            OpCode::LD_B_vx { x } => {
                let v = self.registers[x as usize];
                self.memory[self.i as usize] = v / 100;
                self.memory[self.i as usize + 1] = (v / 10) % 10;
                self.memory[self.i as usize + 2] = v % 10;
            }
            OpCode::LD_I_vx { x } => {
                let end = x as usize;
                for r in 0..=end {
                    let addr = self.i.wrapping_add(r as u16) as usize;
                    self.memory[addr] = self.registers[r];
                }
                self.i = self.i.wrapping_add(end as u16 + 1);
            }
            OpCode::LD_vx_I { x } => {
                let end = x as usize;
                for r in 0..=end {
                    let addr = self.i.wrapping_add(r as u16) as usize;
                    self.registers[r] = self.memory[addr];
                }
                self.i = self.i.wrapping_add(end as u16 + 1);
            }
        }
        Ok(())
    }

    pub fn draw_sprite(&mut self, vx:u8, vy:u8, n:u8) {
        self.registers[0xF] = 0;
        let x0 = vx as usize % W;
        let y0 = vy as usize % H;

        for row in 0..(n as usize) {
            let sprite_byte = self.memory[self.i.wrapping_add(row as u16) as usize];
            let y = (y0 + row) % H;

            for bit in 0..8 {
                let x = (x0 + bit) % W;
                let pixel_on = (sprite_byte >> (7-bit)) & 1 == 1;

                if pixel_on {
                    let prev = self.fb[y][x];
                    let new = !prev;
                    self.fb[y][x] = new;
                    if prev && !new {
                        self.registers[0xF] = 1;
                    }
                }
            }

        }
    }

    pub fn tick_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }
        if self.st > 0 {
            self.st -= 1;
        }
    }

}
