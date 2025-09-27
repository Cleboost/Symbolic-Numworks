#[cfg(target_os = "none")]
use alloc::string::{String, ToString};
#[cfg(target_os = "none")]
use alloc::vec::Vec;

use crate::eadk;

pub struct InputHandler {
    pub input_buffer: String,
    pub calculation_history: Vec<(String, String)>,
    pub last_key_states: [bool; 53],
    pub cursor_position: usize,
    pub cursor_blink_timer: u32,
    pub xnt_cycle: usize,
    pub last_cursor_position: usize,
    pub xnt_inserted: bool,
    pub exponent_mode: bool,
    pub exponent_start_position: usize,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            input_buffer: String::new(),
            calculation_history: Vec::new(),
            last_key_states: [false; 53],
            cursor_position: 0,
            cursor_blink_timer: 0,
            xnt_cycle: 0,
            last_cursor_position: 0,
            xnt_inserted: false,
            exponent_mode: false,
            exponent_start_position: 0,
        }
    }
    
    pub fn handle_exe_key(&mut self) -> bool {
        let keyboard = eadk::input::KeyboardState::scan();
        
        if keyboard.key_down(eadk::input::Key::Exe) && !self.last_key_states[eadk::input::Key::Exe as usize] {
            if !self.input_buffer.is_empty() {
                let result = crate::parsebeta::simplify(&self.input_buffer).unwrap_or_else(|_| {
                    let mut s = heapless::String::<128>::new();
                    let _ = s.push_str("Error");
                    s
                });
                self.calculation_history.push((self.input_buffer.clone(), result.to_string()));
                self.input_buffer.clear();
                self.cursor_position = 0;
            }
            self.last_key_states[eadk::input::Key::Exe as usize] = true;
            return true;
        }
        self.last_key_states[eadk::input::Key::Exe as usize] = keyboard.key_down(eadk::input::Key::Exe);
        false
    }
    
    pub fn handle_backspace(&mut self) {
        let keyboard = eadk::input::KeyboardState::scan();
        
        if keyboard.key_down(eadk::input::Key::Backspace) && !self.last_key_states[eadk::input::Key::Backspace as usize] {
            if self.cursor_position > 0 {
                self.input_buffer.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
            }
        }
        self.last_key_states[eadk::input::Key::Backspace as usize] = keyboard.key_down(eadk::input::Key::Backspace);
    }
    
    pub fn handle_arrow_keys(&mut self) {
        let keyboard = eadk::input::KeyboardState::scan();
        
        if keyboard.key_down(eadk::input::Key::Left) && !self.last_key_states[eadk::input::Key::Left as usize] {
            if self.cursor_position > 0 {
                self.cursor_position -= 1;
            }
        }
        self.last_key_states[eadk::input::Key::Left as usize] = keyboard.key_down(eadk::input::Key::Left);
        
        if keyboard.key_down(eadk::input::Key::Right) && !self.last_key_states[eadk::input::Key::Right as usize] {
            if self.cursor_position < self.input_buffer.len() {
                self.cursor_position += 1;
                
                if self.exponent_mode {
                    self.exponent_mode = false;
                }
                
                if self.cursor_position > 0 {
                    let mut i = self.cursor_position - 1;
                    let mut found_caret = false;
                    let mut caret_pos = 0;
                    
                    while i > 0 {
                        if self.input_buffer.chars().nth(i) == Some('^') {
                            found_caret = true;
                            caret_pos = i;
                            break;
                        }
                        i -= 1;
                    }
                    
                    if found_caret {
                        let mut j = caret_pos + 1;
                        let mut was_in_exponent = true;
                        
                        while j < self.cursor_position - 1 {
                            if !self.input_buffer.chars().nth(j).unwrap_or(' ').is_ascii_digit() {
                                was_in_exponent = false;
                                break;
                            }
                            j += 1;
                        }
                        
                        if was_in_exponent {
                            if self.cursor_position < self.input_buffer.len() {
                                let current_char = self.input_buffer.chars().nth(self.cursor_position).unwrap_or(' ');
                                if current_char == '+' || current_char == '-' || current_char == '*' || 
                                   current_char == '/' || current_char == '^' || current_char == ')' {
                                    self.exponent_mode = false;
                                }
                            } else {
                                if was_in_exponent {
                                    self.input_buffer.insert(self.cursor_position, ')');
                                    self.cursor_position += 1;
                                }
                                self.exponent_mode = false;
                            }
                        }
                    }
                }
            }
        }
        self.last_key_states[eadk::input::Key::Right as usize] = keyboard.key_down(eadk::input::Key::Right);
        
        if self.cursor_position != self.last_cursor_position {
            self.xnt_cycle = 0;
            self.xnt_inserted = false;
            self.last_cursor_position = self.cursor_position;
            
            if self.exponent_mode {
                self.exponent_mode = false;
            }
        }
    }
    
    pub fn handle_xnt_key(&mut self) {
        let keyboard = eadk::input::KeyboardState::scan();
        
        if keyboard.key_down(eadk::input::Key::Xnt) && !self.last_key_states[eadk::input::Key::Xnt as usize] {
            let char_to_insert = match self.xnt_cycle {
                0 => 'x',
                1 => 'n',
                2 => 't',
                _ => 'x',
            };
            
            if self.xnt_inserted && self.cursor_position > 0 && self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.remove(self.cursor_position - 1);
                self.cursor_position -= 1;
            }
            
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert(self.cursor_position, char_to_insert);
                self.cursor_position += 1;
                self.xnt_cycle = (self.xnt_cycle + 1) % 3;
                self.xnt_inserted = true;
                self.last_cursor_position = self.cursor_position;
            }
        }
        self.last_key_states[eadk::input::Key::Xnt as usize] = keyboard.key_down(eadk::input::Key::Xnt);
    }
    
    pub fn handle_operator_keys(&mut self) {
        let keyboard = eadk::input::KeyboardState::scan();
        
        if keyboard.key_down(eadk::input::Key::Minus) && !self.last_key_states[eadk::input::Key::Minus as usize] {
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert(self.cursor_position, '-');
                self.cursor_position += 1;
                self.xnt_cycle = 0;
                self.xnt_inserted = false;
            }
        }
        self.last_key_states[eadk::input::Key::Minus as usize] = keyboard.key_down(eadk::input::Key::Minus);
        
        if keyboard.key_down(eadk::input::Key::Plus) && !self.last_key_states[eadk::input::Key::Plus as usize] {
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert(self.cursor_position, '+');
                self.cursor_position += 1;
                self.xnt_cycle = 0;
                self.xnt_inserted = false;
            }
        }
        self.last_key_states[eadk::input::Key::Plus as usize] = keyboard.key_down(eadk::input::Key::Plus);
        
        if keyboard.key_down(eadk::input::Key::Multiplication) && !self.last_key_states[eadk::input::Key::Multiplication as usize] {
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert(self.cursor_position, '*');
                self.cursor_position += 1;
                self.xnt_cycle = 0;
                self.xnt_inserted = false;
            }
        }
        self.last_key_states[eadk::input::Key::Multiplication as usize] = keyboard.key_down(eadk::input::Key::Multiplication);
        
        if keyboard.key_down(eadk::input::Key::LeftParenthesis) && !self.last_key_states[eadk::input::Key::LeftParenthesis as usize] {
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert(self.cursor_position, '(');
                self.cursor_position += 1;
                self.xnt_cycle = 0;
                self.xnt_inserted = false;
            }
        }
        self.last_key_states[eadk::input::Key::LeftParenthesis as usize] = keyboard.key_down(eadk::input::Key::LeftParenthesis);
        
        if keyboard.key_down(eadk::input::Key::RightParenthesis) && !self.last_key_states[eadk::input::Key::RightParenthesis as usize] {
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert(self.cursor_position, ')');
                self.cursor_position += 1;
                self.xnt_cycle = 0;
                self.xnt_inserted = false;
            }
        }
        self.last_key_states[eadk::input::Key::RightParenthesis as usize] = keyboard.key_down(eadk::input::Key::RightParenthesis);
        
        if keyboard.key_down(eadk::input::Key::Square) && !self.last_key_states[eadk::input::Key::Square as usize] {
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert_str(self.cursor_position, "^2");
                self.cursor_position += 2;
                self.xnt_cycle = 0;
                self.xnt_inserted = false;
            }
        }
        self.last_key_states[eadk::input::Key::Square as usize] = keyboard.key_down(eadk::input::Key::Square);
        
        if keyboard.key_down(eadk::input::Key::Power) && !self.last_key_states[eadk::input::Key::Power as usize] {
            if self.cursor_position <= self.input_buffer.len() {
                self.input_buffer.insert(self.cursor_position, '^');
                self.cursor_position += 1;
                self.exponent_mode = true;
                self.exponent_start_position = self.cursor_position;
                self.xnt_cycle = 0;
                self.xnt_inserted = false;
            }
        }
        self.last_key_states[eadk::input::Key::Power as usize] = keyboard.key_down(eadk::input::Key::Power);
    }
    
    pub fn handle_number_keys(&mut self) {
        let keyboard = eadk::input::KeyboardState::scan();
        
        let number_keys = [
            (eadk::input::Key::Zero, '0'),
            (eadk::input::Key::One, '1'),
            (eadk::input::Key::Two, '2'),
            (eadk::input::Key::Three, '3'),
            (eadk::input::Key::Four, '4'),
            (eadk::input::Key::Five, '5'),
            (eadk::input::Key::Six, '6'),
            (eadk::input::Key::Seven, '7'),
            (eadk::input::Key::Eight, '8'),
            (eadk::input::Key::Nine, '9'),
        ];
        
        for (key, digit) in number_keys.iter() {
            if keyboard.key_down(*key) && !self.last_key_states[*key as usize] {
                if self.cursor_position <= self.input_buffer.len() {
                    self.input_buffer.insert(self.cursor_position, *digit);
                    self.cursor_position += 1;
                    self.xnt_cycle = 0;
                    self.xnt_inserted = false;
                    
                    if self.exponent_mode {
                        self.exponent_mode = false;
                    }
                }
            }
            self.last_key_states[*key as usize] = keyboard.key_down(*key);
        }
    }
    
    pub fn update_timer(&mut self) {
        self.cursor_blink_timer += 1;
    }
}
