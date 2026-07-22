//! # Main app render entry
//! Usage of unwrap is ok because right now filetable is the only thing that can be selected
use std::io;

use crate::{
    filesystem::{FileSystem, node::NodeType},
    renderer::filetable::fill_filetable,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::Style,
    text::Line,
    widgets::{Block, StatefulWidget, Table, TableState, Widget},
};

#[derive(Debug, Default)]
pub struct App {
    exit: bool,

    filesystem: FileSystem,

    current_path: Vec<usize>,
    filetable_state: TableState,

    draw_dots: bool,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        filesystem: FileSystem,
    ) -> io::Result<()> {
        self.filesystem = filesystem;
        self.current_path.push(0);

        self.filetable_state = TableState::default();

        self.filetable_state.select(Some(0));
        self.draw_dots = true;

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

    /// Handle key presses
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('j') | KeyCode::Down => self.cursor_down(),
            KeyCode::Char('k') | KeyCode::Up => self.cursor_up(),
            KeyCode::Char('h') | KeyCode::Left => self.path_up(),
            KeyCode::Char('l') | KeyCode::Right => self.path_down(),
            KeyCode::Char('s') => self.switch_dots(),
            KeyCode::Char('d') => self.delete_entry(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn cursor_down(&mut self) {
        if self.filetable_state.selected().unwrap()
            < self.filesystem.arena[self.current_node()].num_children() - 1
        {
            self.filetable_state.select_next();
        }
    }

    fn cursor_up(&mut self) {
        self.filetable_state.select_previous();
    }

    fn path_up(&mut self) {
        if self.current_path.len() > 1 {
            self.current_path.pop();
            self.filetable_state.select(Some(0));
        }
    }

    fn path_down(&mut self) {
        let current = self.current_node();
        let Some(cp) = self.filetable_state.selected() else {
            return;
        };

        if let NodeType::Dir =
            self.filesystem.arena[self.filesystem.arena[current].children[cp]].node_type
        {
            self.current_path
                .push(self.filesystem.arena[current].children[cp]);
            self.filetable_state.select(Some(0));
        }
    }

    fn current_node(&self) -> usize {
        *self.current_path.last().unwrap()
    }

    fn current_path_str(&self) -> String {
        self.current_path
            .iter()
            .map(|&node_index| self.filesystem.arena[node_index].name())
            .collect::<Vec<_>>()
            .join("/")
    }

    fn switch_dots(&mut self) {
        self.draw_dots = !self.draw_dots;
    }

    fn delete_entry(&mut self) {
        let _selected = self.filetable_state.selected();
        todo!()
    }
}

impl Widget for &App {
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

        let mut table: Table =
            fill_filetable(&self.filesystem, self.current_node(), self.draw_dots);
        let current_path_line: Line = Line::from(self.current_path_str());

        table = table.row_highlight_style(Style::default().on_yellow());

        StatefulWidget::render(
            table,
            filetableblock.inner(filetablerect),
            buf,
            &mut self.filetable_state.clone(),
        );
        filetableblock.render(filetablerect, buf);
        current_path_line.render(pathblock.inner(pathrect), buf);
        pathblock.render(pathrect, buf);
    }
}
