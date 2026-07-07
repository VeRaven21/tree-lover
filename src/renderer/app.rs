use std::io;

use crate::{node::DirNode, renderer::filetable};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Table, Widget},
};

#[derive(Debug, Default)]
pub struct App<'a> {
    exit: bool,

    current_path: Vec<&'a DirNode>,
    cursor: usize,
}

impl<'a> App<'a> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, node: &'a DirNode) -> io::Result<()> {
        self.current_path.push(node);
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('j') => self.cursor_down(),
            KeyCode::Char('k') => self.cursor_up(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn cursor_down(&mut self) {
        match self.current_node() {
            Some(node) => {
                if self.cursor < node.num_entries() - 1 {
                    self.cursor += 1;
                }
            }
            None => {}
        }
    }

    fn cursor_up(&mut self) {
        match self.current_node() {
            Some(_node) => {
                if self.cursor > 0 {
                    self.cursor -= 1;
                }
            }
            None => {}
        }
    }

    fn current_node(&self) -> Option<&DirNode> {
        self.current_path.last().map(|a| &**a)
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Tree Lover");

        let main_box = Block::bordered()
            .title(title.centered())
            .border_style(Style::new());
        let inner_area = main_box.inner(area);
        main_box.render(area, buf);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(inner_area);

        let pathrect = chunks[0];
        let pathblock = Block::bordered();
        let filetablerect = chunks[1];
        let filetableblock = Block::bordered();

        let table: Table;
        let current_dir: Line;
        match self.current_node() {
            Some(node) => {
                current_dir = Line::from(node.name());
                table = filetable::fill_filetable(node);
            }
            None => {
                current_dir = Line::default();
                table = Table::default();
            }
        }

        table.render(filetableblock.inner(filetablerect), buf);
        filetableblock.render(filetablerect, buf);
        current_dir.render(pathblock.inner(pathrect), buf);
        pathblock.render(pathrect, buf);
    }
}
