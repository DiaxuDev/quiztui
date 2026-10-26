use crossterm::event::KeyCode;
use misstep::{Result, ResultExt};
use rand::{RngExt, rngs::ThreadRng};

use crate::cards::{Flashcard, parse_cards};

pub enum State {
    Input,
    Review(bool),
    ForceType,
}

pub struct App {
    pub running: bool,
    pub input: String,
    pub state: State,
    cards: Vec<Flashcard>,
    current_card: usize,
    set: Vec<usize>,
    rng: ThreadRng,
}

impl App {
    pub fn new() -> Result<Self> {
        let cards = parse_cards().context("parsing cards")?;
        let mut rng = rand::rng();
        let current_card = rng.random_range(0..cards.len());
        let set = (0..cards.len()).collect();

        Ok(Self {
            running: true,
            input: String::new(),
            state: State::Input,
            cards,
            current_card,
            set,
            rng,
        })
    }

    pub fn current_card(&self) -> &Flashcard {
        &self.cards[self.current_card]
    }

    pub fn stats(&self) -> (usize, usize) {
        (self.cards.len() - self.set.len(), self.cards.len())
    }

    pub fn handle_keypress(&mut self, event: &crossterm::event::KeyEvent) {
        match event.code {
            KeyCode::Esc => self.quit(),
            KeyCode::Backspace if matches!(self.state, State::Input | State::ForceType) => {
                self.input.pop();
            }
            KeyCode::Enter => match self.state {
                State::Input | State::ForceType => self.check_input(),
                State::Review(success) => {
                    if success {
                        self.next_card(true);
                    } else {
                        self.input.clear();
                        self.state = State::ForceType;
                    }
                }
            },
            code => {
                if let Some(c) = code.as_char() {
                    match self.state {
                        State::Input | State::ForceType => self.input.push(c),
                        State::Review(false) => {
                            self.input.clear();
                            self.state = State::ForceType;
                            self.input.push(c);
                        }
                        State::Review(_) => {}
                    }
                }
            }
        }
    }

    fn check_input(&mut self) {
        match self.state {
            State::Input => {
                self.state = State::Review(self.current_card().check(&self.input));
            }
            State::ForceType => {
                if self.current_card().check(&self.input) {
                    self.next_card(false);
                }
            }
            State::Review(_) => {}
        }
    }

    fn next_card(&mut self, success: bool) {
        if success {
            let Ok(idx) = self.set.binary_search(&self.current_card) else {
                panic!("Binary search on learning set returned no result");
            };
            self.set.remove(idx);
        }

        if self.set.is_empty() {
            self.quit();
            return;
        }

        self.current_card = self.set[self.rng.random_range(0..self.set.len())];

        self.state = State::Input;
        self.input.clear();
    }

    fn quit(&mut self) {
        self.running = false;
    }
}
