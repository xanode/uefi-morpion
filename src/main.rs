#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use uefi::proto::console::{
    gop::GraphicsOutput,
    text::{Key, ScanCode},
};
use uefi::Char16;
use uefi_graphics::UefiDisplay;

mod engine;


#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    // Initialize the engine
    let mut engine = engine::Engine::new();
    // Initialize cursor position
    let mut cursor = 0;
    // Select item that corresponds to the cursor position
    engine.select_item(cursor);

    let mut drawn = false;
    loop {
        let key = system_table.stdin().read_key().unwrap();
        let _ = match key {
            Some(k) => {
                match k {
                    Key::Special(ScanCode::ESCAPE) => break,
                    Key::Special(ScanCode::LEFT) => {
                        if (cursor > 0 && cursor < 3) || (cursor > 3 && cursor < 6) || (cursor > 6 && cursor < 9) {
                            cursor -= 1;
                            engine.select_item(cursor);
                            drawn = false;
                        }
                    },
                    Key::Special(ScanCode::RIGHT) => {
                        if (cursor < 2) || (cursor >= 3 && cursor < 5) || (cursor >= 6 && cursor < 8) {
                            cursor += 1;
                            engine.select_item(cursor);
                            drawn = false;
                        }
                    },
                    Key::Special(ScanCode::UP) => {
                        if cursor > 2 {
                            cursor -= 3;
                            engine.select_item(cursor);
                            drawn = false;
                        }
                    },
                    Key::Special(ScanCode::DOWN) => {
                        if (cursor + 3) < 9 {
                            cursor += 3;
                            engine.select_item(cursor);
                            drawn = false;
                        }
                    },
                    Key::Printable(p) => {
                        if p != Char16::try_from(' ').unwrap() {
                            continue;
                        }
                        let selected_item = engine.get_selected_item_symbol().unwrap();
                        match selected_item {
                            engine::ItemSymbol::Empty => {
                                engine.set_item_symbol(cursor, engine::ItemSymbol::X);
                                drawn = false;
                                // Set a variable to know that this is the computer's turn
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                };
            }
            _ => if !drawn {
                // Open graphics output protocol
                let gop_handle = system_table.boot_services().get_handle_for_protocol::<GraphicsOutput>().unwrap();
                let mut gop = system_table.boot_services().open_protocol_exclusive::<GraphicsOutput>(gop_handle).unwrap();

                // Get the framebuffer
                let mode  = gop.current_mode_info();
                let mut fb = gop.frame_buffer();

                // Get resolution
                let (width, height) = mode.resolution();

                // Create a display
                let display = &mut UefiDisplay::new(
                    fb.as_mut_ptr(),
                    mode.stride() as u32,
                    (width as u32, height as u32),
                    &fb
                );
                engine.draw(width as u32, height as u32, display);
                drawn = true;
            },
        };
    };
    Status::SUCCESS
}
