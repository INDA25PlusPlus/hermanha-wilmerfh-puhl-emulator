pub enum OpCode {
    Cls,
    Ret,
    Jp { addr: u16 },
    Call { addr: u16 },
    SE_vx_byte { x: u8, kk: u8 },
    SNE_vx_byte { x: u8, kk: u8 },
    SE_vx_vy { x: u8, y: u8 },
    LD_vx_byte { x: u8, kk: u8 },
    ADD_vx_byte { x: u8, kk: u8 },
    LD_vx_vy { x: u8, y: u8 },
    OR_vx_vy { x: u8, y: u8 },
    AND_vx_vy { x: u8, y: u8 },
    XOR_vx_vy { x: u8, y: u8 },
    ADD_vx_vy { x: u8, y: u8 },
    SUB_vx_vy { x: u8, y: u8 },
    SHR_vx_vy { x: u8, y: u8 },
    SUBN_vx_vy { x: u8, y: u8 },
    SHL_vx_vy { x: u8, y: u8 },
    SNE_vx_vy { x: u8, y: u8 },
    LD_I_addr { addr: u16 },
    JP_v0_addr { addr: u16 },
    RND_vx_byte { x: u8, kk: u8 },
    DRW_x_y_nibble { x: u8, y: u8, n: u8 },
    SKP_vx { x: u8 },
    SKNP_vx { x: u8 },
    LD_vx_dt { x: u8 },
    LD_vx_k { x: u8 },
    LD_dt_vx { x: u8 },
    LD_st_vx { x: u8 },
    ADD_I_vx { x: u8 },
    LD_F_vx { x: u8 },
    LD_B_vx { x: u8 },
    LD_I_vx { x: u8 },
    LD_vx_I { x: u8 },
}

impl OpCode {
    pub fn from_u16(self: &Self, opcode: u16) -> OpCode {
        let opcode_arr: [u8; 4] = [
            (opcode & 0x0F) as u8,
            ((opcode >> 4) & 0x0F) as u8,
            ((opcode >> 8) & 0x0F) as u8,
            ((opcode >> 12) & 0x0F) as u8,
        ];
        match opcode_arr {
            [0x0, 0x0, 0xE, 0x0] => OpCode::Cls,
            [0x0, 0x0, 0xE, 0xE] => OpCode::Ret,
            [0x1, n2, n3, n4] => {
                let addr = ((n2 as u16) << 8) | ((n3 as u16) << 4) | (n4 as u16);
                OpCode::Jp { addr }
            }
            [0x2, n2, n3, n4] => {
                let addr = ((n2 as u16) << 8) | ((n3 as u16) << 4) | (n4 as u16);
                OpCode::Call { addr }
            }
            [0x3, x, k1, k2] => {
                let kk = (k1 << 4) | k2;
                OpCode::SE_vx_byte { x, kk }
            }
            [0x4, x, k1, k2] => {
                let kk = (k1 << 4) | k2;
                OpCode::SNE_vx_byte { x, kk }
            }
            [0x5, x, y, 0x0] => OpCode::SE_vx_vy { x, y },
            [0x6, x, k1, k2] => {
                let kk = (k1 << 4) | k2;
                OpCode::LD_vx_byte { x, kk }
            }
            [0x7, x, k1, k2] => {
                let kk = (k1 << 4) | k2;
                OpCode::ADD_vx_byte { x, kk }
            }
            [0x8, x, y, 0x0] => OpCode::LD_vx_vy { x, y },
            [0x8, x, y, 0x1] => OpCode::OR_vx_vy { x, y },
            [0x8, x, y, 0x2] => OpCode::AND_vx_vy { x, y },
            [0x8, x, y, 0x3] => OpCode::XOR_vx_vy { x, y },
            [0x8, x, y, 0x4] => OpCode::ADD_vx_vy { x, y },
            [0x8, x, y, 0x5] => OpCode::SUB_vx_vy { x, y },
            [0x8, x, y, 0x6] => OpCode::SHR_vx_vy { x, y },
            [0x8, x, y, 0x7] => OpCode::SUBN_vx_vy { x, y },
            [0x8, x, y, 0xE] => OpCode::SHL_vx_vy { x, y },
            [0x9, x, y, 0x0] => OpCode::SNE_vx_vy { x, y },
            [0xA, n2, n3, n4] => {
                let addr = ((n2 as u16) << 8) | ((n3 as u16) << 4) | (n4 as u16);
                OpCode::LD_I_addr { addr }
            }
            [0xB, n2, n3, n4] => {
                let addr = ((n2 as u16) << 8) | ((n3 as u16) << 4) | (n4 as u16);
                OpCode::JP_v0_addr { addr }
            }
            [0xC, x, k1, k2] => {
                let kk = (k1 << 4) | k2;
                OpCode::RND_vx_byte { x, kk }
            }
            [0xD, x, y, n] => OpCode::DRW_x_y_nibble { x, y, n },
            [0xE, x, 0x9, 0xE] => OpCode::SKP_vx { x },
            [0xE, x, 0xA, 0x1] => OpCode::SKNP_vx { x },
            [0xF, x, 0x0, 0x7] => OpCode::LD_vx_dt { x },
            [0xF, x, 0x0, 0xA] => OpCode::LD_vx_k { x },
            [0xF, x, 0x1, 0x5] => OpCode::LD_dt_vx { x },
            [0xF, x, 0x1, 0x8] => OpCode::LD_st_vx { x },
            [0xF, x, 0x1, 0xE] => OpCode::ADD_I_vx { x },
            [0xF, x, 0x2, 0x9] => OpCode::LD_F_vx { x },
            [0xF, x, 0x3, 0x3] => OpCode::LD_B_vx { x },
            [0xF, x, 0x5, 0x5] => OpCode::LD_I_vx { x },
            [0xF, x, 0x6, 0x5] => OpCode::LD_vx_I { x },
            _ => unimplemented!("Unknown opcode: 0x{:04X}", opcode),
        }
    }
}
