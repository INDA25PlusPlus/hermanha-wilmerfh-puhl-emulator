use std::io;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

use crate::chip8::Chip8;

pub struct App {
    chip8: Chip8,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            chip8: Chip8::new(),
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;

            if event::poll(std::time::Duration::from_millis(100))? {
                if let Event::Key(KeyEvent {
                    kind: KeyEventKind::Press,
                    ..
                }) = event::read()?
                {
                    self.exit = true;
                }
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

        let register_text = Text::from(vec![
            Line::from(format!(
                "V0: {:02X}  V1: {:02X}  V2: {:02X}  V3: {:02X}",
                self.chip8.registers[0],
                self.chip8.registers[1],
                self.chip8.registers[2],
                self.chip8.registers[3]
            )),
            Line::from(format!(
                "V4: {:02X}  V5: {:02X}  V6: {:02X}  V7: {:02X}",
                self.chip8.registers[4],
                self.chip8.registers[5],
                self.chip8.registers[6],
                self.chip8.registers[7]
            )),
            Line::from(format!(
                "V8: {:02X}  V9: {:02X}  VA: {:02X}  VB: {:02X}",
                self.chip8.registers[8],
                self.chip8.registers[9],
                self.chip8.registers[10],
                self.chip8.registers[11]
            )),
            Line::from(format!(
                "VC: {:02X}  VD: {:02X}  VE: {:02X}  VF: {:02X}",
                self.chip8.registers[12],
                self.chip8.registers[13],
                self.chip8.registers[14],
                self.chip8.registers[15]
            )),
            Line::from(""),
            Line::from(format!(
                "I: {:04X}  PC: {:04X}  SP: {:02X}",
                self.chip8.i, self.chip8.pc, self.chip8.sp
            )),
        ]);

        let register_block = Block::bordered()
            .title(Line::from(" Registers ".bold()).centered())
            .border_set(border::THICK);
        let register_paragraph = Paragraph::new(register_text)
            .block(register_block)
            .centered();

        register_paragraph.render(inner_area, buf);
    }
}
