use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use std::time::{Duration, Instant};

use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::chip8::Chip8;
use crate::isa::OpCode;

pub struct App {
    chip8: Chip8,
    instruction_history: Vec<OpCode>,
    exit: bool,
}

fn map_key_to_chip8(code: KeyCode) -> Option<usize> {
    match code {
        KeyCode::Char('1') => Some(0x1),
        KeyCode::Char('2') => Some(0x2),
        KeyCode::Char('3') => Some(0x3),
        KeyCode::Char('4') => Some(0xC),

        KeyCode::Char('q') | KeyCode::Char('Q') => Some(0x4),
        KeyCode::Char('w') | KeyCode::Char('W') => Some(0x5),
        KeyCode::Char('e') | KeyCode::Char('E') => Some(0x6),
        KeyCode::Char('r') | KeyCode::Char('R') => Some(0xD),

        KeyCode::Char('a') | KeyCode::Char('A') => Some(0x7),
        KeyCode::Char('s') | KeyCode::Char('S') => Some(0x8),
        KeyCode::Char('d') | KeyCode::Char('D') => Some(0x9),
        KeyCode::Char('f') | KeyCode::Char('F') => Some(0xE),

        KeyCode::Char('z') | KeyCode::Char('Z') => Some(0xA),
        KeyCode::Char('x') | KeyCode::Char('X') => Some(0x0),
        KeyCode::Char('c') | KeyCode::Char('C') => Some(0xB),
        KeyCode::Char('v') | KeyCode::Char('V') => Some(0xF),
        _ => None,
    }
}

impl App {
    pub fn new(rom: &[u8]) -> Self {
        let mut chip8 = Chip8::new();
        chip8.ROM_loader(rom).expect("Invalid ROM");
        Self {
            chip8,
            instruction_history: Vec::new(),
            exit: false,
        }
    }

    pub fn add_instruction(&mut self, opcode: OpCode) {
        self.instruction_history.push(opcode);
        if self.instruction_history.len() > 15 {
            self.instruction_history.remove(0);
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut last_timer = Instant::now();

        while !self.exit {
            let instruction = OpCode::from_u16(self.chip8.fetch());
            self.add_instruction(instruction);
            self.chip8.execute(instruction).unwrap();

            if last_timer.elapsed() >= Duration::from_micros(16_667) {
                self.chip8.tick_timers();
                last_timer = Instant::now();
            }
            
            terminal.draw(|frame| self.draw(frame))?;

            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    self.exit = true;
                }
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Press,
                    ..
                }) => {
                    if let Some(k) = map_key_to_chip8(code) {
                        self.chip8.keypad[k] = true;
                    }
                }
                Event::Key(KeyEvent {
                    code,
                    kind: KeyEventKind::Release,
                    ..
                }) => {
                    if let Some(k) = map_key_to_chip8(code) {
                        self.chip8.keypad[k] = false;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::bordered()
            .title(Line::from(" Chip8 ".bold()).centered())
            .border_set(border::THICK);
        let inner_area = main_block.inner(area);
        main_block.render(area, buf);

        // Layout
        let horizontal_layout =
            Layout::horizontal([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(inner_area);

        // Display
        self.render_pixel_display(horizontal_layout[0], buf);

        // Right side layout
        let right_layout =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(horizontal_layout[1]);

        // Registers
        let register_text = Text::from(vec![
            Line::from(format!(
                "V0:{:02X} V1:{:02X} V2:{:02X} V3:{:02X}",
                self.chip8.registers[0],
                self.chip8.registers[1],
                self.chip8.registers[2],
                self.chip8.registers[3]
            )),
            Line::from(format!(
                "V4:{:02X} V5:{:02X} V6:{:02X} V7:{:02X}",
                self.chip8.registers[4],
                self.chip8.registers[5],
                self.chip8.registers[6],
                self.chip8.registers[7]
            )),
            Line::from(format!(
                "V8:{:02X} V9:{:02X} VA:{:02X} VB:{:02X}",
                self.chip8.registers[8],
                self.chip8.registers[9],
                self.chip8.registers[10],
                self.chip8.registers[11]
            )),
            Line::from(format!(
                "VC:{:02X} VD:{:02X} VE:{:02X} VF:{:02X}",
                self.chip8.registers[12],
                self.chip8.registers[13],
                self.chip8.registers[14],
                self.chip8.registers[15]
            )),
            Line::from(format!(
                "I:{:04X} PC:{:04X} SP:{:02X}",
                self.chip8.i, self.chip8.pc, self.chip8.sp
            )),
        ]);
        let register_block = Block::bordered()
            .title(Line::from(" Registers ".bold()).centered())
            .border_set(border::THICK);
        let register_paragraph = Paragraph::new(register_text)
            .block(register_block)
            .centered();
        register_paragraph.render(right_layout[0], buf);

        // Instruction history
        let history_lines: Vec<Line> = self
            .instruction_history
            .iter()
            .map(|opcode| Line::from(opcode.to_string()))
            .collect();
        let history_text = Text::from(history_lines);
        let history_block = Block::bordered()
            .title(Line::from(" History ".bold()).centered())
            .border_set(border::THICK);
        let history_paragraph = Paragraph::new(history_text).block(history_block).centered();
        history_paragraph.render(right_layout[1], buf);
    }
}

impl App {
    fn render_pixel_display(&self, area: Rect, buf: &mut Buffer) {
        let pixel_text = self.frame_buffer_to_text();
        let pixel_block = Block::bordered()
            .title(Line::from(" Display ".bold()).centered())
            .border_set(border::THICK);
        let pixel_paragraph = Paragraph::new(pixel_text).block(pixel_block).centered();
        pixel_paragraph.render(area, buf);
    }

    fn frame_buffer_to_text(&self) -> Text {
        let mut lines = Vec::new();
        for y in 0..32 {
            let mut line_chars = String::new();
            for x in 0..64 {
                let pixel = if self.chip8.fb[y][x] { '█' } else { '░' };
                // 2:1 scaling looks better imo
                line_chars.push(pixel);
                line_chars.push(pixel);
            }
            lines.push(Line::from(line_chars));
        }

        Text::from(lines)
    }
}

impl OpCode {
    fn to_string(&self) -> String {
        match self {
            OpCode::Cls => "CLS".to_string(),
            OpCode::Ret => "RET".to_string(),
            OpCode::Jp { addr } => format!("JP 0x{:03X}", addr),
            OpCode::Call { addr } => format!("CALL 0x{:03X}", addr),
            OpCode::SE_vx_byte { x, kk } => format!("SE V{}, 0x{:02X}", x, kk),
            OpCode::SNE_vx_byte { x, kk } => format!("SNE V{}, 0x{:02X}", x, kk),
            OpCode::SE_vx_vy { x, y } => format!("SE V{}, V{}", x, y),
            OpCode::LD_vx_byte { x, kk } => format!("LD V{}, 0x{:02X}", x, kk),
            OpCode::ADD_vx_byte { x, kk } => format!("ADD V{}, 0x{:02X}", x, kk),
            OpCode::LD_vx_vy { x, y } => format!("LD V{}, V{}", x, y),
            OpCode::OR_vx_vy { x, y } => format!("OR V{}, V{}", x, y),
            OpCode::AND_vx_vy { x, y } => format!("AND V{}, V{}", x, y),
            OpCode::XOR_vx_vy { x, y } => format!("XOR V{}, V{}", x, y),
            OpCode::ADD_vx_vy { x, y } => format!("ADD V{}, V{}", x, y),
            OpCode::SUB_vx_vy { x, y } => format!("SUB V{}, V{}", x, y),
            OpCode::SHR_vx_vy { x, y } => format!("SHR V{}, V{}", x, y),
            OpCode::SUBN_vx_vy { x, y } => format!("SUBN V{}, V{}", x, y),
            OpCode::SHL_vx_vy { x, y } => format!("SHL V{}, V{}", x, y),
            OpCode::SNE_vx_vy { x, y } => format!("SNE V{}, V{}", x, y),
            OpCode::LD_I_addr { addr } => format!("LD I, 0x{:03X}", addr),
            OpCode::JP_v0_addr { addr } => format!("JP V0, 0x{:03X}", addr),
            OpCode::RND_vx_byte { x, kk } => format!("RND V{}, 0x{:02X}", x, kk),
            OpCode::DRW_x_y_nibble { x, y, n } => format!("DRW V{}, V{}, {}", x, y, n),
            OpCode::SKP_vx { x } => format!("SKP V{}", x),
            OpCode::SKNP_vx { x } => format!("SKNP V{}", x),
            OpCode::LD_vx_dt { x } => format!("LD V{}, DT", x),
            OpCode::LD_vx_k { x } => format!("LD V{}, K", x),
            OpCode::LD_dt_vx { x } => format!("LD DT, V{}", x),
            OpCode::LD_st_vx { x } => format!("LD ST, V{}", x),
            OpCode::ADD_I_vx { x } => format!("ADD I, V{}", x),
            OpCode::LD_F_vx { x } => format!("LD F, V{}", x),
            OpCode::LD_B_vx { x } => format!("LD B, V{}", x),
            OpCode::LD_I_vx { x } => format!("LD I, V{}", x),
            OpCode::LD_vx_I { x } => format!("LD V{}, I", x),
        }
    }
}
