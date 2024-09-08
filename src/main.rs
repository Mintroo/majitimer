mod settings;
mod timer;
mod tui;

use std::path;

use settings::TimerMode;
use timer::MyToType;

use std::io;

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{
        block::{Position, Title},
        Block, Paragraph, Widget,
    },
    Frame,
};

pub struct App {
    settings: settings::Settings,
    run: settings::RunData,
    exit: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            settings: settings::Settings::init().unwrap(),
            run: settings::RunData::new(),
            exit: false,
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
            self.run.state_process(&self.settings).unwrap();

            // let file = std::fs::File::open("assets/finish.mp3").unwrap();
            // let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            // let audacity = stream_handle
            //     .play_once(std::io::BufReader::new(file))
            //     .unwrap();
            // audacity.set_volume(0.05);
            // std::thread::sleep(std::time::Duration::from_secs(10));
        }
        Ok(())
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                // it's important to check that the event is a key press event as
                // crossterm also emits key release and repeat events on Windows.
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event)
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => self.exit(),
            KeyCode::Char(' ') => match self.run.mode() {
                TimerMode::Init => self.run.init(&self.settings),
                _ => self.run.pause_or_resume(),
            },
            KeyCode::Char('i') | KeyCode::Char('I') => {
                if let Ok(path) = settings::path_picker(false) {
                    self.settings = settings::Settings::import(&path).unwrap();
                    self.settings
                        .export(
                            &dirs::config_dir()
                                .unwrap()
                                .join("majitimer")
                                .join("config.json"),
                        )
                        .unwrap();

                    self.run.update(&self.settings);
                }
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                if let Ok(path) = settings::path_picker(true) {
                    self.settings.export(&path).unwrap();
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => self.run.init(&self.settings),
            KeyCode::Char('m') | KeyCode::Char('M') => {
                if let TimerMode::Endurance | TimerMode::UrgedToReMajiTime = self.run.mode() {
                    self.run.mode_transition_start();
                }
            }
            KeyCode::Char('1') => {
                if let Ok(path) = settings::path_picker(false) {
                    self.settings.other.set_finish_sound(&path);
                    self.settings
                        .export(
                            &dirs::config_dir()
                                .unwrap()
                                .join("majitimer")
                                .join("config.json"),
                        )
                        .unwrap();
                }
            }
            KeyCode::Char('2') => {
                if let Ok(path) = settings::path_picker(false) {
                    self.settings.other.set_restart_sound(&path);
                    self.settings
                        .export(
                            &dirs::config_dir()
                                .unwrap()
                                .join("majitimer")
                                .join("config.json"),
                        )
                        .unwrap();
                }
            }
            KeyCode::Char('3') => {
                if let Ok(path) = settings::path_picker(false) {
                    self.settings.other.set_remind_sound(&path);
                    self.settings
                        .export(
                            &dirs::config_dir()
                                .unwrap()
                                .join("majitimer")
                                .join("config.json"),
                        )
                        .unwrap();
                }
            }
            // KeyCode::Char('4') => {
            //     let file = std::fs::File::open("assets/finish.mp3").unwrap();
            //     let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
            //     let audacity = stream_handle
            //         .play_once(std::io::BufReader::new(file))
            //         .unwrap();
            //     audacity.set_volume(0.05);
            // }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut run_text = self.run.render();
        let block = {
            let title = Title::from(" Maji Timer (本気タイマー) ".bold());
            let instructions = Title::from(Line::from(run_text.1));
            Block::bordered()
                .title(title.alignment(Alignment::Center))
                .title(
                    instructions
                        .alignment(Alignment::Center)
                        .position(Position::Bottom),
                )
                .border_set(border::THICK)
        };

        if self.run.paused() && self.run.mode() != &TimerMode::Init {
            run_text.0.push(Line::from("一時停止中".white().bold()))
        }

        let counter_text = Text::from(run_text.0);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

pub fn main() -> io::Result<()> {
    // let settings = settings::Settings::template();
    // let export_path = settings::path_picker().unwrap();
    // settings.export(&export_path).unwrap();
    let mut terminal = tui::init()?;
    let app_result = App::new().run(&mut terminal);
    tui::restore()?;
    app_result
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use ratatui::style::Style;
//
//     #[test]
//     fn render() {
//         let app = App::new();
//         let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));
//
//         app.render(buf.area, &mut buf);
//
//         let mut expected = Buffer::with_lines(vec![
//             "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
//             "┃                    Value: 0                    ┃",
//             "┃                                                ┃",
//             "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
//         ]);
//         let title_style = Style::new().bold();
//         let counter_style = Style::new().yellow();
//         let key_style = Style::new().blue().bold();
//         expected.set_style(Rect::new(14, 0, 22, 1), title_style);
//         expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
//         expected.set_style(Rect::new(13, 3, 6, 1), key_style);
//         expected.set_style(Rect::new(30, 3, 7, 1), key_style);
//         expected.set_style(Rect::new(43, 3, 4, 1), key_style);
//
//         // note ratatui also has an assert_buffer_eq! macro that can be used to
//         // compare buffers and display the differences in a more readable way
//         assert_eq!(buf, expected);
//     }
//
//     #[test]
//     fn handle_key_event() -> io::Result<()> {
//         let mut app = App::new();
//         app.handle_key_event(KeyCode::Right.into());
//         // assert_eq!(app.counter, 1);
//
//         app.handle_key_event(KeyCode::Left.into());
//         // assert_eq!(app.counter, 0);
//
//         let mut app = App::new();
//         app.handle_key_event(KeyCode::Char('q').into());
//         assert!(app.exit);
//
//         Ok(())
//     }
// }
