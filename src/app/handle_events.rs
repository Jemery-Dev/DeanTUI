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
                                let mut selected_value:u32 = 1; // To switch between inputs
                                match key.code {
                                    KeyCode::Char(c) => {
                                        match selected_value {
                                            1 =>{
                                                tab.name.push(c);
                                                self.tab = Some(tab);
                                            }
                                            2 =>{
                                                self.numberstr.push(c);
                                            }
                                            _ => {}
                                        }
                                    }
                                    KeyCode::Backspace => {
                                        match selected_value {
                                            1 =>{
                                                tab.name.pop();
                                                self.tab = Some(tab);
                                            }
                                            2 =>{
                                                self.numberstr.pop();
                                            }
                                            _ => {}
                                        }
                                    }
                                    KeyCode::Enter => {
                                        if !tab.name.is_empty() {
                                            selected_value = selected_value +1;
                                            if(selected_value == 3){
                                                self.state = AppState::EditingStrings;
                                            }
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