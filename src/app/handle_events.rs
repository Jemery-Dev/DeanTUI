use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use crate::app::app::App;
use crate::app::app_states::AppState;

    impl <'a> App<'a> {
        pub async fn handle_events(&mut self) {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press {
                    match self.state {
                        AppState::Normal => {
                            match key.code {
                                KeyCode::Char('q') => self.should_quit = true,
                                KeyCode::Enter => self.state = AppState::NewProject,
                                _ => {}
                            }
                        }

                        AppState::NewProject => {
                            if let Some(mut tab) = self.tab.take() {
                                match key.code {
                                    KeyCode::Char(c) => {
                                        tab.name.push(c);
                                        self.tab = Some(tab);
                                    }
                                    KeyCode::Backspace => {
                                        tab.name.pop();
                                        self.tab = Some(tab);
                                    }
                                    KeyCode::Enter => {
                                        if !tab.name.is_empty() {

                                        }
                                    }
                                    KeyCode::Esc => self.state = AppState::Normal,
                                    _ => {}
                                }
                            }
                        }

                        AppState::EditingTab => {
                            match key.code {
                                KeyCode::Char('q') => self.should_quit = true,
                                _ => {}
                            }
                        }

                        _ => {}
                    }
                }
            }
        }
    }