extern crate peroxide;

use std::io;

use ratatui::{
    DefaultTerminal, 
    Frame, 
    crossterm::event::{self, Event, KeyCode}, 
    layout::{Constraint, Layout, Rect}, 
    style::{Color, Style, Stylize}, 
    text::{Line, Text}, 
    widgets::{Block, Clear, List, ListState, Padding, Paragraph}
};
use tui_input::backend::crossterm::EventHandler;
use tui_input::Input;

mod calculator;
mod structures;

fn main() -> io::Result<()> {
    ratatui::run(|t| App::default().run(t))
}

#[derive(Debug, Default)]
struct PopupState {
    /// Text to show on popup box
    message: String,
    /// Mode before the popup
    pre_popup_mode: Mode,
    /// Title of the popup
    title: String,
    /// Color of popup border
    color: Style,
}

/// App holds the state of the application
#[derive(Debug, Default)]
struct App {
    /// Current value of the input box
    input: Input,
    /// Current input mode
    mode: Mode,
    /// History of recorded messages
    messages: Vec<[f64;2]>,
    /// State of the List widget containing messages
    message_widget_state: ListState,
    /// State of the popup
    popup_state: PopupState
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Mode {
    #[default]
    Normal,
    Input,
    Edit,
    Select,
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
                        KeyCode::Char('c') => self.find_circle(),
                        KeyCode::Char('a') => self.find_area(),
                        KeyCode::Char('d') => self.messages.clear(),
                        KeyCode::Char('q') => return Ok(()), // exit
                        _ => {}
                    },
                    Mode::Input => match key.code {
                        KeyCode::Enter => self.insert_point(),
                        KeyCode::Esc => self.stop_input(),
                        _ => {
                            self.input.handle_event(&event);
                        }
                    },
                    Mode::Edit => match key.code {
                        KeyCode::Down => self.scroll_points(-1),
                        KeyCode::Up => self.scroll_points(1),
                        KeyCode::Delete => self.delete_selected_point(),
                        KeyCode::Char('e') => self.start_input(),
                        KeyCode::Enter => self.mode = Mode::Select,
                        KeyCode::Esc => self.stop_edit(),
                        _ => {}
                    },
                    Mode::Select => match key.code {
                        KeyCode::Up => self.move_point(true),
                        KeyCode::Down => self.move_point(false),
                        KeyCode::Enter => self.mode = Mode::Edit,
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

    fn start_popup(&mut self, title: String, message: String, border_color: Style) {
        self.popup_state.title = title;
        self.popup_state.message = message;
        self.popup_state.pre_popup_mode = self.mode;
        self.popup_state.color = border_color;
        self.mode = Mode::Popup
    }

    fn stop_popup(&mut self) {
        self.mode = self.popup_state.pre_popup_mode
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

    fn move_point(&mut self, up: bool) {
        let selected = self.message_widget_state.selected_mut();
        let next;
        if up {
            // move index towards 0
            if selected.unwrap_or(0) == 0 { return };
            next = selected.unwrap() - 1;
        } else {
            // move index towards self.messages.len()
            let lim = self.messages.len();
            if selected.unwrap_or(lim) + 1 >= lim { return };
            next = selected.unwrap() + 1;
        };
        self.messages.swap(selected.unwrap(), next);
         *selected = Some(next)
    }

    fn delete_selected_point(&mut self) {
        let Some(n) = self.message_widget_state.selected() else { return; };
        // Should not panic since selected item should be in range
        self.messages.remove(n);
    }

    fn insert_point(&mut self) {
        let input = self.input.value_and_reset();

        if self.mode == Mode::Input {
            if let Some(point) = calculator::float_parser::get_points(&input) {
                if let Some(n) = self.message_widget_state.selected() {
                    self.messages[n] = point;
                    // Exit 1-time input mode and go back to edit
                    // Just reset to Edit mode instead of re-init edit
                    self.mode = Mode::Edit
                } else {
                    self.messages.push(point);
                }
            } else {
                self.start_popup(
                    "Input Error".to_string(), 
                    format!("Cannot get point from input '{}'", input), 
                    Color::Red.into()
                );
            }
        }
    }

    fn show_calc_result(&mut self, title: String, result: Result<String, String>) {
        match result {
            Ok(equation) => {
                self.start_popup(
                    title,
                    equation, 
                    Color::Green.into()
                )
            },
            Err(err) => {
                self.start_popup(
                    "Calculation error".to_string(), 
                    format!("An error occured: {}", err), 
                    Color::Red.into()
                )
            },
        };
    }

    fn find_polynomial(&mut self) {
        self.show_calc_result(
            "Polynomial equation".to_string(),
            calculator::polynomial::solve_by_points(&self.messages)
        )
    }

    fn find_circle(&mut self) {
        self.show_calc_result(
            "Circle Equation".to_string(),
            calculator::circle::solve_by_points(&self.messages)
        )
    }

    fn find_area(&mut self) {
        self.show_calc_result(
            "Area".to_string(),
            calculator::area::solve_by_points(&self.messages)
        )
    }


    fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let [header_area, input_area, messages_area] = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .areas(area);

        let popup_area = area.centered(
            Constraint::Percentage(60), 
            Constraint::Length((area.height / 5).max(5)) // max(3, 20%)
        );

        self.render_header(frame, header_area);
        self.render_input(frame, input_area);
        self.render_messages(frame, messages_area);
        self.render_popup(frame, popup_area);
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Line::from(match self.mode {
            Mode::Normal => "Command mode".bold(),
            Mode::Input => "Input mode".bold(),
            Mode::Edit | Mode::Select => "Edit mode".bold(),
            Mode::Popup => "Polynomial finder mode".bold(),
        });

        let help_message = Line::from_iter(match self.mode {
            Mode::Normal => vec![
                " q ".bold(),
                "exit".on_blue(),
                " d ".bold(),
                "delete all points".on_light_blue(),
                " i ".bold(),
                "input points".on_light_blue(),
                " e ".bold(),
                "edit points".on_light_blue(),
                " p ".bold(),
                "find polynomial".on_light_blue(),
                " c ".bold(),
                "find circle".on_light_blue(),
                " a ".bold(),
                "find area".on_light_blue(),
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
                " Enter ".bold(),
                "select point".on_light_blue(),
                " e ".bold(),
                "edit point".on_light_blue(),
                " Del ".bold(),
                "delete point".on_light_blue(),
            ],
            Mode::Select => vec![
                " Esc ".bold(),
                "stop editing".on_light_blue(),
                " Up/Down ".bold(),
                "move point up/down".on_light_blue(),
                " Enter ".bold(),
                "deselect point".on_light_blue(),
            ]
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
            match self.mode { 
                Mode::Edit | Mode::Select => Color::Yellow.into(),
                _ => Style::default() 
            };
        
        let message_block = Block::bordered()
            .title("Points")
            .border_style(border_style);
        
        use calculator::float_parser::display_point;

        let messages = self
            .messages
            .iter()
            .map(|[x, y]| display_point([*x, *y]));

        let highlight = 
            match self.mode{
                Mode::Select => Style::new().bg(Color::Green),
                Mode::Edit => Style::new().bg(Color::Yellow),
                _ => Style::new()
        };

        let message_widget = List::new(messages)
            .block(message_block)
            .highlight_style(highlight);

        frame.render_stateful_widget(message_widget, area, &mut self.message_widget_state);
    }

    fn render_popup(&mut self, frame: &mut Frame, area: Rect) {
        if self.mode != Mode::Popup {
            return;
        }

        // Clear the area
        frame.render_widget(Clear, area);
        let popup_block = Block::bordered()
            .border_style(self.popup_state.color)
            .title(self.popup_state.title.clone())
            .padding(Padding::proportional(1));
        let popup_message = Paragraph::new(self.popup_state.message.clone())
            .centered()
            .block(popup_block);
        frame.render_widget(popup_message, area);
    }

}
