extern crate peroxide;

use std::io;

use ratatui::{
    DefaultTerminal, 
    Frame, 
    crossterm::event::{self, Event, KeyCode}, 
    layout::{Constraint, Layout, Rect}, 
    style::{Color, Style, Stylize}, 
    text::{Line, Text}, widgets::{Block, List, ListState, Paragraph}
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

mod calculator;

fn main() -> io::Result<()> {
    ratatui::run(|t| App::default().run(t))
}

/// App holds the state of the application
#[derive(Debug, Default)]
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    input_mode: InputMode,
    /// History of recorded messages
    messages: Vec<String>,
    /// State of the List widget containing messages
    message_widget_state: ListState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    #[default]
    Normal,
    Editing(EditMode),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum EditMode {
    #[default]
    Input,
    PolynomialFinder
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            let event = event::read()?;
            if let Event::Key(key) = event {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('i') => self.start_editing(EditMode::Input),
                        KeyCode::Char('p') => self.start_editing(EditMode::PolynomialFinder),
                        KeyCode::Char('c') => self.messages.clear(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        k => self.handle_message_scoll(k),
                    },
                    InputMode::Editing(_) => match key.code {
                        KeyCode::Enter => self.push_message(),
                        KeyCode::Esc => self.stop_editing(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                }
            }
        }
    }

    fn start_editing(&mut self, mode: EditMode) {
        self.input_mode = InputMode::Editing(mode)
    }

    fn stop_editing(&mut self) {
        self.input_mode = InputMode::Normal
    }

    fn push_message(&mut self) {
        let msg = self.input.value_and_reset();

        match self.input_mode {
            InputMode::Editing(EditMode::Input) => self.messages.push(msg),
            InputMode::Editing(EditMode::PolynomialFinder) if msg.is_empty() => {
                use calculator::polynomial::{solve_by_index};
                match solve_by_index(&self.messages) {
                    Ok(equation) => self.messages.push(equation),
                    Err(err) => self.messages.push(format!("An error occured: {:}", err)),
                }
            }
            InputMode::Editing(EditMode::PolynomialFinder) => self.messages.push(msg),
            InputMode::Normal => {},
        }
        
    }

    fn render(&mut self, frame: &mut Frame) {
        let [header_area, input_area, messages_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .areas(frame.area());

        self.render_header(frame, header_area);
        self.render_input(frame, input_area);
        self.render_messages(frame, messages_area);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Line::from(match self.input_mode {
            InputMode::Normal => "Command mode".bold(),
            InputMode::Editing(EditMode::Input) => "Input mode".bold(),
            InputMode::Editing(EditMode::PolynomialFinder) => "Polynomial Finder Mode".bold(),
        });

        let help_message = Line::from_iter(match self.input_mode {
            InputMode::Normal => vec![
                " q ".bold(),
                "exit".on_blue(),
                " c ".bold(),
                "clear output".on_light_blue(),
                " i ".bold(),
                "input points".on_light_blue(),
                " e ".bold(),
                "edit points".on_light_blue(),
                " p ".bold(),
                "polynomial finder mode".on_light_blue(),
            ],
            InputMode::Editing(EditMode::Input) => vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to record the message".into(),
            ],
            InputMode::Editing(EditMode::PolynomialFinder) => vec![
                "Press ".into(),
                "Esc".bold(),
                " to stop editing, ".into(),
                "Enter".bold(),
                " to calculate the polynomial".into(),
            ]
        });

        let header = Text::from(vec![title, help_message]);
        frame.render_widget(header, area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.input_mode {
            InputMode::Normal => Style::default(),
            InputMode::Editing(_) => Color::Yellow.into(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, area);

        if let InputMode::Editing(_) = self.input_mode {
            // Ratatui hides the cursor unless it's explicitly set. Position the  cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    fn render_messages(&mut self, frame: &mut Frame, area: Rect) {
        let messages = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, message)| format!("{}: {}", i, message));
        let messages = List::new(messages).block(Block::bordered().title("Messages"));
        frame.render_stateful_widget(messages, area, &mut self.message_widget_state);
    }

    fn handle_message_scoll(&mut self, keycode: KeyCode) {
        match keycode {
            KeyCode::Up => self.message_widget_state.scroll_up_by(1),
            KeyCode::Down => self.message_widget_state.scroll_down_by(1),
            _ => {}
        }
    }
}
