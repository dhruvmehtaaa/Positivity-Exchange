use std::{sync::Arc, time::Duration};

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use tokio::sync::{broadcast, RwLock};

use self::termination::{Interrupted, Terminator};

pub(crate) mod termination;

pub(crate) enum InputMode {
    Normal,
    Editing,
}

pub(crate) struct App {
    terminator: Terminator,
    pub(crate) input: String,
    pub(crate) cursor_position: usize,
    pub(crate) input_mode: InputMode,
    pub(crate) messages: Vec<String>,
    pub(crate) timer: usize,
}

impl App {
    pub fn new(terminator: Terminator) -> App {
        App {
            terminator,
            input: String::new(),
            input_mode: InputMode::Normal,
            messages: Vec::new(),
            cursor_position: 0,
            timer: 0,
        }
    }

    pub(crate) fn handle_key_event(&mut self, key: KeyEvent) {
        match self.input_mode {
            InputMode::Normal => match key.code {
                KeyCode::Char('e') => {
                    self.input_mode = InputMode::Editing;
                }
                KeyCode::Char('q') => {
                    let _ = self.terminator.terminate(Interrupted::UserInt);
                }
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let _ = self.terminator.terminate(Interrupted::UserInt);
                }
                _ => {}
            },
            InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Enter => self.submit_message(),
                KeyCode::Char(to_insert) => {
                    self.enter_char(to_insert);
                }
                KeyCode::Backspace => {
                    self.delete_char();
                }
                KeyCode::Left => {
                    self.move_cursor_left();
                }
                KeyCode::Right => {
                    self.move_cursor_right();
                }
                KeyCode::Esc => {
                    self.input_mode = InputMode::Normal;
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn increment_timer(&mut self) {
        self.timer += 1;
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn submit_message(&mut self) {
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
    }
}

pub(crate) async fn main_loop(
    mut interrupt_rx: broadcast::Receiver<Interrupted>,
    app: Arc<RwLock<App>>,
) -> anyhow::Result<Interrupted> {
    let mut ticker = tokio::time::interval(Duration::from_secs(1));

    let result = loop {
        tokio::select! {
            // Tick to terminate the select every N milliseconds
            _ = ticker.tick() => {
                let mut app = app.write().await;

                app.increment_timer();
            },
            // Catch and handle interrupt signal to gracefully shutdown
            Ok(interrupted) = interrupt_rx.recv() => {
                break interrupted;
            }
        }
    };

    Ok(result)
}