#[cfg(target_os = "none")]
use alloc::string::{String, ToString};

use crate::eadk;

pub fn estimate_text_width(text: &str, large_font: bool) -> u16 {
    let char_width = if large_font { 12 } else { 8 };
    let mut width = 0u16;
    let mut i = 0;
    while i < text.len() {
        if i + 1 < text.len() && text.chars().nth(i) == Some('^') && text.chars().nth(i+1).unwrap_or(' ').is_ascii_digit() {
            i += 1;
            let mut exp_str = String::new();
            while i < text.len() && text.chars().nth(i).unwrap_or(' ').is_ascii_digit() {
                let _ = exp_str.push(text.chars().nth(i).unwrap());
                i += 1;
            }
            width += (exp_str.len() as u16) * (char_width / 2);
        } else {
            width += char_width;
            i += 1;
        }
    }
    width
}

pub fn calculate_cursor_x(text: &str, cursor_pos: usize, exponent_mode: bool) -> (u16, u16) {
    let char_width = 12;
    let max_width = 300;
    let mut x_offset = 0u16;
    let mut y_offset = 0u16;
    let mut i = 0;
    
    while i < cursor_pos && i < text.len() {
        let current_char = text.chars().nth(i).unwrap_or(' ');
        
        if current_char == '^' && i + 1 < text.len() && text.chars().nth(i+1).unwrap_or(' ').is_ascii_digit() {
            i += 1;
            while i < text.len() && text.chars().nth(i).unwrap_or(' ').is_ascii_digit() {
                x_offset += 8;
                i += 1;
            }
        } else {
            x_offset += char_width;
            i += 1;
        }
        
        if x_offset > max_width {
            x_offset = 0;
            y_offset += 20;
        }
    }
    
    if exponent_mode && cursor_pos > 0 && text.chars().nth(cursor_pos - 1) == Some('^') {
        y_offset = 8;
    }
    if cursor_pos > 0 {
        let mut i = cursor_pos - 1;
        let mut found_caret = false;
        let mut caret_pos = 0;
        
        while i > 0 {
            if text.chars().nth(i) == Some('^') {
                found_caret = true;
                caret_pos = i;
                break;
            }
            i -= 1;
        }
        
        if found_caret {
            let mut j = caret_pos + 1;
            let mut in_exponent = true;
            while j < cursor_pos {
                let char = text.chars().nth(j).unwrap_or(' ');
                if !char.is_ascii_digit() && char != 'x' && char != 'n' && char != 't' {
                    in_exponent = false;
                    break;
                }
                j += 1;
            }
            
            if in_exponent {
                y_offset = 8;
            }
        }
    }
    
    (x_offset, y_offset)
}

pub fn draw_string_with_exponents(text: &str, point: eadk::Point, large_font: bool, text_color: eadk::Color, background_color: eadk::Color) {
    let char_width = if large_font { 12 } else { 8 };
    let mut x_offset = 0u16;
    let mut i = 0;
    
    while i < text.len() {
        let current_char = text.chars().nth(i).unwrap_or(' ');
        
        if current_char == '^' && i + 1 < text.len() && text.chars().nth(i+1).unwrap_or(' ').is_ascii_digit() {
            i += 1;
            let mut exp_text = String::new();
            while i < text.len() && text.chars().nth(i).unwrap_or(' ').is_ascii_digit() {
                exp_text.push(text.chars().nth(i).unwrap_or(' '));
                i += 1;
            }
            
            if !exp_text.is_empty() {
                let exp_y_offset = if large_font { 8 } else { 6 };
                let exp_point = eadk::Point { x: point.x + x_offset, y: point.y - exp_y_offset };
                eadk::display::draw_string(&exp_text, exp_point, false, text_color, background_color);
                x_offset += (exp_text.len() as u16) * 8;
            }
        } else {
            let char_point = eadk::Point { x: point.x + x_offset, y: point.y };
            let char_str = current_char.to_string();
            eadk::display::draw_string(&char_str, char_point, large_font, text_color, background_color);
            x_offset += char_width;
            i += 1;
        }
    }
}

pub fn draw_interface(
    title: &str,
    calculation_history: &[(String, String)],
    input_buffer: &str,
    cursor_position: usize,
    cursor_blink_timer: u32,
    exponent_mode: bool
) {
    eadk::display::push_rect_uniform(eadk::SCREEN_RECT, eadk::Color::from_888(255, 255, 255));
    
    let title_width = estimate_text_width(title, true);
    let title_x = (320u16.saturating_sub(title_width)) / 2;
    let title_point = eadk::Point { x: title_x + 10, y: 10 };
    eadk::display::draw_string(title, title_point, true, eadk::COLOR_BLACK, eadk::Color::from_888(255, 255, 255));
    
    let mut y_offset = 40;
    let max_history_items = 8;
    let start_index = if calculation_history.len() > max_history_items {
        calculation_history.len() - max_history_items
    } else {
        0
    };
    
    for i in start_index..calculation_history.len() {
        let (input, result) = &calculation_history[i];
        
        let input_point = eadk::Point { x: 10, y: y_offset };
        eadk::display::draw_string(input, input_point, false, eadk::COLOR_BLACK, eadk::Color::from_888(255, 255, 255));
        
        let result_width = estimate_text_width(result, false);
        let result_x = 320u16.saturating_sub(result_width).saturating_sub(10);
        let result_point = eadk::Point { x: result_x, y: y_offset };
        draw_string_with_exponents(result, result_point, false, eadk::COLOR_BLACK, eadk::Color::from_888(255, 255, 255));
        
        y_offset += 20;
    }
    
    let separator_y = 200;
    for x in 0..320 {
        let point = eadk::Point { x: x as u16, y: separator_y };
        eadk::display::push_rect_uniform(eadk::Rect { x: point.x, y: point.y, width: 1, height: 1 }, eadk::Color::from_888(200, 200, 200));
    }
    
    let input_point = eadk::Point { x: 10, y: 210 };
    
    draw_string_with_exponents(input_buffer, input_point, true, eadk::COLOR_BLACK, eadk::Color::from_888(255, 255, 255));
    
    let cursor_visible = (cursor_blink_timer / 30) % 2 == 0;
    if cursor_visible {
        let (cursor_x, cursor_y_offset) = calculate_cursor_x(input_buffer, cursor_position, exponent_mode);
        let cursor_point = eadk::Point { x: input_point.x + cursor_x, y: input_point.y + cursor_y_offset };
        
        let mut in_exponent_zone = false;
        if cursor_position > 0 {
            let mut i = cursor_position - 1;
            let mut found_caret = false;
            let mut caret_pos = 0;
            
            while i > 0 {
                if input_buffer.chars().nth(i) == Some('^') {
                    found_caret = true;
                    caret_pos = i;
                    break;
                }
                i -= 1;
            }
            
            if found_caret {
                let mut j = caret_pos + 1;
                let mut in_exponent = true;
                while j < cursor_position {
                    let char = input_buffer.chars().nth(j).unwrap_or(' ');
                    if !char.is_ascii_digit() && char != 'x' && char != 'n' && char != 't' {
                        in_exponent = false;
                        break;
                    }
                    j += 1;
                }
                
                in_exponent_zone = in_exponent;
            }
        }
        
        let cursor_font_size = if in_exponent_zone { false } else { true };
        eadk::display::draw_string("|", cursor_point, cursor_font_size, eadk::COLOR_BLACK, eadk::Color::from_888(255, 255, 255));
    }
}
