#![cfg_attr(target_os = "none", no_std)]
#![no_main]

#[allow(unused_imports)]
#[cfg(target_os = "none")]
use cortex_m;

#[cfg(target_os = "none")]
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
#[cfg(target_os = "none")]
static HEAP: Heap = Heap::empty();

#[cfg(target_os = "none")]
extern crate alloc;

pub mod eadk;

// Function to estimate text width
fn estimate_text_width(text: &str, large_font: bool) -> u16 {
    let char_width = if large_font { 12 } else { 8 }; // Estimation based on Numworks fonts
    text.len() as u16 * char_width
}

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_name")]
pub static EADK_APP_NAME: [u8; 13] = *b"Num Template\0"; //YOUR APP NAME HERE

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_api_level")]
pub static EADK_APP_API_LEVEL: u32 = 0; 

#[used]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".rodata.eadk_app_icon")]
//Dont forget to change size (1520) to the size of your icon (in bytes)
pub static EADK_APP_ICON: [u8; 1520] = *include_bytes!("../target/icon.nwi"); //YOUR APP ICON HERE

#[unsafe(no_mangle)]
fn main() -> isize {
    // Init the heap
    #[cfg(target_os = "none")]
    {
        let heap_size: usize = eadk::heap_size();
        unsafe { HEAP.init(eadk::HEAP_START as usize, heap_size) }
    }

    // Wait for OK key release to avoid instant click
    while eadk::input::KeyboardState::scan().key_down(eadk::input::Key::Ok) {
        eadk::timing::msleep(50);
    }

    // Clear screen with white background
    eadk::display::push_rect_uniform(eadk::SCREEN_RECT, eadk::Color::from_888(255, 255, 255));
    
    // Draw "Hello World!" text (perfectly centered)
    let hello_text = "Hello World!";
    let hello_width = estimate_text_width(hello_text, true);
    let hello_x = (320u16.saturating_sub(hello_width)) / 2; // Perfect centering with overflow protection
    let text_point = eadk::Point { x: hello_x, y: 100 };
    eadk::display::draw_string(hello_text, text_point, true, eadk::COLOR_BLACK, eadk::Color::from_888(255, 255, 255));
    
    // Draw instructions (perfectly centered)
    let instructions = "Press EXE to exit";
    let inst_width = estimate_text_width(instructions, true);
    let inst_x = (320u16.saturating_sub(inst_width)) / 2; // Perfect centering with overflow protection
    let inst_point = eadk::Point { x: inst_x, y: 150 };
    eadk::display::draw_string(instructions, inst_point, true, eadk::COLOR_BLACK, eadk::Color::from_888(255, 255, 255));

    // Wait for EXE key to exit
    loop {
        if eadk::input::KeyboardState::scan().key_down(eadk::input::Key::Exe) {
            break;
        }
        eadk::timing::msleep(16); // ~60 FPS
    }

    0
}
