#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap;

#[cfg(target_os = "none")]
#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();

mod eadk;
mod ui;
mod input;
mod parser;
mod parsebeta;

#[cfg(target_os = "none")]
mod critical_section_impl {
    use core::sync::atomic::AtomicBool;
    
    static CRITICAL_SECTION: AtomicBool = AtomicBool::new(false);
    
    #[unsafe(no_mangle)]
    pub extern "C" fn _critical_section_1_0_acquire() -> u8 {
        0
    }
    
    #[unsafe(no_mangle)]
    pub extern "C" fn _critical_section_1_0_release(_state: u8) {
    }
}


use input::InputHandler;

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 17] = *b"Symbolic Compute\0";

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
pub static EADK_APP_ICON: [u8; 4] = *b"\x00\x00\x00\x00";

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_API_LEVEL: u32 = 0;

#[unsafe(no_mangle)]
pub extern "C" fn main() {
    #[cfg(target_os = "none")]
    unsafe {
        HEAP.init(0x20000000 as usize, 1024 * 1024);
    }
    
    let mut input_handler = InputHandler::new();
    
    let keyboard = eadk::input::KeyboardState::scan();
    while keyboard.key_down(eadk::input::Key::Exe) {
        eadk::timing::msleep(10);
    }
    
    loop {
        let title = "SYMBOLIC COMPUTE";
        
        ui::draw_interface(
            title,
            &input_handler.calculation_history,
            &input_handler.input_buffer,
            input_handler.cursor_position,
            input_handler.cursor_blink_timer,
            input_handler.exponent_mode
        );
        
        let keyboard = eadk::input::KeyboardState::scan();
        
        if keyboard.key_down(eadk::input::Key::Back) {
            break;
        }
        
        if input_handler.handle_exe_key() {
            continue;
        }
        
        input_handler.handle_backspace();
        input_handler.handle_arrow_keys();
        input_handler.handle_xnt_key();
        input_handler.handle_operator_keys();
        input_handler.handle_number_keys();
        input_handler.update_timer();
        
        eadk::timing::msleep(16);
    }
}