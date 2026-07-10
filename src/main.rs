extern crate peroxide;

use std::io;

use ratatui::{
    DefaultTerminal, 
    Frame, 
    crossterm::event::{self, Event, KeyCode}, 
    layout::{Constraint, Layout, Rect}, 
    style::{Color, Style, Stylize}, 
    text::{Line, Text}, widgets::{Block, Clear, List, ListState, Padding, Paragraph}
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
    mode: Mode,
    /// History of recorded messages
    messages: Vec<String>,
    /// State of the List widget containing messages
    message_widget_state: ListState,
    /// Text to show on popup box
    popup_message: String
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Input,
    Edit,
    Popup,
}

impl App {
    fn run(mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;

            let event = event::read()?;
            if let Event::Key(key) = event {
                match self.mode {
                    Mode::Popup => self.stop_popup(),
                    Mode::Normal => match key.code {
                        KeyCode::Char('i') => self.start_input(),
                        KeyCode::Char('e') => self.start_edit(),
                        KeyCode::Char('p') => self.find_polynomial(),
                        KeyCode::Char('c') => self.messages.clear(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        _ => {}
                    },
                    Mode::Input => match key.code {
                        KeyCode::Enter => self.push_message(),
                        KeyCode::Esc => self.stop_input(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                    Mode::Edit => match key.code {
                        KeyCode::Down => self.scroll_points(-1),
                        KeyCode::Up => self.scroll_points(1),
                        KeyCode::Delete => self.delete_selected_point(),
                        KeyCode::Esc => self.stop_edit(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn start_input(&mut self) {
        self.mode = Mode::Input
    }

    fn stop_input(&mut self) {
        self.mode = Mode::Normal
    }

    fn start_popup(&mut self, message: String) {
        self.popup_message = message;
        self.mode = Mode::Popup
    }

    fn stop_popup(&mut self) {
        self.mode = Mode::Normal
    }

    fn start_edit(&mut self) {
        self.message_widget_state.select_first();
        self.mode = Mode::Edit
    }

    fn stop_edit(&mut self) {
        self.message_widget_state.select(None);
        self.mode = Mode::Normal
    }

    /// positve n scrolls up
    fn scroll_points(&mut self, n: i32) {
        if n > 0 {
            self.message_widget_state.scroll_up_by(n as u16);
        } else {
            self.message_widget_state.scroll_down_by((-n) as u16);
        }
    }

    fn delete_selected_point(&mut self) {
        let Some(n) = self.message_widget_state.selected() else { return; };
        // By right should not panic since selected item will always be in range?
        self.messages.remove(n);
    }

    fn push_message(&mut self) {
        let msg = self.input.value_and_reset();

        if self.mode == Mode::Input {
            self.messages.push(msg)
        }
    }

    fn find_polynomial(&mut self) {
        use calculator::polynomial::solve_by_index;
        let message = match solve_by_index(&self.messages) {
            Ok(equation) => equation,
            Err(err) => format!("An error occured: {:}", err),
        };

        self.start_popup(message);
    }

    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [header_area, input_area, messages_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .areas(area);

        let popup_area = area.centered(Constraint::Percentage(60), Constraint::Percentage(20));

        self.render_header(frame, header_area);
        self.render_input(frame, input_area);
        self.render_messages(frame, messages_area);
        self.render_popup(frame, popup_area);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Line::from(match self.mode {
            Mode::Normal => "Command mode".bold(),
            Mode::Input => "Input mode".bold(),
            Mode::Edit => "Edit mode".bold(),
            Mode::Popup => "Polynomial finder mode".bold(),
        });

        let help_message = Line::from_iter(match self.mode {
            Mode::Normal => vec![
                " q ".bold(),
                "exit".on_blue(),
                " c ".bold(),
                "clear points".on_light_blue(),
                " i ".bold(),
                "input points".on_light_blue(),
                " e ".bold(),
                "edit points".on_light_blue(),
                " p ".bold(),
                "find polynomial".on_light_blue(),
            ],
            Mode::Input => vec![
                " Esc ".bold(),
                "stop editing".on_light_blue(),
                " Enter ".bold(),
                "record point".on_light_blue(),
            ],
            Mode::Popup => vec![
                " Esc ".bold(),
                "close popup".on_light_blue(),
            ],
            Mode::Edit => vec![
                " Esc ".bold(),
                "stop editing".on_light_blue(),
                " Up/Down ".bold(),
                "scroll up/down".on_light_blue(),
                " Del ".bold(),
                "delete point".on_light_blue(),
            ],
        });

        let header = Text::from(vec![title, help_message]);
        frame.render_widget(header, area);
    }

    fn render_input(&self, frame: &mut Frame, area: Rect) {
        // keep 2 for borders and 1 for cursor
        let width = area.width.max(3) - 3;
        let scroll = self.input.visual_scroll(width as usize);
        let style = match self.mode {
            Mode::Input => Color::Yellow.into(),
            _ => Style::default(),
        };
        let input = Paragraph::new(self.input.value())
            .style(style)
            .scroll((0, scroll as u16))
            .block(Block::bordered().title("Input"));
        frame.render_widget(input, area);

        if self.mode == Mode::Input{
            // Ratatui hides the cursor unless it's explicitly set. Position the cursor past the
            // end of the input text and one line down from the border to the input line
            let x = self.input.visual_cursor().max(scroll) - scroll + 1;
            frame.set_cursor_position((area.x + x as u16, area.y + 1))
        }
    }

    fn render_messages(&mut self, frame: &mut Frame, area: Rect) {
        let border_style = 
            if self.mode == Mode::Edit { 
                Color::Yellow.into() 
            } else { 
                Style::default() 
            };
        
        let message_block = Block::bordered()
            .title("Points")
            .border_style(border_style);

        let messages = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, message)| format!("{}: {}", i, message));

        let message_widget = List::new(messages)
            .block(message_block)
            .highlight_style(Style::new().bg(Color::Green));

        frame.render_stateful_widget(message_widget, area, &mut self.message_widget_state);
    }

    fn render_popup(&mut self, frame: &mut Frame, area: Rect) {
        if self.mode != Mode::Popup {
            return;
        }

        // Clear the area
        frame.render_widget(Clear, area);
        let popup_block = Block::bordered()
            .border_style(Color::Green)
            .title("Equation")
            .padding(Padding::proportional(1));
        let popup_message = Paragraph::new(self.popup_message.clone())
            .centered()
            .block(popup_block);
        frame.render_widget(popup_message, area);
    }

}
